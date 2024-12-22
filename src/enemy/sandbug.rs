use crate::asset::GameAssets;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::{Actor, ActorEvent, ActorGroup};
use crate::entity::life::LifeBeingSprite;
use crate::hud::life_bar::LifeBarResource;
use crate::spell::SpellType;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationState, AseSpriteAnimation};
use bevy_rapier2d::plugin::PhysicsSet;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

pub fn spawn_sandbag(
    mut commands: &mut Commands,
    aseprite: &Res<GameAssets>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
) {
    spawn_basic_enemy(
        &mut commands,
        aseprite.sandbug.clone(),
        position,
        life_bar_locals,
        Sandbug {
            home: position,
            animation: 0,
        },
        "sandbag",
        SpellType::SlimeCharge,
        ENEMY_MOVE_FORCE,
        0,
        ActorGroup::Enemy,
        None,
        10000000,
    );
}

#[derive(Component)]
struct Sandbug {
    home: Vec2,
    animation: u32,
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
    mut sprite_query: Query<(&mut AseSpriteAnimation, &mut AnimationState), With<LifeBeingSprite>>,
    mut sandbag_query: Query<(&mut Sandbug, &Children)>,
) {
    for event in actor_event.read() {
        let ActorEvent::Damaged { actor, .. } = event;
        if let Ok((mut sandbag, children)) = sandbag_query.get_mut(*actor) {
            for child in children {
                if let Ok((mut aseprite, mut animation_state)) = sprite_query.get_mut(*child) {
                    info!("frown");
                    aseprite.animation.tag = Some("frown".to_string());
                    animation_state.current_frame = 2;
                    sandbag.animation = 0;
                }
            }
        }
    }
}

fn idle(
    mut sprite_query: Query<(&mut AseSpriteAnimation, &mut AnimationState), With<LifeBeingSprite>>,
    mut sandbag_query: Query<(&mut Sandbug, &Children)>,
) {
    for (mut sandbag, children) in sandbag_query.iter_mut() {
        sandbag.animation += 1;
        if sandbag.animation == 60 {
            for child in children {
                if let Ok((mut aseprite, mut animation_state)) = sprite_query.get_mut(*child) {
                    aseprite.animation.tag = Some("idle".to_string());
                    animation_state.current_frame = 0;
                }
            }
        }
    }
}

pub struct SandbagPlugin;

impl Plugin for SandbagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (frown_on_damage, idle).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            FixedUpdate,
            go_back
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
