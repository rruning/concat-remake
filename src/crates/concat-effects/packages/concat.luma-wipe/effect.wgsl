fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let lum = luma(from_at(uv).rgb);
    let thr = 1.0 - progress;
    let m = smoothstep(thr - 0.08, thr + 0.08, lum);
    return mix(from_at(uv), to_at(uv), m);
}
