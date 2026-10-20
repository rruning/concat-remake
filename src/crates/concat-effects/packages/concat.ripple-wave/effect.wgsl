struct Params { strength: f32 }

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let dir = uv - vec2<f32>(0.5, 0.5);
    let r = max(length(dir), 0.0001);
    let amp = sin(progress * 3.14159265) * params.strength * 0.0005;
    let wave = sin(r * 48.0 - progress * 30.0) * amp;
    let suv = uv + dir / r * wave;
    return mix(from_at(suv), to_at(suv), smoothstep(0.0, 1.0, progress));
}
