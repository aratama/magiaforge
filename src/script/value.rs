use bevy::math::Vec2;
use boa_engine::js_string;
use boa_engine::object::ObjectInitializer;
use boa_engine::property::Attribute;
use boa_engine::Context;
use boa_engine::JsValue;

#[allow(dead_code)]
pub fn vec2_to_tuple(v: Vec2, context: &mut Context) -> JsValue {
    let actor_position = ObjectInitializer::new(context)
        .property(
            js_string!("x"),
            JsValue::Rational(v.x as f64),
            Attribute::READONLY,
        )
        .property(
            js_string!("y"),
            JsValue::Rational(v.y as f64),
            Attribute::READONLY,
        )
        .build();
    JsValue::from(actor_position)
}
