use crate::asset::GameAssets;
use crate::audio::play_se;
use crate::config::GameConfig;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use std::cmp::Ordering;

#[derive(Event, Clone, Copy)]
pub struct SEEvent {
    se: SE,
    position: Option<Vec2>,
}

impl SEEvent {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Deserialize)]
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
    Shuriken,
    Bus,
    Glass,
    Kaminari,
    Jishin,
    Bakuhatsu,
    Status2,
    Zombie,
    Kamae,
    Sen,
    Dragon,
    DragonFlutter,
    Taoreru,
    Freeze,
    Basha2,
    Suna,
    Chakuchi,
    Scene2,
    Kyushu2Short,
    Ken2,
}

/// 効果音イベントを順次再生していきます
/// ただし、同一のフレームに同じ効果音が複数回再生されると極端に音が大きくなり不自然なため、
/// 同じ効果音が同時に複数回リクエストされても、最も距離が近いもののみが再生されます
fn se_events(
    mut commands: Commands,
    assets: Res<GameAssets>,
    config: Res<GameConfig>,
    mut reader: EventReader<SEEvent>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    let camera_position = camera_query
        .get_single()
        .map_or(Vec2::ZERO, |c| c.translation.truncate());

    let mut vec = Vec::<SEEvent>::new();
    for event in reader.read() {
        vec.push(*event);
    }

    vec.sort_by(|a, b| {
        let la = (camera_position - a.position.unwrap_or(camera_position)).length_squared();
        let lb = (camera_position - b.position.unwrap_or(camera_position)).length_squared();
        la.partial_cmp(&lb).unwrap_or(Ordering::Equal)
    });

    let mut played = HashSet::<SE>::new();

    for SEEvent { se, position } in vec.iter() {
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
            SE::Shuriken => &assets.shuriken,
            SE::Bus => &assets.bus,
            SE::Glass => &assets.glass,
            SE::Kaminari => &assets.kaminari,
            SE::Jishin => &assets.jishin,
            SE::Bakuhatsu => &assets.bakuhatsu,
            SE::Status2 => &assets.status2,
            SE::Zombie => &assets.zombie,
            SE::Kamae => &assets.kamae,
            SE::Sen => &assets.sen,
            SE::Dragon => &assets.dragon,
            SE::DragonFlutter => &assets.dragon_flutter,
            SE::Taoreru => &assets.taoreru,
            SE::Freeze => &assets.freeze_se,
            SE::Basha2 => &assets.basha2,
            SE::Suna => &assets.suna,
            SE::Chakuchi => &assets.chakuchi,
            SE::Scene2 => &assets.scene2,
            SE::Kyushu2Short => &assets.kyushu2_short,
            SE::Ken2 => &assets.ken2,
        };

        play_se(&mut commands, &config, handle, position, camera_position);
    }
}

pub struct SECommandPlugin;

impl Plugin for SECommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SEEvent>();
        app.add_systems(Update, se_events.run_if(resource_exists::<GameAssets>));
    }
}
