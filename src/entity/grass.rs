use crate::asset::GameAssets;
use crate::component::entity_depth::EntityDepth;
use crate::constant::*;
use crate::entity::fire::Burnable;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component, Reflect)]
struct Grasses;

pub fn spawn_grasses(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    commands
        .spawn((
            Name::new("grasses"),
            Grasses,
            StateScoped(GameState::InGame),
            EntityDepth::new(),
            Burnable {
                life: 30 * 10 + rand::random::<u32>() % 30,
            },
            Visibility::default(),
            Transform::from_translation(position.extend(0.0)),
            (
                Sensor,
                Collider::cuboid(TILE_HALF, TILE_HALF),
                // 草はいかなる弾丸も妨げないので SENSOR_GROUP に属します
                *SENSOR_GROUPS,
                ActiveEvents::COLLISION_EVENTS,
            ),
        ))
        .with_children(|builder| {
            builder.spawn((
                Name::new("grass2"),
                Transform::from_xyz(0.0, 12.0, Z_ORDER_SCALE * 12.0),
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: format!("grass_{}", rand::random::<u32>() % 3).into(),
                },
            ));
            builder.spawn((
                Name::new("grass1"),
                Transform::from_xyz(0.0, 8.0, Z_ORDER_SCALE * 8.0),
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: format!("grass_{}", rand::random::<u32>() % 3).into(),
                },
            ));
            builder.spawn((
                Name::new("grass0"),
                Transform::from_xyz(0.0, 4.0, Z_ORDER_SCALE * 4.0),
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: format!("grass_{}", rand::random::<u32>() % 3).into(),
                },
            ));
        });
}

fn burnout(mut commands: Commands, query: Query<(Entity, &Burnable), With<Grasses>>) {
    for (entity, burnable) in query.iter() {
        if burnable.life <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            burnout
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
        app.register_type::<Grasses>();
    }
}
