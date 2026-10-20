fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let edge = 1.0 - progress;
    if (uv.x > edge) {
        return to_at(uv);
    }
    let mx = 2.0 * edge - uv.x;
    if (mx <= 1.0) {
        return from_at(vec2<f32>(mx, uv.y)) * 0.8;
    }
    return from_at(uv);
}
