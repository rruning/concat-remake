// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer and Concat contributors

//! The Audio page's Built-in shelf: sound effects made on this machine.
//!
//! Sixteen small effects - a whoosh, an explosion, a bass boom, a riser,
//! a click, a pop, a ding, a coin, a sweep down, a laser, a punch, a
//! sparkle, a record scratch, a camera shutter, a rewind and an error
//! buzz - synthesised here from noise, sines and envelopes, and written
//! as WAV into `<library>/audio/builtin/` the first time they are asked
//! for. No network and no download: the samples are made, not fetched,
//! and a file someone deleted is simply made again. Every sound is
//! original; none is a copy of anyone's game or library.
//!
//! Nothing here touches a project. A sound that goes onto the timeline is
//! copied into that project's own folder by the window, the same way a
//! sound of the user's own is - see `concat_host::sounds`.

use std::path::{Path, PathBuf};

use crate::AppDirs;

/// Samples a second, mono, 16-bit: the file a card stands for.
const RATE: u32 = 44_100;

/// One built-in sound: what its card says, and which samples are made
/// for it.
pub struct Builtin {
    /// The part of the card's id after the shelf's `fx-` prefix.
    pub id: &'static str,
    /// What the card says.
    pub name: &'static str,
    /// A word the page's categories map on: Whoosh, Impact, Riser, UI.
    pub category: &'static str,
    /// Seconds, for the card.
    pub duration: f64,
}

/// Every built-in sound, in shelf order.
pub const BUILTIN: [Builtin; 16] = [
    Builtin {
        id: "whoosh",
        name: "Whoosh",
        category: "Whoosh",
        duration: 0.9,
    },
    Builtin {
        id: "explosion",
        name: "Explosion",
        category: "Impact",
        duration: 1.6,
    },
    Builtin {
        id: "boom",
        name: "Bass Boom",
        category: "Impact",
        duration: 1.2,
    },
    Builtin {
        id: "riser",
        name: "Riser",
        category: "Riser",
        duration: 2.0,
    },
    Builtin {
        id: "click",
        name: "Click",
        category: "UI",
        duration: 0.07,
    },
    Builtin {
        id: "pop",
        name: "Pop",
        category: "UI",
        duration: 0.22,
    },
    Builtin {
        id: "ding",
        name: "Ding",
        category: "UI",
        duration: 1.4,
    },
    Builtin {
        id: "coin",
        name: "Coin",
        category: "UI",
        duration: 0.4,
    },
    Builtin {
        id: "sweep-down",
        name: "Sweep Down",
        category: "Whoosh",
        duration: 0.7,
    },
    Builtin {
        id: "laser",
        name: "Laser",
        category: "Whoosh",
        duration: 0.5,
    },
    Builtin {
        id: "punch",
        name: "Punch",
        category: "Impact",
        duration: 0.35,
    },
    Builtin {
        id: "sparkle",
        name: "Sparkle",
        category: "UI",
        duration: 0.9,
    },
    Builtin {
        id: "scratch",
        name: "Scratch",
        category: "Riser",
        duration: 0.8,
    },
    Builtin {
        id: "shutter",
        name: "Shutter",
        category: "UI",
        duration: 0.3,
    },
    Builtin {
        id: "rewind",
        name: "Rewind",
        category: "Riser",
        duration: 1.0,
    },
    Builtin {
        id: "error",
        name: "Error",
        category: "UI",
        duration: 0.6,
    },
];

/// The shelf's sound with this id, when there is one.
pub fn get(id: &str) -> Option<&'static Builtin> {
    BUILTIN.iter().find(|sound| sound.id == id)
}

/// Where the shelf's files are written: under the library, beside the
/// user's own sounds but never among them.
pub fn dir_in(dirs: &AppDirs) -> PathBuf {
    dirs.library().join("audio").join("builtin")
}

/// Where one sound's file lives.
pub fn file_in(dirs: &AppDirs, id: &str) -> PathBuf {
    dir_in(dirs).join(format!("{id}.wav"))
}

/// Writes every sound that is not on disk yet, so the shelf can show its
/// cards with their envelopes before anything is played. A failure for
/// one sound is left for that sound's first use to report.
pub fn ensure(dirs: &AppDirs) {
    for sound in &BUILTIN {
        let _ = ensure_one(dirs, sound.id);
    }
}

