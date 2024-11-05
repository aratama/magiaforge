pub mod life_bar;
pub mod overlay;
pub mod pointer;

use super::constant::HUD_Z_INDEX;
use super::controller::player::Player;
use super::entity::actor::Actor;
use super::states::GameState;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use iyes_perf_ui::entries::PerfUiBundle;

const PLAYER_LIFE_BAR_WIDTH: f32 = 200.0;
const PLAYER_LIFE_BAR_HEIGHT: f32 = 20.0;
const PLAYER_LIFE_BAR_LEFT: f32 = 12.0;
const PLAYER_LIFE_BAR_TOP: f32 = 12.0;
const PLAYER_MANA_BAR_TOP: f32 = PLAYER_LIFE_BAR_TOP + 24.0;

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct PlayerLifeBar;

#[derive(Component)]
pub struct PlayerLifeText;

#[derive(Component)]
pub struct PlayerManaBar;

#[derive(Component)]
pub struct PlayerManaText;

#[derive(Component)]
pub struct PlayerGold;

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
                z_index: ZIndex::Global(HUD_Z_INDEX),
                ..default()
            },
        ));

        parent.spawn((NodeBundle {
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
            z_index: ZIndex::Global(HUD_Z_INDEX),
            ..default()
        },));

        parent.spawn((
            PlayerLifeText,
            Name::new("hitpoint"),
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.35),
                        ..default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT + 8.0),
                    top: Val::Px(PLAYER_LIFE_BAR_TOP - 2.0),
                    ..default()
                },
                z_index: ZIndex::Global(HUD_Z_INDEX),
                ..default()
            },
            HUD,
        ));

        parent.spawn((
            PlayerManaBar,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(PLAYER_MANA_BAR_TOP),
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT),
                    width: Val::Px(PLAYER_LIFE_BAR_WIDTH),
                    height: Val::Px(PLAYER_LIFE_BAR_HEIGHT),
                    ..default()
                },
                background_color: Color::srgba(0., 0., 0.7, 0.9).into(),
                z_index: ZIndex::Global(HUD_Z_INDEX),
                ..default()
            },
        ));

        parent.spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(PLAYER_MANA_BAR_TOP),
                left: Val::Px(PLAYER_LIFE_BAR_LEFT),
                width: Val::Px(PLAYER_LIFE_BAR_WIDTH),
                height: Val::Px(PLAYER_LIFE_BAR_HEIGHT),
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            background_color: Color::srgba(0., 0., 0., 0.5).into(),
            border_color: Color::WHITE.into(),
            z_index: ZIndex::Global(HUD_Z_INDEX),
            ..default()
        },));

        parent.spawn((
            PlayerManaText,
            Name::new("hitpoint"),
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.35),
                        ..default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT + 8.0),
                    top: Val::Px(PLAYER_MANA_BAR_TOP - 2.0),

                    ..default()
                },
                z_index: ZIndex::Global(HUD_Z_INDEX),
                ..default()
            },
            HUD,
        ));

        parent.spawn((
            PlayerGold,
            Name::new("golds"),
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        color: Color::hsla(0.0, 0.0, 1.0, 0.35),
                        ..default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(PLAYER_LIFE_BAR_LEFT + 8.0),
                    top: Val::Px(PLAYER_LIFE_BAR_TOP + 24.0 + 24.0),
                    ..default()
                },
                z_index: ZIndex::Global(HUD_Z_INDEX),
                ..default()
            },
            HUD,
        ));
    });

    #[cfg(feature = "debug")]
    commands.spawn(PerfUiBundle::default());
}

fn update_hud(
    player_query: Query<(&Player, &Actor), Without<Camera2d>>,
    mut player_life_text_query: Query<&mut Text, With<PlayerLifeText>>,
    mut player_mana_text_query: Query<&mut Text, (With<PlayerManaText>, Without<PlayerLifeText>)>,
    mut player_life_bar_query: Query<&mut Style, With<PlayerLifeBar>>,
    mut player_mana_bar_query: Query<&mut Style, (With<PlayerManaBar>, Without<PlayerLifeBar>)>,

    mut player_gold_query: Query<
        &mut Text,
        (
            With<PlayerGold>,
            Without<PlayerLifeText>,
            Without<PlayerManaText>,
        ),
    >,
) {
    if let Ok((player, actor)) = player_query.get_single() {
        let mut player_life_text = player_life_text_query.single_mut();
        let mut player_mana_text = player_mana_text_query.single_mut();
        let mut player_life_bar = player_life_bar_query.single_mut();
        let mut player_mana_bar = player_mana_bar_query.single_mut();
        let mut player_gold = player_gold_query.single_mut();

        player_life_text.sections[0].value = format!("{} / {}", actor.life, actor.max_life);

        player_mana_text.sections[0].value =
            format!("{} / {}", actor.mana / 10, actor.max_mana / 10);

        let life_bar_width = (actor.life as f32 / actor.max_life as f32) * PLAYER_LIFE_BAR_WIDTH;

        let player_mana_width = (actor.mana as f32 / actor.max_mana as f32) * PLAYER_LIFE_BAR_WIDTH;

        player_life_bar.width = Val::Px(life_bar_width);
        player_mana_bar.width = Val::Px(player_mana_width);

        player_gold.sections[0].value = format!("{} GOLDS", player.golds);
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        app.add_systems(Update, update_hud.run_if(in_state(GameState::InGame)));
    }
}
