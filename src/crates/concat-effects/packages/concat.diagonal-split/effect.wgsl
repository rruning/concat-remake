fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let s = uv.x - uv.y;
    let sweep = progress * 2.2 - 1.1;
    let m = smoothstep(-0.01, 0.01, sweep - s);
    return mix(from_at(uv), to_at(uv), m);
}
