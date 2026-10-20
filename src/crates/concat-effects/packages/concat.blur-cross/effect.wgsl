struct Params { radius: f32 }

fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    let r = (1.0 - abs(progress * 2.0 - 1.0)) * params.radius * 0.0004;
    var acc = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    for (var i: i32 = 0; i < 5; i++) {
        let off = (f32(i) - 2.0) * r;
        let o = vec2<f32>(off, off * 0.6);
        acc += mix(from_at(uv + o), to_at(uv + o), progress);
    }
    return acc / 5.0;
}
