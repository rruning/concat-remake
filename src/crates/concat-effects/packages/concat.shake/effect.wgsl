struct Params { amount: f32, speed: f32 }

// The picture shaken about its centre: an offset walked through a smooth
// noise of time, by the amount as a share of the frame. The edges are
// the sample's own - a shake overruns the frame the way it does
// anywhere. The noise is the prelude's own hash, smoothed between whole
// seconds of the walk.

/// Smoothed noise in `0..=1` along one dimension of time.
fn wobble(t: f32, seed: f32) -> f32 {
    let i = floor(t);
    let f = fract(t);
    let a = hash(vec2<f32>(i, 0.0), seed);
    let b = hash(vec2<f32>(i + 1.0, 0.0), seed);
    return mix(a, b, f * f * (3.0 - 2.0 * f));
}

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let t = frame.time * params.speed * 6.0;
    let strength = params.amount * 0.01 * 0.06;
    let offset = vec2<f32>(wobble(t, 1.0) - 0.5, wobble(t, 37.0) - 0.5) * 2.0 * strength;
    return sample(uv + offset);
}