/// The sound's file, made when it is not there yet. A file already there
/// is believed as it is - the samples are the same every time - so this
/// costs one `stat` after the first run. Blocks: a few hundred kilobytes
/// at most.
pub fn ensure_one(dirs: &AppDirs, id: &str) -> Result<PathBuf, String> {
    let _sound = get(id).ok_or_else(|| format!("no built-in sound named {id}"))?;
    let path = file_in(dirs, id);
    if !path.is_file() {
        let dir = dir_in(dirs);
        std::fs::create_dir_all(&dir)
            .map_err(|error| format!("could not create {}: {error}", dir.display()))?;
        write_wav(&path, &render(id))
            .map_err(|error| format!("could not write {}: {error}", path.display()))?;
    }
    Ok(path)
}

/// The envelope a card draws: [`super::sounds::BARS`] peaks across the
/// sound's own samples - the same shape an imported file's has, read
/// without decoding anything.
/// A card as the shelf's waves name it, or empty when it is unknown.
pub fn bars(id: &str) -> Vec<f32> {
    let samples = render(id);
    let chunk = (samples.len() / super::sounds::BARS).max(1);
    (0..super::sounds::BARS)
        .map(|bar| {
            let from = bar * chunk;
            let to = if bar + 1 == super::sounds::BARS {
                samples.len()
            } else {
                (from + chunk).min(samples.len())
            };
            let peak = samples[from..to]
                .iter()
                .fold(0.0f32, |peak, sample| peak.max(sample.abs()));
            (peak.clamp(0.0, 1.0) * 100.0).round() / 100.0
        })
        .collect()
}

/// The samples themselves: the sound's own length, at [`RATE`].
fn render(id: &str) -> Vec<f32> {
    match id {
        "whoosh" => whoosh(),
        "explosion" => explosion(),
        "boom" => boom(),
        "riser" => riser(),
        "click" => click(),
        "pop" => pop(),
        "ding" => ding(),
        "coin" => coin(),
        "sweep-down" => sweep_down(),
        "laser" => laser(),
        "punch" => punch(),
        "sparkle" => sparkle(),
        "scratch" => scratch(),
        "shutter" => shutter(),
        "rewind" => rewind(),
        "error" => error(),
        _ => Vec::new(),
    }
}

/// White noise from a xorshift, `-1..=1`: the same state is always the
/// same noise, so a sound is the same samples every time it is made.
fn noise(state: &mut u32) -> f32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    (*state as f32 / u32::MAX as f32) * 2.0 - 1.0
}

/// One sample of a one-pole lowpass at `cutoff` hertz.
fn lowpass(sample: f32, state: &mut f32, cutoff: f32) -> f32 {
    let a = 1.0 - (-2.0 * std::f32::consts::PI * cutoff / RATE as f32).exp();
    *state += a * (sample - *state);
    *state
}

/// Scales to `peak`, keeping the shape.
fn normalise(samples: &mut [f32], peak: f32) {
    let loudest = samples
        .iter()
        .fold(0.0f32, |loudest, sample| loudest.max(sample.abs()));
    if loudest > 0.0 {
        let gain = peak / loudest;
        for sample in samples {
            *sample *= gain;
        }
    }
}

/// Something passing the listener: noise through a band whose centre
/// sweeps up and back down, under an envelope slow to rise and quicker
/// to fall away.
fn whoosh() -> Vec<f32> {
    let n = (0.9 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0x1234_5678;
    let mut low = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / n as f32;
        // 0 at the ends, 1 in the middle: the band is brightest as the
        // thing goes by.
        let past = 1.0 - (2.0 * t - 1.0).abs();
        let cutoff = 380.0 + 5000.0 * past * past;
        let heard = lowpass(noise(&mut state), &mut low, cutoff);
        let high = heard - low * 0.35;
        let env = (t * std::f32::consts::PI).sin().powi(3);
        *sample = high * env * 1.6;
    }
    normalise(&mut out, 0.9);
    out
}

