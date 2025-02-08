use crate::actor::Actor;
use crate::camera::GameCamera;
use crate::controller::player::Player;
use crate::language::Dict;
use crate::script::cmd::Cmd;
use crate::script::event::CmdEvent;
use crate::set::FixedUpdateGameActiveSet;
use crate::ui::speech_bubble::SpeechBubble;
use crate::ui::spell_list::SpellList;
use bevy::asset::io::Reader;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::prelude::*;
use boa_engine::js_string;
use boa_engine::object::builtins::JsArray;
use boa_engine::object::ObjectInitializer;
use boa_engine::property::PropertyKey;
use boa_engine::Context;
use boa_engine::JsError;
use boa_engine::JsValue;
use boa_engine::NativeFunction;
use boa_engine::Source;
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct JavaScriptSource {
    pub path: String,
    pub source: String,
}

#[derive(Default)]
pub struct JavaScriptContext {
    context: Context,
    generator: Option<JsValue>,
    value: Option<JsValue>,
}

impl JavaScriptContext {
    /// 指定した式でジェネレータ関数を開始します
    pub fn generate(&mut self, calling: String) {
        let generator = self.context.eval(Source::from_bytes(&calling));
        if let Ok(generator) = generator {
            self.generator = Some(generator);
            self.value = None;
        } else {
            self.generator = None;
            self.value = None;
        }
    }

    /// 現在中断しているジェネレータ関数の実行を強制的に再開します
    pub fn resume(&mut self) {
        info!("resume");
        self.value = None;
    }

    pub fn abort(&mut self) {
        self.generator = None;
        self.value = None;
    }
}

#[derive(Default)]
pub struct JavaScriptLoader;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Could not read asset")]
    ReadError,
}

impl AssetLoader for JavaScriptLoader {
    type Asset = JavaScriptSource;
    type Settings = ();
    type Error = ReadError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut source = "".to_string();
        if let Err(err) = reader.read_to_string(&mut source).await {
            error!("[javascript_loader] error: {:?}", err);
            return Err(ReadError::ReadError);
        }

        Ok(JavaScriptSource {
            source,
            path: load_context
                .path()
                .as_os_str()
                .to_string_lossy()
                .to_string(),
        })
    }
    fn extensions(&self) -> &[&str] {
        &["js"]
    }
}

fn log(
    _caller: &JsValue,
    arguments: &[JsValue],
    context: &mut Context,
) -> Result<JsValue, JsError> {
    for value in arguments {
        if value.is_undefined() {
            info!("undefined");
        } else {
            let json = value.to_json(context);
            info!("{:?}", json);
        }
    }
    Ok(JsValue::Undefined)
}

fn read_asset_event(
    mut event_reader: EventReader<AssetEvent<JavaScriptSource>>,
    assets: Res<Assets<JavaScriptSource>>,
    mut javascript_context: NonSendMut<JavaScriptContext>,
) {
    for event in event_reader.read() {
        match event {
            AssetEvent::Added { id } => {
                let source = assets.get(*id).unwrap();
                let mut context = Context::default();
                let result = context.eval(Source::from_bytes(&source.source));
                if let Err(e) = result {
                    error!("[javascript_loader] error: {:?}", e);
                    return;
                }
                *javascript_context = JavaScriptContext {
                    context,
                    ..default()
                };
                info!("JavaScript loaded: {:?}", source.path);
            }
            AssetEvent::Modified { id } => {
                let source = assets.get(*id).unwrap();
                let mut context = Context::default();
                let result = context.eval(Source::from_bytes(&source.source));
                if let Err(e) = result {
                    error!("[javascript_loader] error: {:?}", e);
                    return;
                }
                *javascript_context = JavaScriptContext {
                    context,
                    ..default()
                };
                info!("JavaScript modified: {:?}", source.path);
            }
            _ => {}
        }
    }
}

/// 現在停止中のジェネレータ関数を再開し、値を更新します
/// ただし、すでに生成された値が存在する場合は何もしません
/// 値に応じた再開条件を満たされるとvalueをNoneに設定され、実行が再開されます
fn update(
    mut javascript_context: NonSendMut<JavaScriptContext>,
    mut cmd_writer: EventWriter<CmdEvent>,
    mut speech_query: Query<(&mut SpeechBubble, &mut Visibility)>,
    mut camera: Query<&mut GameCamera>,
) {
    let JavaScriptContext {
        context,
        ref mut value,
        ref mut generator,
    } = javascript_context.as_mut();

    if let Some(_value) = &value {
        // skip
        return;
    };

    let Some(generator_value) = generator else {
        return;
    };

    info!("calling next...");

    let generator_function = generator_value.to_object(context).unwrap();

    let next_function_value: JsValue = generator_function
        .get(PropertyKey::String("next".into()), context)
        .unwrap();

    let next_function = match next_function_value.to_object(context) {
        Ok(next_function) => next_function,
        Err(err) => {
            error!("[javascript_loader] error: {:?}", err);
            *generator = None;
            *value = None;
            return;
        }
    };

    let next_result = match next_function.call(&generator_value, &[], context) {
        Ok(result) => result,
        Err(err) => {
            error!("[javascript_loader] error: {:?}", err);
            *generator = None;
            *value = None;
            return;
        }
    };

    let result_object = next_result.to_object(context).unwrap();

    let next_result_done = result_object
        .get(PropertyKey::String("done".into()), context)
        .unwrap();

    let next_result_value = result_object
        .get(PropertyKey::String("value".into()), context)
        .unwrap();

    info!("done:{:?}", next_result_done.to_json(context),);
    if next_result_value.is_undefined() {
        info!(" value: undefined");
    } else {
        info!(" value:{:?}", next_result_value.to_json(context));
    }

    *value = Some(next_result_value.clone());

    if next_result_done == JsValue::Boolean(true) {
        *generator = None;

        if let Ok((mut speech, mut speech_visibility)) = speech_query.get_single_mut() {
            *speech_visibility = Visibility::Hidden;
            speech.dict = Dict::empty();
            speech.entity = None;
        }

        if let Ok(mut camera) = camera.get_single_mut() {
            camera.target = None;
        }

        info!("generator done");
    }

    if next_result_value.is_object() {
        if let Ok(value) = next_result_value.to_json(context) {
            match serde_json::from_value::<Cmd>(value) {
                Ok(cmd) => {
                    cmd_writer.send(CmdEvent(cmd));
                }
                Err(err) => {
                    error!("[javascript_loader] error: {:?}", err);
                }
            }
        }
    }
}

fn register_globals(
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

    let log_object = ObjectInitializer::new(context)
        .function(NativeFunction::from_fn_ptr(log), js_string!("log"), 1)
        .build();

    context
        .global_object()
        .set(
            PropertyKey::String("console".into()),
            JsValue::from(log_object),
            false,
            context,
        )
        .expect("Failed to register global property");

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

fn apply_globals(
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

pub struct JavaScriptLoaderPlugin;

impl Plugin for JavaScriptLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(JavaScriptContext::default());
        app.init_asset::<JavaScriptSource>();
        app.register_asset_loader(JavaScriptLoader);
        app.add_systems(Update, read_asset_event);
        app.add_systems(
            FixedUpdate,
            (register_globals, update, apply_globals)
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
