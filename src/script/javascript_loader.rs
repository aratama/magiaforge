use crate::{
    actor::Actor,
    camera::GameCamera,
    controller::player::Player,
    interpreter::{cmd::Cmd, interpreter::CmdEvent},
    language::Dict,
    ui::speech_bubble::SpeechBubble,
};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt},
    prelude::*,
};
use boa_engine::{
    js_string,
    object::{builtins::JsArray, ObjectInitializer},
    property::{Attribute, PropertyKey},
    Context, JsError, JsObject, JsValue, NativeFunction, Source,
};
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

                let array = JsArray::new(&mut context);
                context
                    .register_global_property(
                        PropertyKey::String("inventory".into()),
                        JsValue::from(array),
                        Attribute::READONLY,
                    )
                    .expect("Failed to register global property");

                register_console_log(&mut context);

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
    player_query: Query<&Actor, With<Player>>,
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
    } else if let Some(generator_value) = generator {
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

        register_console_log(context);

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

        let arguments = JsObject::default();
        arguments
            .set(
                PropertyKey::String("actor".into()),
                JsValue::String("foo".into()),
                false,
                context,
            )
            .expect("Failed to set arguments");

        let result =
            match next_function.call(&generator_value, &[JsValue::Object(arguments)], context) {
                Ok(result) => result,
                Err(err) => {
                    error!("[javascript_loader] error: {:?}", err);
                    *generator = None;
                    *value = None;
                    return;
                }
            };

        let result_object = result.to_object(context).unwrap();

        let done = result_object
            .get(PropertyKey::String("done".into()), context)
            .unwrap();

        let v = result_object
            .get(PropertyKey::String("value".into()), context)
            .unwrap();

        *value = Some(v.clone());

        if done == JsValue::Boolean(true) {
            *generator = None;

            if let Ok((mut speech, mut speech_visibility)) = speech_query.get_single_mut() {
                *speech_visibility = Visibility::Hidden;
                speech.dict = Dict::empty();
                speech.entity = None;
            }

            if let Ok(mut camera) = camera.get_single_mut() {
                camera.target = None;
            }
        }

        if v.is_object() {
            if let Ok(value) = v.to_json(context) {
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
}

fn register_console_log(context: &mut Context) {
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
}

pub struct JavaScriptLoaderPlugin;

impl Plugin for JavaScriptLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(JavaScriptContext::default());
        app.init_asset::<JavaScriptSource>();
        app.register_asset_loader(JavaScriptLoader);
        app.add_systems(Update, (read_asset_event, update).chain());
    }
}
