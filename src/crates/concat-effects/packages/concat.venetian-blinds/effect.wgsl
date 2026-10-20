fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let slats = 12.0;
    let f = fract(uv.y * slats);
    let m = smoothstep(f, f + 0.08, progress * 1.1);
    return mix(from_at(uv), to_at(uv), m);
}
