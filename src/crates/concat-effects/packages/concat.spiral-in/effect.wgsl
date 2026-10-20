fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let aspect = frame.size.x / frame.size.y;
    let p = (uv - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect, 1.0);
    let ang = atan2(p.y, p.x);
    let sval = fract(ang / 6.28318530 + length(p) * 3.0);
    let m = smoothstep(-0.02, 0.02, progress * 1.1 - sval);
    return mix(from_at(uv), to_at(uv), m);
}