/// A burst of noise falling from bright to a rumble, with the low end
/// dropping under it - the shape of a bang rolling away.
fn explosion() -> Vec<f32> {
    let n = (1.6 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0xdead_beef;
    let mut low = 0.0;
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        // The crack up front: bright, and gone in a moment.
        let crack = if t < 0.03 {
            noise(&mut state) * (1.0 - t / 0.03)
        } else {
            0.0
        };
        // The body: a wide band closing down over the burst.
        let cutoff = 320.0 + 4800.0 * (-t * 7.0).exp();
        let body = lowpass(noise(&mut state), &mut low, cutoff) * (-t / 0.38).exp();
        // The rumble: a low tone falling as it dies.
        let hz = 38.0 + 34.0 * (-t * 3.0).exp();
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let rumble = phase.sin() * (-t / 0.5).exp() * 0.8;
        *sample = crack * 0.8 + body * 1.8 + rumble;
    }
    normalise(&mut out, 0.95);
    out
}

/// A single low thump: a tone falling fast from its note into the sub,
/// with a short breath of noise at the hit.
fn boom() -> Vec<f32> {
    let n = (1.2 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0xc0ffee;
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let hz = 30.0 + 110.0 * (-t * 6.0).exp();
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let env = (t / 0.008).min(1.0) * (-t / 0.42).exp();
        let hit = noise(&mut state) * (-t / 0.012).exp() * 0.25;
        *sample = phase.sin() * env + hit;
    }
    normalise(&mut out, 0.95);
    out
}

/// The climb before a cut: noise whose band rises, and a tone rising
/// with it, both under an envelope that keeps climbing to the end - the
/// editor cuts where the drop would be.
fn riser() -> Vec<f32> {
    let n = (2.0 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0xfeed_1234;
    let mut low = 0.0;
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / n as f32;
        let cutoff = 220.0 + 4200.0 * t * t;
        let hiss = noise(&mut state) - lowpass(noise(&mut state), &mut low, cutoff);
        let hz = 180.0 + 1260.0 * t * t;
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let tone = phase.sin() * 0.35;
        let env = (t * t * 0.85 + 0.05) * (t / 0.01).min(1.0);
        *sample = (hiss * 0.8 + tone) * env;
    }
    normalise(&mut out, 0.85);
    out
}

/// A short wooden blip - a menu tick, a block set down. A tone with one
/// harmonic over it, and a grain of noise at the strike.
fn click() -> Vec<f32> {
    let n = (0.07 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0x51c0_1234;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let body = (2.0 * std::f32::consts::PI * 830.0 * t).sin() * (-t / 0.010).exp();
        let overtone = (2.0 * std::f32::consts::PI * 1660.0 * t).sin() * (-t / 0.006).exp() * 0.35;
        let grain = noise(&mut state) * (-t / 0.0015).exp() * 0.5;
        *sample = body + overtone + grain;
    }
    normalise(&mut out, 0.85);
    out
}

/// A soft blip falling in pitch - a bubble, a message arriving.
fn pop() -> Vec<f32> {
    let n = (0.22 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let hz = 160.0 + 780.0 * (-t * 20.0).exp();
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        *sample = phase.sin() * (-t / 0.045).exp();
    }
    normalise(&mut out, 0.9);
    out
}

/// A small bell: one strike, its partials ringing out at their own
/// rates.
fn ding() -> Vec<f32> {
    let n = (1.4 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let strike = (t / 0.002).min(1.0);
        let f = 1318.5;
        let bell = (2.0 * std::f32::consts::PI * f * t).sin() * (-t / 0.55).exp()
            + (2.0 * std::f32::consts::PI * f * 2.76 * t).sin() * (-t / 0.35).exp() * 0.22
            + (2.0 * std::f32::consts::PI * f * 5.4 * t).sin() * (-t / 0.15).exp() * 0.08;
        *sample = bell * strike;
    }
    normalise(&mut out, 0.85);
    out
}

/// The two notes of a coin taken: a bright square-ish blip, then its
/// note a fourth up, ringing away.
fn coin() -> Vec<f32> {
    let n = (0.4 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut phase = 0.0;
    let split = 0.06;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let hz = if t < split { 987.8 } else { 1318.5 };
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let square = phase.sin() + 0.25 * (3.0 * phase).sin();
        let env = if t < split {
            1.0
        } else {
            (-(t - split) / 0.22).exp()
        };
        *sample = square * env * (t / 0.001).min(1.0);
    }
    normalise(&mut out, 0.85);
    out
}

