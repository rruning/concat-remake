fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let aspect = frame.size.x / frame.size.y;
    let p = (uv - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect, 1.0);
    let s = 0.2 + progress * progress * 4.0;
    let x = p.x * 2.4 / s;
    let y = -p.y * 2.4 / s;
    let t = x * x + y * y - 1.0;
    let f = t * t * t - x * x * y * y * y;
    let m = 1.0 - smoothstep(-0.02, 0.02, f);
    return mix(from_at(uv), to_at(uv), m);
}
