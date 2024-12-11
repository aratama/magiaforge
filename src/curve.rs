pub fn jump_curve(t_max: f32, h: f32, t: f32) -> f32 {
    let t0 = t_max * 0.5;
    let y = -(h / t0.powf(2.0)) * (t - t0).powf(2.0) + h;
    return y.max(0.0);
}
