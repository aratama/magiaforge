use crate::{
    asset::GameAssets, audio::NextBGM, entity::life::Life, hud::overlay::OverlayEvent,
    states::GameState,
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseUiAnimation;
use bevy_rapier2d::plugin::PhysicsSet;

#[derive(Component)]
pub struct DespawnAndGoEnding;

#[derive(Component)]
pub struct EndingImage;

fn despown(
    mut commands: Commands,
    query: Query<(Entity, &Life), With<DespawnAndGoEnding>>,
    mut writer: EventWriter<OverlayEvent>,
) {
    for (entity, life) in query.iter() {
        if life.life <= 0 {
            commands.entity(entity).despawn_recursive();
            writer.send(OverlayEvent::Close(GameState::Ending));
        }
    }
}

fn setup(mut commands: Commands, assets: Res<GameAssets>, mut next_bgm: ResMut<NextBGM>) {
    next_bgm.0 = Some(assets.ending_bgm.clone());

    commands.spawn((
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

pub struct EndingPlugin;

impl Plugin for EndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interaction.run_if(in_state(GameState::Ending)));
        app.add_systems(FixedUpdate, despown.before(PhysicsSet::SyncBackend));
        app.add_systems(OnEnter(GameState::Ending), setup);
    }
}
