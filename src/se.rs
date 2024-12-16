use crate::{asset::GameAssets, audio::play_se, config::GameConfig};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_rapier2d::plugin::PhysicsSet;
use std::cmp::Ordering;

#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct SECommand {
    se: SE,
    position: Option<Vec2>,
}

impl SECommand {
    pub fn pos(se: SE, position: Vec2) -> Self {
        Self {
            se,
            position: Some(position),
        }
    }

    pub fn new(se: SE) -> Self {
        Self { se, position: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SE {
    Damage,
    NoDamage,
    Cry,
    Break,
    Click,
    Fire,
    Steps,
    TurnOn,
    Warp,
    PickUp,
    Heal,
    Switch,
    Drop,
    Growl,
    Puyon,
    Bicha,
    Kawaii,
    Register,
}

/// 効果音イベントを順次再生していきます
/// ただし、同一のフレームに同じ効果音が複数回再生されると極端に音が大きくなり不自然なため、
/// 同じ効果音が同時に複数回リクエストされても、最も距離が近いもののみが再生されます
fn process_se_commands(
    mut commands: Commands,
    assets: Res<GameAssets>,
    config: Res<GameConfig>,
    mut reader: EventReader<SECommand>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    let camera_position = camera_query.single().translation.truncate();

    let mut vec = Vec::<SECommand>::new();
    for command in reader.read() {
        vec.push(*command);
    }

    vec.sort_by(|a, b| {
        let la = (camera_position - a.position.unwrap_or(camera_position)).length_squared();
        let lb = (camera_position - b.position.unwrap_or(camera_position)).length_squared();
        la.partial_cmp(&lb).unwrap_or(Ordering::Equal)
    });

    let mut played = HashSet::<SE>::new();

    for SECommand { se, position } in vec.iter() {
        if played.contains(se) {
            continue;
        }

        played.insert(*se);

        let handle = match se {
            SE::Damage => &assets.dageki,
            SE::NoDamage => &assets.shibafu,
            SE::Cry => &assets.hiyoko,
            SE::Break => &assets.kuzureru,
            SE::Click => &assets.kettei,
            SE::Fire => &assets.suburi,
            SE::Steps => &assets.asphalt,
            SE::TurnOn => &assets.menu_open,
            SE::Warp => &assets.warp,
            SE::PickUp => &assets.cancel,
            SE::Heal => &assets.kaifuku1,
            SE::Switch => &assets.cursor2,
            SE::Drop => &assets.drop,
            SE::Growl => &assets.inoshishi,
            SE::Puyon => &assets.puyon,
            SE::Bicha => &assets.bicha,
            SE::Kawaii => &assets.kawaii,
            SE::Register => &assets.register,
        };

        play_se(&mut commands, &config, handle, position, camera_position);
    }
}

pub struct SECommandPlugin;

impl Plugin for SECommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SECommand>().add_systems(
            FixedUpdate,
            process_se_commands
                .run_if(resource_exists::<GameAssets>)
                .before(PhysicsSet::SyncBackend),
        );
    }
}
