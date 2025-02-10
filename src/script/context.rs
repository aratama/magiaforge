use super::api::apply_globals;
use super::api::console_log;
use super::api::register_globals;
use super::cmd::read_cmd_event;
use super::cmd::CmdEvent;
use super::loader::JavaScriptSource;
use crate::camera::GameCamera;
use crate::language::Dict;
use crate::script::cmd::Cmd;
use crate::set::FixedUpdateGameActiveSet;
use crate::states::GameState;
use crate::states::TimeState;
use crate::ui::speech_bubble::SpeechBubble;
use bevy::prelude::*;
use boa_engine::js_string;
use boa_engine::object::ObjectInitializer;
use boa_engine::property::PropertyKey;
use boa_engine::Context;
use boa_engine::JsValue;
use boa_engine::NativeFunction;
use boa_engine::Source;

#[derive(Default)]
pub struct JavaScriptContext {
    pub context: Context,
    generator: Option<JsValue>,
    value: Option<JsValue>,
    pub wait: u32,
}

impl JavaScriptContext {
    /// 指定した式でジェネレータ関数を開始します
    /// それまで実行されていたジェネレータ関数がある場合は破棄されます
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

    /// 現在実行中のジェネレータ関数を中断します
    pub fn abort(&mut self) {
        self.generator = None;
        self.value = None;
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
        ref mut wait,
    } = javascript_context.as_mut();

    if 0 < *wait {
        *wait -= 1;
        if *wait == 0 {
            *value = None;
        } else {
            return;
        }
    }

    if let Some(_value) = &value {
        // skip
        return;
    };

    let Some(generator_value) = generator else {
        return;
    };

    let Ok(generator_function) = generator_value.to_object(context) else {
        warn!("the return value is not a function {:?}", generator_value);
        return;
    };

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

    // if next_result_value.is_undefined() {
    //     info!(" value: undefined");
    // } else {
    //     info!(" value:{:?}", next_result_value.to_json(context));
    // }

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

fn read_asset_event(
    mut event_reader: EventReader<AssetEvent<JavaScriptSource>>,
    assets: Res<Assets<JavaScriptSource>>,
    mut javascript_context: NonSendMut<JavaScriptContext>,
) {
    for event in event_reader.read() {
        match event {
            AssetEvent::Added { id } => {
                let source = assets.get(*id).unwrap();
                *javascript_context = initialize_context(source);
                info!("JavaScript loaded: {:?}", source.path);
            }
            AssetEvent::Modified { id } => {
                let source = assets.get(*id).unwrap();
                *javascript_context = initialize_context(source);
                info!("JavaScript modified: {:?}", source.path);
            }
            _ => {}
        }
    }
}

fn initialize_context(source: &JavaScriptSource) -> JavaScriptContext {
    let mut context = Context::default();

    let log_object = ObjectInitializer::new(&mut context)
        .function(
            NativeFunction::from_fn_ptr(console_log),
            js_string!("log"),
            1,
        )
        .build();

    context
        .global_object()
        .set(
            PropertyKey::String("console".into()),
            JsValue::from(log_object),
            false,
            &mut context,
        )
        .expect("Failed to register global property");

    // スクリプト読み込み ///////////////////////////////////////////////////////////////////////

    let result = context.eval(Source::from_bytes(&source.source));
    if let Err(e) = result {
        error!("[javascript_loader] error: {:?}", e);
    }
    JavaScriptContext {
        context,
        ..default()
    }
}

// fn se(_caller: &JsValue, arguments: &[JsValue], context: &mut Context) -> Result<JsValue, JsError> {
//     Ok(JsValue::Undefined)
// }

pub struct JavaScriptContextPlugin;

impl Plugin for JavaScriptContextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(JavaScriptContext::default());
        app.add_event::<CmdEvent>();
        app.add_systems(
            Update,
            read_cmd_event.run_if(in_state(GameState::InGame).and(in_state(TimeState::Active))),
        );
        app.add_systems(Update, read_asset_event);
        app.add_systems(
            FixedUpdate,
            (register_globals, update, apply_globals)
                .chain()
                .in_set(FixedUpdateGameActiveSet),
        );
    }
}
