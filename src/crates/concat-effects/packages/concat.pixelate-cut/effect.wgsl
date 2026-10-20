fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let k = 1.0 - abs(progress * 2.0 - 1.0);
    let block = mix(1.0, 48.0, k);
    let snapped = (floor(uv * frame.size / block) + vec2<f32>(0.5, 0.5)) * block / frame.size;
    if (progress < 0.5) {
        return from_at(snapped);
    }
    return to_at(snapped);
}
