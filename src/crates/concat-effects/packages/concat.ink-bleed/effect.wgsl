fn kx_rand(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let n = kx_rand(floor(uv * 64.0)) * 0.5 + kx_rand(floor(uv * 17.0)) * 0.5;
    let t = progress * 1.1;
    let m = smoothstep(n - 0.04, n + 0.04, t);
    return mix(from_at(uv), to_at(uv), m);
}
