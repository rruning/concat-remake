fn kx_rot(v: vec2<f32>, ang: f32) -> vec2<f32> {
    let c = cos(ang);
    let s = sin(ang);
    return vec2<f32>(c * v.x - s * v.y, s * v.x + c * v.y);
}

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let aspect = frame.size.x / frame.size.y;
    let p = (uv - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect, 1.0);
    let a = progress * 3.14159265;
    let r1 = kx_rot(p, -a);
    let r2 = kx_rot(p, 3.14159265 - a);
    let suv1 = vec2<f32>(r1.x / aspect, r1.y) + vec2<f32>(0.5, 0.5);
    let suv2 = vec2<f32>(r2.x / aspect, r2.y) + vec2<f32>(0.5, 0.5);
    return mix(from_at(suv1), to_at(suv2), smoothstep(0.45, 0.55, progress));
}
