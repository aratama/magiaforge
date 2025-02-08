use bevy::math::Vec2;
use boa_engine::{js_string, object::ObjectInitializer, property::Attribute, Context, JsValue};

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
