use super::{
    asset::GameAssets,
    audio::play_se,
    life_bar::{spawn_life_bar, LifeBarResource},
    player::Player,
    set::GameSet,
    states::GameState,
};
use crate::game::constant::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub life: i32,
    pub max_life: i32,
}

static ENEMY_MOVE_FORCE: f32 = 50000.0;

static ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 5.0;

pub fn spawn_enemy(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    position: Vec2,
    life_bar_locals: &Res<LifeBarResource>,
) {
    commands
        .spawn((
            Name::new("enemy"),
            StateScoped(GameState::InGame),
            Enemy {
                life: 20,
                max_life: 20,
            },
            AsepriteAnimationBundle {
                aseprite: aseprite,
                transform: Transform::from_translation(position.extend(5.0)),
                animation: Animation::default().with_tag("idle").with_speed(0.2),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(8.0),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 6.0,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            ExternalImpulse::default(),
        ))
        .with_children(|mut parent| {
            spawn_life_bar(&mut parent, &life_bar_locals);
        });
}

fn setup_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    life_bar_res: Res<LifeBarResource>,
) {
    spawn_enemy(
        &mut commands,
        assets.slime.clone(),
        Vec2::new(TILE_SIZE * 8.0, TILE_SIZE * -10.0),
        &life_bar_res,
    );

    spawn_enemy(
        &mut commands,
        assets.slime.clone(),
        Vec2::new(TILE_SIZE * 13.0, TILE_SIZE * -10.0),
        &life_bar_res,
    );
}

pub fn update_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut query: Query<(Entity, &Enemy, &mut ExternalForce, &GlobalTransform), Without<Camera2d>>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    let player = player_query.get_single();

    for (entity, enemy, mut force, enemy_transform) in query.iter_mut() {
        // 1マス以上5マス以内にプレイヤーがいたら追いかける
        if let Ok(player_transform) = player {
            let diff = player_transform.translation() - enemy_transform.translation();
            if TILE_SIZE < diff.length() && diff.length() < ENEMY_DETECTION_RANGE {
                let direction = diff.normalize_or_zero();
                force.force = direction.truncate() * ENEMY_MOVE_FORCE;
            } else {
                force.force = Vec2::ZERO;
            }
        }

        // ライフが0以下になったら消滅
        if enemy.life <= 0 {
            commands.entity(entity).despawn_recursive();
            play_se(&mut commands, assets.hiyoko.clone());
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_enemy);
        app.add_systems(
            FixedUpdate,
            update_enemy
                .run_if(in_state(GameState::InGame))
                .in_set(GameSet),
        );
    }
}