/// The mirror of the riser: a bright hiss falling away for exits and drops.
fn sweep_down() -> Vec<f32> {
    let n = (0.7 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0x5eed_d0d0;
    let mut low = 0.0;
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / n as f32;
        let cutoff = 4200.0 * (1.0 - t * 0.92) + 220.0;
        let hiss = noise(&mut state) - lowpass(noise(&mut state), &mut low, cutoff);
        let hz = 1400.0 * (1.0 - t) + 160.0;
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let tone = phase.sin() * 0.3;
        let env = (1.0 - t) * (1.0 - t) + 0.03 * (1.0 - t);
        *sample = (hiss * 0.7 + tone) * env;
    }
    normalise(&mut out, 0.85);
    out
}

/// A zapping laser: a tone falling fast with a wobble on it.
fn laser() -> Vec<f32> {
    let n = (0.5 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let hz = 2200.0 * (-t * 6.0).exp() + 220.0;
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let wobble = 1.0 + 0.25 * (2.0 * std::f32::consts::PI * 30.0 * t).sin();
        *sample = phase.sin() * 0.55 * wobble * (-t * 6.0).exp();
    }
    normalise(&mut out, 0.85);
    out
}

/// A punchy hit: a low thump with a noisy snap on top.
fn punch() -> Vec<f32> {
    let n = (0.35 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0x9e5c_1700;
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let hz = 60.0 + 110.0 * (-t * 18.0).exp();
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let thump = phase.sin() * (-t / 0.12).exp();
        let snap = noise(&mut state) * (-t / 0.012).exp() * 0.5;
        *sample = thump * 0.9 + snap;
    }
    normalise(&mut out, 0.95);
    out
}

/// A glittering sparkle: little high bells over a shimmer.
fn sparkle() -> Vec<f32> {
    let n = (0.9 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0x5154_4152;
    let bells = [5232.5, 6592.5, 7840.0, 10_466.0];
    let ats = [0.0, 0.12, 0.24, 0.36];
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let mut rung = 0.0;
        let mut b = 0;
        while b < 4 {
            let at = ats[b];
            if t >= at {
                let dt = t - at;
                rung +=
                    (2.0 * std::f32::consts::PI * bells[b] * dt).sin() * (-dt / 0.3).exp() * 0.25;
            }
            b += 1;
        }
        let shimmer = noise(&mut state) * 0.06 * (-t / 0.4).exp();
        *sample = (rung + shimmer) * (t / 0.002).min(1.0);
    }
    normalise(&mut out, 0.8);
    out
}

