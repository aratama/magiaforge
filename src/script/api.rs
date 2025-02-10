use super::context::JavaScriptContext;
use crate::actor::Actor;
use crate::controller::player::Player;
use crate::ui::spell_list::SpellList;
use bevy::prelude::*;
use boa_engine::js_string;
use boa_engine::object::builtins::JsArray;
use boa_engine::property::PropertyKey;
use boa_engine::Context;
use boa_engine::JsError;
use boa_engine::JsValue;

pub fn register_globals(
    mut javascript_context: NonSendMut<JavaScriptContext>,
    player_query: Query<(&Actor, &Player)>,
    spell_list_query: Query<&SpellList>,
) {
    let JavaScriptContext { context, .. } = javascript_context.as_mut();

    let Ok((player_actor, player)) = player_query.get_single() else {
        warn!("Failed to get player");
        return;
    };

    // inventory ////////////////////////////////////////////////////////

    let inventory_jsarray = JsArray::from_iter(
        player_actor
            .inventory
            .as_string_array()
            .iter()
            .map(|str| JsValue::String(js_string!(str.clone()))),
        context,
    );

    context
        .global_object()
        .set(
            PropertyKey::String("inventory".into()),
            inventory_jsarray,
            false,
            context,
        )
        .expect("Failed to set inventory");

    // discovered_spells ////////////////////////////////////////////////////////

    let discovered_spells_jsarray = JsArray::from_iter(
        player
            .discovered_spells
            .iter()
            .map(|spell| JsValue::String(js_string!(spell.0.clone()))),
        context,
    );

    context
        .global_object()
        .set(
            PropertyKey::String("discoveries".into()),
            discovered_spells_jsarray,
            false,
            context,
        )
        .expect("Failed to set discoveries");

    // spellListOpen /////////////////////////////////////////////////////

    let spell_list = spell_list_query.single();

    context
        .global_object()
        .set(
            PropertyKey::String("spellListOpen".into()),
            JsValue::from(spell_list.open),
            false,
            context,
        )
        .expect("Failed to register global property");
}

pub fn apply_globals(
    mut javascript_context: NonSendMut<JavaScriptContext>,
    mut spell_list_query: Query<&mut SpellList>,
) {
    let JavaScriptContext {
        ref mut context, ..
    } = javascript_context.as_mut();

    let global = context.global_object();

    let open = global
        .get(PropertyKey::String("spellListOpen".into()), context)
        .unwrap_or_default()
        .as_boolean()
        .unwrap_or_default();

    let mut spell_list = spell_list_query.single_mut();
    spell_list.open = open;
}

pub fn console_log(
    _caller: &JsValue,
    arguments: &[JsValue],
    context: &mut Context,
) -> Result<JsValue, JsError> {
    for value in arguments {
        if value.is_undefined() {
            info!("[console.log] undefined");
        } else {
            let json = value.to_json(context);
            info!("[console.log] {:?}", json);
        }
    }
    Ok(JsValue::Undefined)
}
