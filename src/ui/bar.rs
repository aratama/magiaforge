use crate::asset::GameAssets;
use crate::states::GameState;
use bevy::prelude::*;

const BAR_HEIGHT: f32 = 19.0;

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
    assets: &Res<GameAssets>,
    marker: T,
    value: i32,
    max_value: i32,
    color: Color,
) {
    parent
        .spawn((
            Name::new("status_bar"),
            marker,
            StatusBar { value, max_value },
            Node {
                height: Val::Px(BAR_HEIGHT),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                StatusBarBackground,
                BackgroundColor::from(Color::srgba(0., 0., 0., 0.5)),
                ZIndex(0),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    height: Val::Px(BAR_HEIGHT),
                    ..default()
                },
            ));

            parent.spawn((
                StatusBarRect,
                BackgroundColor(color),
                ZIndex(1),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    height: Val::Px(BAR_HEIGHT),
                    ..default()
                },
            ));

            parent.spawn((
                StatusBarBorder,
                BorderColor(Color::WHITE),
                ZIndex(2),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    height: Val::Px(BAR_HEIGHT),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
            ));

            parent.spawn((
                StatusBarText,
                Text::new(""),
                TextColor(Color::WHITE),
                TextFont {
                    font: assets.noto_sans_jp.clone(),
                    font_size: 15.0,
                    ..default()
                },
                ZIndex(3),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(8.0),
                    top: Val::Px(0.0),
                    ..default()
                },
            ));
        });
}

fn update_status_bar(
    status_bar_query: Query<&StatusBar>,
    mut background_query: Query<(&Parent, &mut Node), With<StatusBarBackground>>,
    mut rect_query: Query<
        (&Parent, &mut Node),
        (With<StatusBarRect>, Without<StatusBarBackground>),
    >,
    mut border_query: Query<
        (&Parent, &mut Node),
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
        text.0 = format!("{}", bar.value);
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
