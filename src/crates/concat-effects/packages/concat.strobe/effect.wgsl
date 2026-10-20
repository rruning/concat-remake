struct Params { amount: f32, speed: f32 }

// White let over the picture a quarter of every beat and gone the rest
// of it - a strobe, not a fade: the flash is a square wave of light,
// screened onto whatever is under it by the amount.

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let c = sample(uv);
    let on = step(fract(frame.time * params.speed), 0.25);
    let flash = params.amount * 0.01 * on * 0.85 * c.a;
    return vec4<f32>(mix(c.rgb, vec3<f32>(1.0) * c.a, flash), c.a);
}