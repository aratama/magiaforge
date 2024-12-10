use crate::asset::GameAssets;
use crate::constant::*;
use crate::controller::enemy::Enemy;
use crate::entity::actor::{Actor, ActorFireState, ActorState};
use crate::entity::life::Life;
use crate::entity::EntityDepth;
use crate::spell::SpellType;
use crate::states::GameState;
use crate::wand::{Wand, WandType};
use bevy::{prelude::*, sprite};
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::*;

#[derive(Component)]
pub struct HugeSlime {
    pub animation: u32,
    pub up_velocity: f32,
}

#[derive(Component)]
pub struct HugeSlimeSprite;

pub fn spawn_huge_slime(commands: &mut Commands, assets: &Res<GameAssets>, position: Vec2) {
    let mut slots = [None; MAX_SPELLS_IN_WAND];
    slots[0] = Some(SpellType::MagicBolt);

    commands
        .spawn((
            Name::new("huge slime"),
            StateScoped(GameState::InGame),
            Enemy,
            Life {
                life: 200,
                max_life: 200,
                amplitude: 0.0,
            },
            HugeSlime {
                animation: 0,
                up_velocity: 0.0,
            },
            Actor {
                uuid: Uuid::new_v4(),
                spell_delay: 0,
                mana: 1000,
                max_mana: 1000,
                pointer: Vec2::ZERO,
                intensity: 0.0,
                move_direction: Vec2::ZERO,
                move_force: 0.0,
                fire_state: ActorFireState::Idle,
                group: ENEMY_GROUP,
                filter: ENTITY_GROUP | WALL_GROUP | WITCH_GROUP,
                current_wand: 0,
                effects: default(),
                wands: [
                    Some(Wand {
                        wand_type: WandType::CypressWand,
                        slots,
                        index: 0,
                    }),
                    None,
                    None,
                    None,
                ],
            },
            ActorState::default(),
            EntityDepth,
            AseSpriteAnimation {
                aseprite: assets.huge_slime_shadow.clone(),
                animation: Animation::default().with_tag("idle"),
            },
            Transform::from_translation(position.extend(PAINT_LAYER_Z)),
            (
                RigidBody::Dynamic,
                Collider::ball(32.0),
                GravityScale(0.0),
                LockedAxes::ROTATION_LOCKED,
                Damping {
                    linear_damping: 10.0,
                    angular_damping: 1.0,
                },
                ExternalForce::default(),
                ExternalImpulse::default(),
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(
                    ENEMY_GROUP,
                    ENTITY_GROUP | WALL_GROUP | WITCH_GROUP | WITCH_BULLET_GROUP | ENEMY_GROUP,
                ),
            ),
        ))
        .with_children(|parent| {
            parent.spawn((
                HugeSlimeSprite,
                AseSpriteAnimation {
                    aseprite: assets.huge_slime.clone(),
                    animation: Animation::default().with_tag("idle"),
                },
                Transform::from_xyz(0.0, 30.0, ENTITY_LAYER_Z),
            ));
        });
}

fn update_huge_slime(
    mut query: Query<&mut HugeSlime>,
    mut sprite_query: Query<(&Parent, &mut Transform), With<HugeSlimeSprite>>,
) {
    const GRAVITY: f32 = 0.2;
    const JUMP_POWER: f32 = 5.0;
    const JUMP_TIMESPAN: u32 = 120;

    for (parent, mut transform) in sprite_query.iter_mut() {
        let mut huge_slime = query.get_mut(**parent).unwrap();

        huge_slime.animation += 1;
        if huge_slime.animation >= JUMP_TIMESPAN {
            huge_slime.animation = 0;
            huge_slime.up_velocity = JUMP_POWER;
        }

        huge_slime.up_velocity -= GRAVITY;
        transform.translation.y =
            (transform.translation.y + huge_slime.up_velocity as f32).max(0.0);
    }
}

pub struct HugeSlimePlugin;

impl Plugin for HugeSlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_huge_slime
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
