use crate::asset::GameAssets;
use crate::constant::*;
use crate::curve::jump_curve;
use crate::enemy::slime::spawn_slime;
use crate::hud::life_bar::LifeBarResource;
use crate::se::{SECommand, SE};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct SlimeSeed {
    animation: u32,
    from: Vec2,
    to: Vec2,
    speed: u32,
}

#[derive(Component)]
pub struct SlimeSeedSprite;

#[derive(Event)]
pub struct SpawnSlimeSeed {
    pub from: Vec2,
    pub to: Vec2,
}

pub fn spawn_slime_seed(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut reader: EventReader<SpawnSlimeSeed>,
) {
    for SpawnSlimeSeed { from, to } in reader.read() {
        commands
            .spawn((
                StateScoped(GameState::InGame),
                SlimeSeed {
                    animation: 0,
                    from: *from,
                    to: *to,
                    speed: 60 + rand::random::<u32>() % 30,
                },
                AseSpriteSlice {
                    aseprite: assets.atlas.clone(),
                    name: "slime_shadow".into(),
                },
                Transform::from_translation(from.extend(ENTITY_LAYER_Z)),
            ))
            .with_child((
                SlimeSeedSprite,
                AseSpriteAnimation {
                    aseprite: assets.slime.clone(),
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
    mut se_writer: EventWriter<SECommand>,
) {
    for (entity, mut seed, mut transform) in query.iter_mut() {
        seed.animation += 1;
        transform.translation = seed
            .from
            .lerp(seed.to, seed.animation as f32 / seed.speed as f32)
            .extend(ENTITY_LAYER_Z);
        if seed.animation == seed.speed {
            commands.entity(entity).despawn_recursive();
            spawn_slime(
                &mut commands,
                &assets,
                seed.to,
                &life_bar_locals,
                30 + rand::random::<u32>() % 30,
                0,
            );
            se_writer.send(SECommand::pos(SE::Bicha, seed.to));
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
