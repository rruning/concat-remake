fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let aspect = frame.size.x / frame.size.y;
    let p = (uv - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect, 1.0);
    let q = vec2<f32>(p.x, p.y * 1.6);
    let reach = progress * 1.6;
    let m = 1.0 - smoothstep(reach - 0.02, reach + 0.02, length(q));
    return mix(from_at(uv), to_at(uv), m);
}
