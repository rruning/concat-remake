fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let pick = select(0.0, 1.0, progress > 0.5);
    let base = mix(from_at(uv), to_at(uv), pick);
    let flash = 1.0 - abs(progress - 0.5) * 2.0;
    return mix(base, vec4<f32>(1.0, 1.0, 1.0, 1.0), smoothstep(0.55, 1.0, flash));
}
