use super::constant::*;
use super::{
    asset::GameAssets,
    audio::play_se,
    life_bar::{spawn_life_bar, LifeBarResource},
    player::Player,
    set::GameSet,
    states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub life: i32,
    pub max_life: i32,
}

const ENEMY_MOVE_FORCE: f32 = 50000.0;

const ENEMY_DETECTION_RANGE: f32 = TILE_SIZE * 5.0;

const ENEMY_ATTACK_POINT: i32 = 8;

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
                linear_damping: 10.0,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            ExternalImpulse::default(),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(ENEMY_GROUP, Group::ALL),
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
    mut player_query: Query<(Entity, &mut Player, &GlobalTransform, &mut ExternalImpulse)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    if let Ok((player_entity, mut player, player_transform, mut player_impulse)) =
        player_query.get_single_mut()
    {
        for (entity, enemy, mut force, enemy_transform) in query.iter_mut() {
            // 1マス以上5マス以内にプレイヤーがいたら追いかける

            let diff = player_transform.translation() - enemy_transform.translation();
            if diff.length() < ENEMY_DETECTION_RANGE {
                let direction = diff.normalize_or_zero();
                force.force = direction.truncate() * ENEMY_MOVE_FORCE;
            } else {
                force.force = Vec2::ZERO;
            }

            // ライフが0以下になったら消滅
            if enemy.life <= 0 {
                commands.entity(entity).despawn_recursive();
                play_se(&mut commands, assets.hiyoko.clone());
            }
        }

        for collision_event in collision_events.read() {
            match collision_event {
                CollisionEvent::Started(a, b, _) => {
                    if *a == player_entity {
                        if let Ok(en) = query.get(*b) {
                            process_attack_event(
                                &en.3,
                                &mut player,
                                &player_transform,
                                &mut player_impulse,
                                &mut commands,
                                &assets,
                            );
                        };
                    } else if *b == player_entity {
                        if let Ok(en) = query.get(*a) {
                            process_attack_event(
                                &en.3,
                                &mut player,
                                &player_transform,
                                &mut player_impulse,
                                &mut commands,
                                &assets,
                            );
                        };
                    }
                }
                _ => {}
            }
        }
    }
}

fn process_attack_event(
    enemy_transform: &GlobalTransform,
    player: &mut Player,
    player_transform: &GlobalTransform,
    player_impulse: &mut ExternalImpulse,
    mut commands: &mut Commands,
    assets: &Res<GameAssets>,
) {
    let direction = player_transform.translation() - enemy_transform.translation();
    let impulse = direction.normalize_or_zero() * 20000.0;
    player_impulse.impulse = impulse.truncate();
    player.life = (player.life - ENEMY_ATTACK_POINT).max(0);

    play_se(&mut commands, assets.dageki.clone());
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
