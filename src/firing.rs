use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 生成される弾丸の大半の情報を収めた構造体です
/// 実際に弾丸を生成する spawn_bullet 関数のパラメータとして使われるほか、
/// リモートで送信される RemoteMessage::Fire のデータとしても共通で使われることで、
/// ローカルとリモートの弾丸生成を共通化します
#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Firing {
    pub sender: Option<Uuid>,
    pub uuid: Uuid,
    pub position: Vec2,
    pub velocity: Vec2,
    pub bullet_lifetime: u32,
    pub damage: i32,
    pub impulse: f32,
    pub slice: String,
    pub collier_radius: f32,
    pub light_intensity: f32,
    pub light_radius: f32,
    pub light_color_hlsa: [f32; 4],
    pub homing: f32,
}
