use crate::asset::GameAssets;
use crate::component::life::LifeBeingSprite;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorEvent;
use crate::entity::actor::ActorGroup;
use crate::hud::life_bar::LifeBarResource;
use crate::set::FixedUpdateGameActiveSet;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteAnimation;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

/// Sandbugは敵のアクターグループですが攻撃を行いません
#[derive(Component)]
struct Sandbug {
    home: Vec2,
    animation: u32,
}

pub fn spawn_sandbag(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
) {
    spawn_basic_enemy(
        &mut commands,
        &assets,
        assets.sandbug.clone(),
        position,
        life_bar_locals,
        Sandbug {
            home: position,
            animation: 0,
        },
        "sandbag",
        None,
        ENEMY_MOVE_FORCE,
        0,
        ActorGroup::Enemy,
        None,
        10000000,
        8.0,
    );
}

fn go_back(mut query: Query<(&mut Actor, &Transform, &Sandbug)>) {
    for (mut actor, witch_transform, dummy) in query.iter_mut() {
        let diff = dummy.home - witch_transform.translation.truncate();
        actor.move_direction = if 8.0 < diff.length() {
            diff.normalize_or_zero()
        } else {
            Vec2::ZERO
        };
    }
}

fn frown_on_damage(
    mut actor_event: EventReader<ActorEvent>,
    mut sprite_query: Query<&mut AseSpriteAnimation, With<LifeBeingSprite>>,
    mut sandbag_query: Query<(&mut Sandbug, &Children)>,
) {
    for event in actor_event.read() {
        match *event {
            ActorEvent::Damaged { actor, .. } => {
                if let Ok((mut sandbag, children)) = sandbag_query.get_mut(actor) {
                    for child in children {
                        if let Ok(mut aseprite) = sprite_query.get_mut(*child) {
                            aseprite.animation.tag = Some("frown".to_string());
                            sandbag.animation = 0;
                        }
                    }
                }
            }
            ActorEvent::FatalFall { .. } => {}
        }
    }
}

fn idle(
    mut sprite_query: Query<&mut AseSpriteAnimation, With<LifeBeingSprite>>,
    mut sandbag_query: Query<(&mut Sandbug, &Children)>,
) {
    for (mut sandbag, children) in sandbag_query.iter_mut() {
        sandbag.animation += 1;
        if sandbag.animation == 60 {
            for child in children {
                if let Ok(mut aseprite) = sprite_query.get_mut(*child) {
                    aseprite.animation.tag = Some("idle".to_string());
                }
            }
        }
    }
}

pub struct SandbagPlugin;

impl Plugin for SandbagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (frown_on_damage, idle, go_back).in_set(FixedUpdateGameActiveSet),
        );
    }
}
