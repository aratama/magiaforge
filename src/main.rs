//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use wasm_bindgen::JsValue;
use web_sys::console;

fn main() {
    console::log_1(&JsValue::from("start"));

    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup);
    app.add_systems(Update, keyboard_input);
    app.run();
}

const X_EXTENT: f32 = 900.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }

    commands.spawn(
        TextBundle::from_section("Primitives Test", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        console::log_1(&JsValue::from("space was pressed"));
    }
    if keys.just_released(KeyCode::ControlLeft) {
        // Left Ctrl was released
        console::log_1(&JsValue::from("left ctrl was released"));
    }
    if keys.pressed(KeyCode::KeyW) {
        // W is being held down
        console::log_1(&JsValue::from("W is being held down"));
    }
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        // Either the left or right shift are being held down
        console::log_1(&JsValue::from(
            "Either the left or right shift are being held down",
        ));
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Backspace]) {
        // Either delete or backspace was just pressed
    }
}
