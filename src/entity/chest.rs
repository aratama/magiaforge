use super::{breakable::Breakable, gold::spawn_gold, EntityDepth};
use crate::{
    asset::GameAssets, audio::play_se, config::GameConfig, constant::*, states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;
use rand::random;

const ENTITY_WIDTH: f32 = 8.0;

const ENTITY_HEIGHT: f32 = 8.0;

#[derive(Default, Component, Reflect)]
struct Chest {
    pub golds: i32,
}

/// チェストを生成します
/// 指定する位置はスプライトの左上ではなく、重心のピクセル座標です
pub fn spawn_chest(commands: &mut Commands, aseprite: Handle<Aseprite>, x: f32, y: f32) {
    let tx = x + ENTITY_WIDTH - TILE_SIZE / 2.0;
    let ty = y - ENTITY_HEIGHT + TILE_SIZE / 2.0;
    commands.spawn((
        Name::new("chest"),
        StateScoped(GameState::InGame),
        Breakable { life: 30 },
        Chest { golds: 10 },
        EntityDepth,
        AsepriteSliceBundle {
            aseprite: aseprite,
            slice: "chest".into(),
            transform: Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            ..default()
        },
        Collider::cuboid(ENTITY_WIDTH, ENTITY_HEIGHT),
        CollisionGroups::new(WALL_GROUP, ACTOR_GROUP | BULLET_GROUP),
    ));
}

fn break_chest(
    mut commands: Commands,
    query: Query<(Entity, &Breakable, &Transform), With<Chest>>,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for (entity, breakabke, transform) in query.iter() {
        if breakabke.life <= 0 {
            commands.entity(entity).despawn_recursive();
            play_se(&audio, &config, assets.kuzureru.clone());

            for _ in 0..(3 + random::<i32>().abs() % 10) {
                spawn_gold(
                    &mut commands,
                    &assets,
                    transform.translation.x,
                    transform.translation.y,
                );
            }
        }
    }
}

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        // ここを FixedUpdate にするとパーティクルの発生位置がおかしくなる
        app.add_systems(FixedUpdate, break_chest.run_if(in_state(GameState::InGame)));
        app.register_type::<Chest>();
    }
}
