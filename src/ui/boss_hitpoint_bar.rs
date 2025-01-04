use crate::asset::GameAssets;
use crate::component::life::Life;
use crate::enemy::huge_slime::Boss;
use crate::language::{Dict, M18NTtext};
use crate::states::GameState;
use bevy::prelude::*;

const BAR_WIDTH: f32 = 800.0;
const BAR_HEIGHT: f32 = 8.0;

#[derive(Component)]
pub struct BossHitpointBar;

#[derive(Component)]
pub struct StatusBarBackground;

#[derive(Component)]
pub struct StatusBarRect;

#[derive(Component)]
pub struct StatusBarBorder;

#[derive(Component)]
pub struct StatusBarText;

#[derive(Component)]
pub struct BossNameText;

pub fn spawn_boss_hitpoint_bar(parent: &mut ChildBuilder, assets: &Res<GameAssets>) {
    parent
        .spawn((
            Name::new("boss_hitpoint_bar"),
            BossHitpointBar,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0),
                left: Val::Px(240.0),
                width: Val::Px(BAR_WIDTH),
                height: Val::Px(BAR_HEIGHT),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                StatusBarBackground,
                BackgroundColor::from(Color::hsva(0., 0., 0.5, 0.1)),
                ZIndex(0),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    width: Val::Px(BAR_WIDTH),
                    height: Val::Px(BAR_HEIGHT),
                    ..default()
                },
            ));

            parent.spawn((
                StatusBarRect,
                BackgroundColor(Color::hsva(0.0, 1.0, 1.0, 0.1)),
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
                BorderColor(Color::hsla(0.0, 0.0, 0.5, 0.2)),
                ZIndex(2),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    width: Val::Px(BAR_WIDTH),
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
                    top: Val::Px(-8.0),
                    ..default()
                },
            ));

            parent.spawn((
                BossNameText,
                Text::new(""),
                M18NTtext(Dict::empty()),
                TextColor(Color::hsva(0.0, 0.0, 1.0, 0.5)),
                TextFont {
                    font: assets.noto_sans_jp.clone(),
                    font_size: 15.0,
                    ..default()
                },
                ZIndex(3),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(8.0),
                    top: Val::Px(-28.0),
                    ..default()
                },
            ));
        });
}

fn update_bar_visibility(
    mut bar_query: Query<&mut Visibility, With<BossHitpointBar>>,
    boss_query: Query<(&Boss, &Life)>,
    mut rect_query: Query<&mut Node, (With<StatusBarRect>, Without<StatusBarBackground>)>,
    mut text_query: Query<
        &mut Text,
        (
            With<StatusBarText>,
            Without<StatusBarRect>,
            Without<StatusBarBackground>,
        ),
    >,
    mut name_query: Query<
        &mut M18NTtext,
        (
            With<BossNameText>,
            Without<StatusBarText>,
            Without<StatusBarRect>,
            Without<StatusBarBackground>,
        ),
    >,
) {
    for mut visibility in bar_query.iter_mut() {
        if let Ok((boss, life)) = boss_query.get_single() {
            *visibility = Visibility::Inherited;

            for mut rect in rect_query.iter_mut() {
                rect.width = Val::Px(BAR_WIDTH * life.life as f32 / life.max_life.max(1) as f32);
            }

            for mut text in text_query.iter_mut() {
                text.0 = format!("{}", life.life);
            }

            for mut text in name_query.iter_mut() {
                text.0 = boss.name.clone();
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub struct BossHitpointBarPlugin;

impl Plugin for BossHitpointBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_bar_visibility.run_if(in_state(GameState::InGame)),
        );
    }
}
