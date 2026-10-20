struct Params { amount: f32, speed: f32 }

// The frame thumping in twice a beat, the way a heart goes - a hard
// swell, a smaller one close behind it, and the long quiet between
// beats. The zoom rides those swells; the frame is sampled nearer its
// centre the tighter it is.

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let phase = fract(frame.time * params.speed * 0.9);
    let thump = exp(-phase * 16.0) + 0.55 * exp(-max(phase - 0.22, 0.0) * 16.0);
    let zoom = 1.0 + params.amount * 0.01 * 0.10 * thump;
    return sample(vec2<f32>(0.5, 0.5) + (uv - vec2<f32>(0.5, 0.5)) / zoom);
}