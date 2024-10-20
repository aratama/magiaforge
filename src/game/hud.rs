use super::player::*;
use super::states::GameState;
use bevy::prelude::*;

const PLAYER_LIFE_BAR_WIDTH: f32 = 200.0;
const PLAYER_LIFE_BAR_HEIGHT: f32 = 20.0;

#[cfg(feature = "debug")]
use iyes_perf_ui::entries::PerfUiBundle;

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct PlayerLifeBar;

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
                    top: Val::Px(12.0),
                    left: Val::Px(12.0),
                    width: Val::Px(PLAYER_LIFE_BAR_WIDTH),
                    height: Val::Px(PLAYER_LIFE_BAR_HEIGHT),
                    ..default()
                },
                background_color: Color::srgba(0., 0.7, 0., 0.9).into(),
                ..default()
            },
        ));
        parent.spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.0),
                    left: Val::Px(12.0),
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
                    left: Val::Px(20.0),
                    top: Val::Px(10.0),
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
    mut player_life_query: Query<&mut Text, With<PlayerLifeText>>,
) {
    if let Ok(player) = player_query.get_single() {
        let mut player_life = player_life_query.single_mut();
        player_life.sections[0].value = format!("{} / {}", player.life, player.max_life);
        let mut player_life_bar = player_life_bar_query.single_mut();
        player_life_bar.width =
            Val::Px((player.life as f32 / player.max_life as f32) * PLAYER_LIFE_BAR_WIDTH);
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        app.add_systems(Update, update_hud.run_if(in_state(GameState::InGame)));
    }
}
