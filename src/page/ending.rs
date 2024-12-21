use crate::{
    asset::GameAssets,
    audio::NextBGM,
    constant::LAST_BOSS_LEVEL,
    enemy::huge_slime::HugeSlime,
    hud::overlay::OverlayEvent,
    level::{CurrentLevel, GameLevel},
    states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseUiAnimation;

#[derive(Component)]
pub struct EndingImage;

fn setup(mut commands: Commands, assets: Res<GameAssets>, mut next_bgm: ResMut<NextBGM>) {
    next_bgm.0 = Some(assets.ending_bgm.clone());

    commands.spawn((
        Name::new("ending"),
        EndingImage,
        StateScoped(GameState::Ending),
        AseUiAnimation {
            aseprite: assets.ending.clone(),
            animation: "ending".into(),
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            ..default()
        },
        Interaction::default(),
    ));
}

fn interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<EndingImage>)>,
    mut writer: EventWriter<OverlayEvent>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                writer.send(OverlayEvent::Close(GameState::MainMenu));
            }
            _ => {}
        }
    }
}

fn start_ending(
    mut local: Local<u32>,
    boss_query: Query<&HugeSlime>,
    mut writer: EventWriter<OverlayEvent>,
    current: Res<CurrentLevel>,
) {
    if current.level == Some(GameLevel::Level(LAST_BOSS_LEVEL)) && boss_query.is_empty() {
        *local += 1;
        if *local == 120 {
            writer.send(OverlayEvent::Close(GameState::Ending));
        }
    }
}

pub struct EndingPlugin;

impl Plugin for EndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_ending.run_if(in_state(GameState::InGame)));
        app.add_systems(Update, interaction.run_if(in_state(GameState::Ending)));
        app.add_systems(OnEnter(GameState::Ending), setup);
    }
}
