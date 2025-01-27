use crate::actor::Actor;
use crate::actor::ActorSpriteGroup;
use crate::component::counter::Counter;
use crate::entity::explosion::SpawnExplosion;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Default, Component, Reflect)]
pub struct Bomb;

fn explode_bomb(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Actor, &Counter), With<Bomb>>,
    mut explosion_writer: EventWriter<SpawnExplosion>,
) {
    for (entity, transform, life, counter) in query.iter() {
        if life.life <= 0 || 180 <= counter.count {
            let position = transform.translation.truncate();
            commands.entity(entity).despawn_recursive();

            explosion_writer.send(SpawnExplosion {
                position,
                radius: 60.0,
                impulse: 100000.0,
                damage: 100,
            });
        }
    }
}

fn set_bomb_rotation(
    bomb_query: Query<&Transform, With<Bomb>>,
    group_query: Query<&Parent, With<ActorSpriteGroup>>,
    mut sprite_query: Query<(&Parent, &mut Transform), (With<AseSpriteAnimation>, Without<Bomb>)>,
) {
    for (parent, mut sprite_transform) in sprite_query.iter_mut() {
        if let Ok(group) = group_query.get(parent.get()) {
            if let Ok(bomb_transform) = bomb_query.get(group.get()) {
                sprite_transform.rotation = Quat::from_rotation_z(
                    (bomb_transform.translation.x + bomb_transform.translation.y) * -0.1,
                );
            }
        }
    }
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (explode_bomb, set_bomb_rotation).in_set(FixedUpdateGameActiveSet),
        );
        app.register_type::<Bomb>();
    }
}
