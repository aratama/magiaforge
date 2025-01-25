use crate::actor::Actor;
use crate::asset::GameAssets;
use crate::controller::player::Player;
use crate::hud::overlay::OverlayEvent;
use crate::page::in_game::GameLevel;
use crate::page::in_game::LevelSetup;
use crate::player_state::PlayerState;
use crate::registry::Registry;
use crate::set::FixedUpdateInGameSet;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;

fn process_debug_command(
    registry: Registry,
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
    assets: Res<GameAssets>,
    aseprites: Res<Assets<Aseprite>>,
    mut level: ResMut<LevelSetup>,
    mut writer: EventWriter<OverlayEvent>,
    mut player_query: Query<(&Player, &mut Actor)>,
    mut in_game_time: ResMut<NextState<TimeState>>,
) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match ev.logical_key {
            Key::Character(ref c) => {
                local.push_str(c);
            }
            _ => {}
        }
    }

    let Some(level_map) = aseprites.get(&assets.level) else {
        return;
    };
    let atmark_level_names = level_map.slices.iter().map(|(s, _)| format!("@{}", s));
    for atmark_level_name in atmark_level_names {
        if local.ends_with(&atmark_level_name) {
            level.next_level = GameLevel::new(atmark_level_name.strip_prefix("@").unwrap());
            level.next_state = Some(PlayerState::from_query(
                &player_query.transmute_lens().query(),
            ));
            writer.send(OverlayEvent::Close(GameState::Warp));
            local.clear();
            return;
        }
    }

    if local.ends_with("@next") {
        in_game_time.set(TimeState::Inactive);
        local.clear();
    } else if local.ends_with("@pause") {
        in_game_time.set(TimeState::Inactive);
        local.clear();
    } else if local.ends_with("@resume") {
        in_game_time.set(TimeState::Active);
        local.clear();
    } else if local.ends_with("@ending") {
        writer.send(OverlayEvent::Close(GameState::Ending));
        local.clear();
    } else if local.ends_with("@item") {
        if let Ok((_, mut actor)) = player_query.get_single_mut() {
            for spell in registry.game().debug_items.iter() {
                actor.inventory.insert_spell(spell.clone());
            }
        }
        local.clear();
    }
}

pub struct DebugCommandPlugin;

impl Plugin for DebugCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            process_debug_command.in_set(FixedUpdateInGameSet),
        );
    }
}
