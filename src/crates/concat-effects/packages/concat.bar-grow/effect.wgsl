fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let n = 10.0;
    let local = fract(uv.x * n) - 0.5;
    let m = step(abs(local) * 2.0, progress);
    return mix(from_at(uv), to_at(uv), m);
}
