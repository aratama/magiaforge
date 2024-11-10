use crate::command::GameCommand;
use crate::constant::HUD_Z_INDEX;
use crate::ui::on_press::OnPress;
use crate::{
    asset::GameAssets,
    states::{GameState, MainMenuPhase},
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AsepriteSliceUiBundle;
use git_version::git_version;

#[derive(Event, PartialEq, Eq, Debug, Clone, Copy)]
enum Events {
    Start,
}

fn setup_main_menu(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut writer: EventWriter<GameCommand>,
) {
    writer.send(GameCommand::BGMBoubaku);

    commands.spawn((
        StateScoped(GameState::MainMenu),
        ImageBundle {
            z_index: ZIndex::Global(-1000),
            style: Style {
                width: Val::Px(1280.0),
                height: Val::Px(720.0),
                ..default()
            },
            ..default()
        },
        AsepriteSliceUiBundle {
            slice: "all".into(),
            aseprite: assets.title.clone(),
            ..default()
        },
    ));

    commands.spawn((
        StateScoped(GameState::MainMenu),
        Name::new("Git Version"),
        TextBundle {
            text: Text::from_section(
                format!("Version: {}", git_version!()),
                TextStyle {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                    font_size: 12.0,
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(700.0),
                ..default()
            },
            z_index: ZIndex::Global(HUD_Z_INDEX),

            ..default()
        },
    ));
}

fn on_click(buttons: Res<ButtonInput<MouseButton>>, mut writer: EventWriter<Events>) {
    if buttons.any_just_pressed(vec![MouseButton::Left, MouseButton::Right]) {
        writer.send(Events::Start);
    }
}

fn read_events(
    mut query: Query<&mut Visibility, With<OnPress>>,
    mut menu_next_state: ResMut<NextState<MainMenuPhase>>,
    mut writer: EventWriter<GameCommand>,
    mut reader: EventReader<Events>,
) {
    for event in reader.read() {
        match event {
            Events::Start => {
                for mut visibility in &mut query {
                    *visibility = Visibility::Hidden;
                }
                menu_next_state.set(MainMenuPhase::Paused);
                writer.send(GameCommand::SEKettei(None));
                writer.send(GameCommand::StateInGame);
                writer.send(GameCommand::BGMNone);
            }
        }
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Events>();
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
        app.add_systems(
            Update,
            (on_click, read_events).run_if(in_state(GameState::MainMenu)),
        );
    }
}
