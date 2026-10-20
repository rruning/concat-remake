fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let k = sin(progress * 3.14159265);
    let zf = 1.0 + k * 0.35;
    let fromuv = vec2<f32>(0.5, 0.5) + (uv - vec2<f32>(0.5, 0.5)) / zf;
    return mix(from_at(fromuv), to_at(uv), smoothstep(0.35, 0.65, progress));
}
