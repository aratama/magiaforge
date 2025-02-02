use crate::actor::Actor;
use crate::controller::player::Player;
use crate::hud::overlay::OverlayEvent;
use crate::ldtk::loader::LDTK;
use crate::level::world::GameLevel;
use crate::level::world::GameWorld;
use crate::player_state::PlayerState;
use crate::registry::Registry;
use crate::set::FixedUpdateInGameSet;
use crate::states::GameState;
use crate::states::TimeState;
use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use rand::seq::SliceRandom;

fn process_debug_command(
    registry: Registry,
    mut evr_kbd: EventReader<KeyboardInput>,
    mut local: Local<String>,
    ldtk_assets: Res<Assets<LDTK>>,
    mut level: ResMut<GameWorld>,
    mut writer: EventWriter<OverlayEvent>,
    mut player_query: Query<(&Player, &mut Actor, &Transform)>,
    mut in_game_time: ResMut<NextState<TimeState>>,
) {
    let ldtk = ldtk_assets.get(registry.assets.ldtk_level.id()).unwrap();

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

    let atmark_level_names = ldtk.get_levels().map(|s| format!("@{}", s.identifier));
    for atmark_level_name in atmark_level_names {
        if local.ends_with(&atmark_level_name) {
            level.next_level = GameLevel::new(atmark_level_name.strip_prefix("@").unwrap());
            info!("next_level {:?}", level.next_level);
            level.next_state = Some(PlayerState::from_query(
                &player_query.transmute_lens().query(),
            ));
            writer.send(OverlayEvent::Close(GameState::Warp));
            local.clear();
            return;
        }
    }

    if local.ends_with("@next") {
        let Ok((_, _, player_transform)) = player_query.get_single() else {
            return;
        };
        let position = player_transform.translation.truncate();
        let Some(current) = &level.find_chunk_by_position(position) else {
            return;
        };
        let props = registry.get_level(&current.level);
        let next = props.next.choose(&mut rand::thread_rng()).unwrap();
        level.next_level = next.clone();
        level.next_state = Some(PlayerState::from_query(
            &player_query.transmute_lens().query(),
        ));
        writer.send(OverlayEvent::Close(GameState::Warp));
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
        if let Ok((_, mut actor, _)) = player_query.get_single_mut() {
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
