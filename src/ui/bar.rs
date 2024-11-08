use crate::states::GameState;
use bevy::{ecs::system::EntityCommands, prelude::*};

const BAR_HEIGHT: f32 = 16.0;

#[derive(Component)]
pub struct StatusBar {
    pub value: i32,
    pub max_value: i32,
}

#[derive(Component)]
pub struct StatusBarBackground;

#[derive(Component)]
pub struct StatusBarRect;

#[derive(Component)]
pub struct StatusBarBorder;

#[derive(Component)]
pub struct StatusBarText;

pub fn spawn_status_bar<T: Component>(
    parent: &mut ChildBuilder,
    marker: T,
    value: i32,
    max_value: i32,
    color: Color,
) {
    parent
        .spawn((
            marker,
            StatusBar { value, max_value },
            NodeBundle {
                style: Style {
                    height: Val::Px(BAR_HEIGHT),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                StatusBarBackground,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        left: Val::Px(0.),
                        height: Val::Px(BAR_HEIGHT),
                        ..default()
                    },
                    background_color: Color::srgba(0., 0., 0., 0.5).into(),
                    z_index: ZIndex::Local(0),
                    ..default()
                },
            ));

            parent.spawn((
                StatusBarRect,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        left: Val::Px(0.),
                        height: Val::Px(BAR_HEIGHT),
                        ..default()
                    },
                    background_color: color.into(),
                    z_index: ZIndex::Local(1),
                    ..default()
                },
            ));

            parent.spawn((
                StatusBarBorder,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        left: Val::Px(0.),
                        height: Val::Px(BAR_HEIGHT),
                        border: UiRect::all(Val::Px(1.)),
                        ..default()
                    },
                    border_color: Color::WHITE.into(),
                    z_index: ZIndex::Local(2),
                    ..default()
                },
            ));

            parent.spawn((
                StatusBarText,
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            color: Color::hsla(0.0, 0.0, 1.0, 0.5),
                            font_size: 18.0,
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(8.0),
                        top: Val::Px(-1.0),
                        height: Val::Px(BAR_HEIGHT),
                        max_width: Val::Px(500.0),
                        ..default()
                    },
                    z_index: ZIndex::Local(3),
                    ..default()
                },
            ));
        });
}

fn update_status_bar(
    status_bar_query: Query<&StatusBar>,
    mut background_query: Query<(&Parent, &mut Style), With<StatusBarBackground>>,
    mut rect_query: Query<
        (&Parent, &mut Style),
        (With<StatusBarRect>, Without<StatusBarBackground>),
    >,
    mut border_query: Query<
        (&Parent, &mut Style),
        (
            With<StatusBarBorder>,
            Without<StatusBarRect>,
            Without<StatusBarBackground>,
        ),
    >,
    mut text_query: Query<(&Parent, &mut Text), With<StatusBarText>>,
) {
    for (parent, mut rect) in background_query.iter_mut() {
        let bar = status_bar_query.get(parent.get()).unwrap();
        rect.width = Val::Px(bar.max_value as f32);
    }
    for (parent, mut rect) in rect_query.iter_mut() {
        let bar = status_bar_query.get(parent.get()).unwrap();
        rect.width = Val::Px(bar.value as f32);
    }
    for (parent, mut border) in border_query.iter_mut() {
        let bar = status_bar_query.get(parent.get()).unwrap();
        border.width = Val::Px(bar.max_value as f32);
    }
    for (parent, mut text) in text_query.iter_mut() {
        let bar = status_bar_query.get(parent.get()).unwrap();
        text.sections[0].value = format!("{} / {}", bar.value, bar.max_value);
    }
}

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_status_bar.run_if(in_state(GameState::InGame)),
        );
    }
}
