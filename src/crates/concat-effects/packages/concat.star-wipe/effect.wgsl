fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let aspect = frame.size.x / frame.size.y;
    let p = (uv - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect, 1.0);
    let ang = atan2(p.y, p.x);
    let star = 0.5 + 0.5 * abs(cos(2.5 * ang));
    let reach = progress * 1.6 * star;
    let m = smoothstep(0.0, 0.02, reach - length(p));
    return mix(from_at(uv), to_at(uv), m);
}
