struct Params { amount: f32, speed: f32 }

// The frame jumped one way and then another, held between jumps: the
// offset is a fresh throw of the prelude's hash at every step of the
// clock, small and quick - jitter, not a shake, which wanders. The
// edges are the sample's own, as anywhere the picture overruns the
// frame.

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let step_n = floor(frame.time * params.speed * 24.0);
    let strength = params.amount * 0.01 * 0.025;
    let offset = (vec2<f32>(hash(vec2<f32>(step_n, 0.0), 1.0),
                            hash(vec2<f32>(step_n, 0.0), 13.0)) - 0.5) * 2.0 * strength;
    return sample(uv + offset);
}