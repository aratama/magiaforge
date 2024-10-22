pub mod life_bar;
pub mod overlay;
pub mod pointer;

use super::constant::HUD_Z_INDEX;
use super::entity::player::*;
use super::states::GameState;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use iyes_perf_ui::entries::PerfUiBundle;

const PLAYER_LIFE_BAR_WIDTH: f32 = 200.0;
const PLAYER_LIFE_BAR_HEIGHT: f32 = 20.0;
const PLAYER_LIFE_BAR_LEFT: f32 = 12.0;
const PLAYER_LIFE_BAR_TOP: f32 = 12.0;

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct PlayerLifeBar;

#[derive(Component)]
pub struct PlayerDamageBar;

#[derive(Component)]
pub struct PlayerLifeText;

fn setup_hud(mut commands: Commands) {
    let mut root = commands.spawn((
        StateScoped(GameState::InGame),
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,

                ..default()
            },
            z_index: ZIndex::Global(HUD_Z_INDEX),
            ..default()
        },
    ));

    root.with_children(|parent| {
        parent.spawn((
            PlayerLifeBar,
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(PLAYER_LIFE_BAR_TOP),
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT),
                    width: Val::Px(PLAYER_LIFE_BAR_WIDTH),
                    height: Val::Px(PLAYER_LIFE_BAR_HEIGHT),
                    ..default()
                },
                background_color: Color::srgba(0., 0.7, 0., 0.9).into(),
                ..default()
            },
        ));
        parent.spawn((
            PlayerDamageBar,
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(PLAYER_LIFE_BAR_TOP),
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT),
                    width: Val::Px(0.0),
                    height: Val::Px(PLAYER_LIFE_BAR_HEIGHT),
                    ..default()
                },
                background_color: Color::srgba(0.7, 0., 0., 0.9).into(),
                ..default()
            },
        ));
        parent.spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(PLAYER_LIFE_BAR_TOP),
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT),
                    width: Val::Px(PLAYER_LIFE_BAR_WIDTH),
                    height: Val::Px(PLAYER_LIFE_BAR_HEIGHT),
                    border: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                background_color: Color::srgba(0., 0., 0., 0.5).into(),
                border_color: Color::WHITE.into(),
                ..default()
            },
        ));
        parent.spawn((
            PlayerLifeText,
            Name::new("hitpoint"),
            StateScoped(GameState::InGame),
            TextBundle {
                text: Text::from_section("", TextStyle::default()),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT + 8.0),
                    top: Val::Px(PLAYER_LIFE_BAR_TOP - 2.0),
                    ..default()
                },
                ..default()
            },
            HUD,
        ));
    });

    #[cfg(feature = "debug")]
    commands.spawn(PerfUiBundle::default());
}

fn update_hud(
    player_query: Query<&Player, Without<Camera2d>>,
    mut player_life_bar_query: Query<&mut Style, With<PlayerLifeBar>>,
    mut player_damage_bar_query: Query<&mut Style, (With<PlayerDamageBar>, Without<PlayerLifeBar>)>,
    mut player_life_query: Query<&mut Text, With<PlayerLifeText>>,
) {
    if let Ok(player) = player_query.get_single() {
        let mut player_life = player_life_query.single_mut();
        let mut player_life_bar = player_life_bar_query.single_mut();
        let mut player_damage_bar = player_damage_bar_query.single_mut();

        player_life.sections[0].value = format!("{} / {}", player.life, player.max_life);

        let life_bar_width = (player.life as f32 / player.max_life as f32) * PLAYER_LIFE_BAR_WIDTH;
        let damage_bar_width =
            (player.latest_damage as f32 / player.max_life as f32) * PLAYER_LIFE_BAR_WIDTH;

        player_life_bar.width = Val::Px(life_bar_width);
        player_damage_bar.width = Val::Px(damage_bar_width);
        player_damage_bar.left = Val::Px(PLAYER_LIFE_BAR_LEFT + life_bar_width);
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        app.add_systems(Update, update_hud.run_if(in_state(GameState::InGame)));
    }
}
