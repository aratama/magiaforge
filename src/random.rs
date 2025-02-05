use bevy::prelude::*;

/// 方向ベクトルを角度と長さについてランダムに変更します。
#[allow(dead_code)]
pub fn randomize_velocity(velocity: Vec2, angle_error: f32, length_error: f32) -> Vec2 {
    let angle = velocity.to_angle() + (rand::random::<f32>() - 0.5) * angle_error;
    let length = velocity.length() * (1.0 + (rand::random::<f32>() - 0.5) * length_error);
    let velocity = Vec2::from_angle(angle) * length;
    velocity
}
