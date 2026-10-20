struct Params { amount: f32, speed: f32 }

// The picture drawn in and let back on a steady beat: the scale rides
// the rise of a sine, so it eases at each end the way a thing bounces,
// and the frame is sampled nearer its centre the tighter it is.

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let beat = abs(sin(frame.time * params.speed * 2.2));
    let zoom = 1.0 + params.amount * 0.01 * 0.22 * beat;
    return sample(vec2<f32>(0.5, 0.5) + (uv - vec2<f32>(0.5, 0.5)) / zoom);
}