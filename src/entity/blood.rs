use crate::actor::Actor;
use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::constant::{GameActors, BLOOD_LAYER_Z};
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Deserialize)]
pub enum Blood {
    Red,
    Blue,
}

fn blood(
    mut commands: Commands,
    assets: Res<GameAssets>,
    ron: Res<Assets<GameActors>>,
    query: Query<(&Life, &Transform, &Actor)>,
) {
    let constants = ron.get(assets.actors.id()).unwrap();
    for (life, transform, actor) in query.iter() {
        if life.life <= 0 {
            let props = actor.to_props(&constants);
            if let Some(ref blood) = props.blood {
                let position = transform.translation.truncate();
                commands.spawn((
                    StateScoped(GameState::InGame),
                    AseSpriteSlice {
                        aseprite: assets.atlas.clone(),
                        name: match blood {
                            Blood::Red => format!("blood_{}", rand::random::<u8>() % 3),
                            Blood::Blue => format!("slime_blood_{}", rand::random::<u8>() % 3),
                        },
                    },
                    Transform::from_translation(position.extend(BLOOD_LAYER_Z))
                        .with_scale(Vec3::new(2.0, 2.0, 1.0)),
                ));
            }
        }
    }
}

pub struct BloodPlugin;

impl Plugin for BloodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (blood).in_set(FixedUpdateGameActiveSet));
    }
}
