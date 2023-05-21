pub fn move_toward(current: f32, target: f32, acceleration: f32, delta: f32) -> f32 {
    let dv = target - current;
    if dv == 0.0 {
        return target;
    }
    match (target - current).is_sign_positive() {
        true => return (current + acceleration * delta).min(target),
        false => return (current - acceleration * delta).max(target),
    }
}
