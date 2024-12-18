use crate::asset::GameAssets;
use crate::constant::*;
use crate::curve::jump_curve;
use crate::enemy::slime::spawn_slime;
use crate::hud::life_bar::LifeBarResource;
use crate::level::tile::Tile;
use crate::level::CurrentLevel;
use crate::se::{SEEvent, SE};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use super::actor::ActorGroup;

#[derive(Component)]
pub struct SlimeSeed {
    animation: u32,
    from: Vec2,
    to: Vec2,
    speed: u32,
    actor_group: ActorGroup,
    owner: Entity,
}

#[derive(Component)]
pub struct SlimeSeedSprite;

#[derive(Event)]
pub struct SpawnSlimeSeed {
    pub from: Vec2,
    pub to: Vec2,
    pub actor_group: ActorGroup,
    pub owner: Entity,
}

pub fn spawn_slime_seed(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut reader: EventReader<SpawnSlimeSeed>,
) {
    for SpawnSlimeSeed {
        from,
        to,
        actor_group,
        owner,
    } in reader.read()
    {
        commands
            .spawn((
                Name::new("slime_seed"),
                StateScoped(GameState::InGame),
                SlimeSeed {
                    animation: 0,
                    from: *from,
                    to: *to,
                    speed: 60 + rand::random::<u32>() % 30,
                    actor_group: *actor_group,
                    owner: *owner,
                },
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "entity_shadow".into(),
                },
                Transform::from_translation(from.extend(SHADOW_LAYER_Z)),
            ))
            .with_child((
                SlimeSeedSprite,
                AseSpriteAnimation {
                    aseprite: match actor_group {
                        ActorGroup::Player => assets.friend_slime.clone(),
                        ActorGroup::Enemy => assets.slime.clone(),
                    },
                    animation: Animation::default().with_tag("idle"),
                },
            ));
    }
}

fn update_slime_seed(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SlimeSeed, &mut Transform)>,
    assets: Res<GameAssets>,
    life_bar_locals: Res<LifeBarResource>,
    mut se_writer: EventWriter<SEEvent>,
    current: Res<CurrentLevel>,
) {
    for (entity, mut seed, mut transform) in query.iter_mut() {
        seed.animation += 1;
        transform.translation = seed
            .from
            .lerp(seed.to, seed.animation as f32 / seed.speed as f32)
            .extend(SLIME_SEED_LAYER_Z);
        if seed.animation == seed.speed {
            commands.entity(entity).despawn_recursive();
            if let Some(ref chunk) = current.chunk {
                match chunk.get_tile_by_coords(seed.to) {
                    Tile::StoneTile => {
                        spawn_slime(
                            &mut commands,
                            &assets,
                            seed.to,
                            &life_bar_locals,
                            30 + rand::random::<u32>() % 30,
                            0,
                            seed.actor_group,
                            Some(seed.owner),
                        );
                        se_writer.send(SEEvent::pos(SE::Bicha, seed.to));
                    }
                    tile => {
                        warn!("SlimeSeed: Hit non-stone tile: {:?}", tile);
                    }
                }
            }
        }
    }
}

fn update_slime_seed_sprite(
    parent_query: Query<&SlimeSeed>,
    mut query: Query<(&Parent, &mut Transform), With<SlimeSeedSprite>>,
) {
    for (parent, mut transform) in query.iter_mut() {
        if let Ok(seed) = parent_query.get(parent.get()) {
            transform.translation.y = jump_curve(seed.speed as f32, 100.0, seed.animation as f32);
        }
    }
}

pub struct SlimeSeedPlugin;

impl Plugin for SlimeSeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSlimeSeed>();
        app.add_systems(
            FixedUpdate,
            (
                spawn_slime_seed,
                update_slime_seed,
                update_slime_seed_sprite,
            )
                .run_if(in_state(GameState::InGame))
                .before(PhysicsSet::SyncBackend),
        );
    }
}
