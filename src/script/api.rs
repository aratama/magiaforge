use super::context::JavaScriptContext;
use crate::actor::Actor;
use crate::controller::player::Player;
use crate::ui::spell_list::SpellList;
use bevy::prelude::*;
use boa_engine::js_string;
use boa_engine::object::builtins::JsArray;
use boa_engine::object::ObjectInitializer;
use boa_engine::property::PropertyKey;
use boa_engine::Context;
use boa_engine::JsError;
use boa_engine::JsValue;
use boa_engine::NativeFunction;

// # API
//
// yield で発行するコマンドについては、cmd.rs の Cmd を参照してください。
// 以下はスクリプトのグローバルスコープに公開されるもので、
// デフォルトでは読み取り専用のです。
//
// inventory: string[]
//     プレイヤーのインベントリ。呪文名が文字列の配列で格納されます。
//
// actorPosition: { x: number, y: number }
//     スクリプト起動のトリガーとなったアクターの位置。
//
// console.log(...args: any[]): void
//     ログを出力します
//
// spellListOpen: boolean
//    呪文リストが開いているかどうかを設定します
//    このプロパティには書き込みが可能です
//
pub fn register_globals(
    mut javascript_context: NonSendMut<JavaScriptContext>,
    player_query: Query<&Actor, With<Player>>,
    spell_list_query: Query<&SpellList>,
) {
    let JavaScriptContext { context, .. } = javascript_context.as_mut();

    let inventory: Vec<String> = player_query
        .get_single()
        .map(|actor| actor.inventory.as_string_array())
        .unwrap_or_default();

    let inventory_jsarray = JsArray::from_iter(
        inventory
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
