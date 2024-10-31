use crate::{
    constant::*, controller::player::Player, hud::overlay::OverlayNextState, states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_light_2d::light::{PointLight2d, PointLight2dBundle};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, CollisionGroups, Sensor};

#[derive(Default, Component)]
pub struct MagicCircle;

pub fn spawn_magic_circle(commands: &mut Commands, aseprite: Handle<Aseprite>, x: f32, y: f32) {
    commands.spawn((
        Name::new("magic_circle"),
        StateScoped(GameState::InGame),
        MagicCircle,
        AsepriteSliceBundle {
            aseprite: aseprite,
            slice: "magic_circle".into(),
            transform: Transform::from_translation(Vec3::new(x, y, PAINT_LAYER_Z)),
            ..default()
        },
        Collider::cuboid(TILE_HALF, TILE_HALF),
        Sensor,
        // RigidBody::Fixed,
        CollisionGroups::new(WALL_GROUP, ENEMY_GROUP),
        ActiveEvents::COLLISION_EVENTS,
    ));

    // 光源をスプライトの子にすると、画面外に出た時に光が消えてしまうことに注意
    commands.spawn((
        Name::new("magic_circle_light"),
        StateScoped(GameState::InGame),
        PointLight2dBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            point_light: PointLight2d {
                color: Color::hsla(240.0, 1.0, 0.5, 1.0),
                radius: 100.0,
                intensity: 1.0,
                falloff: 10.0,
                ..default()
            },
            ..default()
        },
    ));
}

fn jump_on_touch(
    player_query: Query<&Player>,
    circle_query: Query<&MagicCircle>,
    mut events: EventReader<CollisionEvent>,
    mut overlay_next_state: ResMut<OverlayNextState>,
) {
    for event in events.read() {
        match event {
            CollisionEvent::Started(a, b, flags) => {
                if process_touch_event(a, b, &player_query, &circle_query)
                    || process_touch_event(b, a, &player_query, &circle_query)
                {
                    *overlay_next_state = OverlayNextState(Some(GameState::MainMenu));
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn process_touch_event(
    a: &Entity,
    b: &Entity,
    players: &Query<&Player>,
    circle_query: &Query<&MagicCircle>,
) -> bool {
    players.contains(*a) && circle_query.contains(*b)
}

pub struct MagicCirclePlugin;

impl Plugin for MagicCirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            jump_on_touch.run_if(in_state(GameState::InGame)),
        );
    }
}
