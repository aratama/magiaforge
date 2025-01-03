use crate::asset::GameAssets;
use crate::constant::*;
use crate::enemy::basic::spawn_basic_enemy;
use crate::entity::actor::Actor;
use crate::entity::actor::ActorFireState;
use crate::entity::actor::ActorGroup;
use crate::finder::Finder;
use crate::hud::life_bar::LifeBarResource;
use crate::set::GameSet;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct EyeballControl;

const ENEMY_MOVE_FORCE: f32 = 100000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 10.0;

const ENEMY_ATTACK_RANGE: f32 = TILE_SIZE * 8.0;

pub fn spawn_eyeball(
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
    actor_group: ActorGroup,
    golds: u32,
) {
    spawn_basic_enemy(
        &mut commands,
        &assets,
        match actor_group {
            ActorGroup::Player => assets.eyeball_friend.clone(),
            ActorGroup::Enemy => assets.eyeball.clone(),
            ActorGroup::Neutral => assets.eyeball_friend.clone(),
        },
        position,
        life_bar_locals,
        EyeballControl,
        "eyeball",
        Some(SpellType::PurpleBolt),
        ENEMY_MOVE_FORCE,
        golds,
        actor_group,
        None,
        25,
        8.0,
    );
}

fn control_eyeball(
    mut actor_query: Query<(
        Entity,
        Option<&mut EyeballControl>,
        &mut Actor,
        &mut Transform,
    )>,
    rapier_context: Query<&RapierContext, With<DefaultRapierContext>>,
) {
    let finder = Finder::new(&actor_query);

    // 各アイボールの行動を選択します
    for (eyeball_entity, eyeball_optional, mut eyeball_actor, eyeball_transform) in
        actor_query.iter_mut()
    {
        if let Some(_) = eyeball_optional {
            eyeball_actor.move_direction = Vec2::ZERO;
            eyeball_actor.fire_state = ActorFireState::Idle;

            // 最も近くにいる、別グループのアクターに対して接近または攻撃
            let origin = eyeball_transform.translation.truncate();
            if let Some(nearest) = finder.nearest(
                &rapier_context,
                eyeball_entity,
                eyeball_actor.actor_group,
                origin,
            ) {
                let diff = nearest.position - origin;
                if diff.length() < ENEMY_ATTACK_RANGE {
                    eyeball_actor.move_direction = Vec2::ZERO;
                    eyeball_actor.pointer = diff;
                    eyeball_actor.fire_state = ActorFireState::Fire;
                } else if diff.length() < ENEMY_DETECTION_RANGE {
                    eyeball_actor.move_direction = diff.normalize_or_zero();
                    eyeball_actor.fire_state = ActorFireState::Idle;
                }
            }
        }
    }
}

pub struct EyeballControlPlugin;

impl Plugin for EyeballControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            control_eyeball
                .run_if(in_state(GameState::InGame).and(in_state(TimeState::Active)))
                .in_set(GameSet)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
