fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let n = 8.0;
    let col = floor(uv.x * n);
    let even = (col - 2.0 * floor(col * 0.5)) < 0.5;
    let dir = select(-1.0, 1.0, even);
    let shift = dir * progress;
    let yf = uv.y + shift;
    if (yf >= 0.0 && yf <= 1.0) {
        return from_at(vec2<f32>(uv.x, yf));
    }
    let yt = uv.y + shift - dir;
    return to_at(vec2<f32>(uv.x, yt));
}
