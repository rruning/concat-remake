fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let gap = progress * 0.5;
    if (abs(uv.x - 0.5) < gap) {
        return to_at(uv);
    }
    let shift = (uv.x - 0.5) * (1.0 - progress * 0.3);
    return from_at(vec2<f32>(0.5 + shift, uv.y));
}
