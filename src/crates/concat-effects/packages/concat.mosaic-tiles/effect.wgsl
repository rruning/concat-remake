fn kx_rand(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let n = 6.0;
    let c = floor(uv * n);
    let local = fract(uv * n) - vec2<f32>(0.5, 0.5);
    let d = kx_rand(c) * 0.5;
    let s = clamp((progress - d) * 2.0, 0.0, 1.0);
    let inside = max(abs(local.x), abs(local.y)) <= s * 0.5;
    if (inside) {
        return to_at(uv);
    }
    return from_at(uv);
}