/// A vinyl scratch: a chirp dragged back and forth over noise.
fn scratch() -> Vec<f32> {
    let n = (0.8 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0x5c12_a7c4;
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / n as f32;
        let drag = (t * 3.0 * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;
        let hz = 400.0 + 1400.0 * drag;
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let env = (1.0 - t) * (0.4 + 0.6 * drag);
        *sample = (phase.sin() * 0.3 + noise(&mut state) * 0.35) * env;
    }
    normalise(&mut out, 0.85);
    out
}

/// A camera shutter: two quick clicks, open and close.
fn shutter() -> Vec<f32> {
    let n = (0.3 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut state = 0x5177_7300;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let first = (-((t - 0.02) * 220.0).powi(2)).exp();
        let second = (-((t - 0.16) * 200.0).powi(2)).exp();
        *sample = noise(&mut state) * (first + second) * 0.8;
    }
    normalise(&mut out, 0.9);
    out
}

/// A tape rewind: a rising chirp chopped into quick pulses.
fn rewind() -> Vec<f32> {
    let n = (1.0 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    let mut phase = 0.0;
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let hz = 300.0 + 2400.0 * t * t;
        phase += 2.0 * std::f32::consts::PI * hz / RATE as f32;
        let chop = if (t * 28.0).fract() < 0.55 { 1.0 } else { 0.25 };
        let env = 0.15 + 0.85 * (1.0 - (2.0 * t - 0.6).abs().min(1.0));
        *sample = phase.sin() * 0.4 * chop * env;
    }
    normalise(&mut out, 0.8);
    out
}

/// An error buzz: two flat low tones saying "try again".
fn error() -> Vec<f32> {
    let n = (0.6 * RATE as f32) as usize;
    let mut out = vec![0.0; n];
    for (i, sample) in out.iter_mut().enumerate() {
        let t = i as f32 / RATE as f32;
        let on = if t < 0.22 || (t >= 0.3 && t < 0.52) {
            1.0
        } else {
            0.0
        };
        let square = if (2.0 * std::f32::consts::PI * 165.0 * t).sin() >= 0.0 {
            1.0
        } else {
            -1.0
        };
        *sample = square * 0.32 * on;
    }
    normalise(&mut out, 0.8);
    out
}

/// A mono 16-bit WAV of the samples - the file every player reads.
fn write_wav(path: &Path, samples: &[f32]) -> std::io::Result<()> {
    let data_len = (samples.len() * 2) as u32;
    let mut bytes = Vec::with_capacity(44 + samples.len() * 2);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
    bytes.extend_from_slice(&1u16.to_le_bytes()); // one channel
    bytes.extend_from_slice(&RATE.to_le_bytes());
    bytes.extend_from_slice(&(RATE * 2).to_le_bytes()); // byte rate
    bytes.extend_from_slice(&2u16.to_le_bytes()); // block align
    bytes.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_len.to_le_bytes());
    for sample in samples {
        let held = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
        bytes.extend_from_slice(&held.to_le_bytes());
    }
    std::fs::write(path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A folder of its own for one test, emptied before and after.
    fn scratch(name: &str) -> AppDirs {
        let dir = std::env::temp_dir().join(format!("concat-sfx-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        AppDirs::under(&dir)
    }

    /// The first ask writes every file; a second finds them there and
    /// writes nothing over them.
    #[test]
    fn every_sound_is_made_once() {
        let dirs = scratch("made");
        ensure(&dirs);
        for sound in &BUILTIN {
            assert!(file_in(&dirs, sound.id).is_file(), "{} was made", sound.id);
        }
        let first = std::fs::read(file_in(&dirs, "whoosh")).unwrap();
        ensure(&dirs);
        let again = std::fs::read(file_in(&dirs, "whoosh")).unwrap();
        assert_eq!(first, again, "a sound already made is left alone");
        let _ = std::fs::remove_dir_all(&dirs.config);
    }

    /// A made file is a whole WAV: its header says what the sound is,
    /// and its samples are not silence.
    #[test]
    fn a_made_sound_is_a_whole_wav() {
        let dirs = scratch("wav");
        let path = ensure_one(&dirs, "click").unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
        let rate = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        assert_eq!(rate, RATE);
        let bits = u16::from_le_bytes(bytes[34..36].try_into().unwrap());
        assert_eq!(bits, 16);
        let want = (get("click").unwrap().duration * RATE as f64) as usize * 2;
        assert_eq!(
            u32::from_le_bytes(bytes[40..44].try_into().unwrap()) as usize,
            want
        );
        let loud = bytes[44..]
            .chunks_exact(2)
            .map(|held| i16::from_le_bytes(held.try_into().unwrap()).unsigned_abs())
            .max()
            .unwrap();
        assert!(loud > 8_000, "the click is audible: {loud}");
        let _ = std::fs::remove_dir_all(&dirs.config);
    }

    /// The envelope is sixty-four bars, each a share in `0..=1`, and it
    /// hears the sound: the loudest part of an explosion is its front.
    #[test]
    fn the_envelope_of_a_sound_is_sixty_four_bars() {
        let bars = bars("explosion");
        assert_eq!(bars.len(), super::super::sounds::BARS);
        assert!(bars.iter().all(|bar| (0.0..=1.0).contains(bar)));
        let loudest = bars
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(at, _)| at)
            .unwrap();
        assert!(loudest < 8, "the burst is at the front: bar {loudest}");
    }

    /// The same id is always the same samples, and an unknown id is
    /// silence rather than a panic.
    #[test]
    fn a_sound_is_deterministic() {
        assert_eq!(render("boom"), render("boom"));
        assert!(render("not-a-sound").is_empty());
    }

    /// An unknown id is refused, and its name is in the words.
    #[test]
    fn an_unknown_id_is_refused() {
        let dirs = scratch("unknown");
        let error = ensure_one(&dirs, "not-a-sound").unwrap_err();
        assert!(error.contains("not-a-sound"));
        let _ = std::fs::remove_dir_all(&dirs.config);
    }
}
