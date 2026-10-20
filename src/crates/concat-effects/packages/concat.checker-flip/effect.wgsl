fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let cells = 8.0;
    let c = floor(uv * cells);
    let d = fract((c.x + c.y) * 0.5) * 0.5;
    let fl = clamp((progress - d) * 1.34, 0.0, 1.0);
    if (fl > 0.5) {
        return to_at(uv);
    }
    return from_at(uv);
}
