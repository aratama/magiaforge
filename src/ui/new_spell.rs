use crate::actor::Actor;
use crate::constant::UI_PRIMARY;
use crate::constant::UI_PRIMARY_DARKER;
use crate::controller::player::Player;
use crate::language::M18NTtext;
use crate::message::NEW_SPELL;
use crate::registry::Registry;
use crate::script::javascript_loader::JavaScriptContext;
use crate::se::SEEvent;
use crate::se::HAKKEN;
use crate::set::FixedUpdateInGameSet;
use crate::spell::Spell;
use crate::states::GameMenuState;
use crate::states::TimeState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseUiSlice;

#[derive(Component)]
pub struct NewSpell;

fn spawn_new_spell_window(
    commands: &mut Commands,
    registry: &Registry,
    time: &mut ResMut<NextState<TimeState>>,
    spell: Spell,
    se: &mut EventWriter<SEEvent>,
) {
    se.send(SEEvent::new(HAKKEN));

    time.set(TimeState::Inactive);

    let props = registry.get_spell_props(&spell);
    commands
        // 背景
        .spawn((
            NewSpell,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor::from(Color::hsla(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|builder| {
            builder
                .spawn(
                    // ウィンドウ
                    (
                        Node {
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor::from(UI_PRIMARY),
                    ),
                )
                .with_children(|builder| {
                    // ウィンドウタイトル
                    builder
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_child((M18NTtext(NEW_SPELL.to_string()), TextColor::BLACK));

                    // ウィンドウ本体
                    builder
                        .spawn((Node {
                            padding: UiRect::all(Val::Px(10.0)),
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.0),
                            ..default()
                        },))
                        .with_children(|builder| {
                            // 呪文のアイコン
                            builder.spawn((
                                Node {
                                    width: Val::Px(64.0),
                                    height: Val::Px(64.0),
                                    ..default()
                                },
                                AseUiSlice {
                                    aseprite: registry.assets.atlas.clone(),
                                    name: props.icon.to_string(),
                                },
                                BackgroundColor::from(UI_PRIMARY_DARKER),
                            ));
                            builder
                                .spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                })
                                .with_children(|builder| {
                                    // 呪文の名前
                                    builder.spawn((
                                        Node {
                                            padding: UiRect::vertical(Val::Px(10.0)),
                                            ..default()
                                        },
                                        M18NTtext::new(&props.name),
                                        TextColor::BLACK,
                                        TextFont {
                                            font_size: 32.0,
                                            ..default()
                                        },
                                    ));

                                    // 呪文の説明
                                    builder.spawn((
                                        Node {
                                            width: Val::Px(300.0),
                                            ..default()
                                        },
                                        M18NTtext::new(&props.description),
                                        TextColor::BLACK,
                                    ));
                                });
                        });
                });
        });
}

fn open(
    mut commands: Commands,
    registry: Registry,
    mut time: ResMut<NextState<TimeState>>,
    mut se: EventWriter<SEEvent>,
    mut player_query: Query<(&Actor, &mut Player)>,
    query: Query<Entity, With<NewSpell>>,
) {
    let Ok((actor, mut player)) = player_query.get_single_mut() else {
        return;
    };

    if !query.is_empty() {
        return;
    }

    for spell in actor.inventory.0.iter() {
        if let Some(spell) = spell {
            if !player.discovered_spells.contains(&spell) {
                spawn_new_spell_window(&mut commands, &registry, &mut time, spell.clone(), &mut se);
                player.discovered_spells.insert(spell.clone());
                return;
            }
        };
    }
}

fn close(
    mut commands: Commands,
    query: Query<Entity, With<NewSpell>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut time: ResMut<NextState<TimeState>>,
    menu: Res<State<GameMenuState>>,
    mut script: NonSendMut<JavaScriptContext>,
) {
    if *menu == GameMenuState::Closed
        && (mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right))
    {
        if let Ok(entity) = query.get_single() {
            commands.entity(entity).despawn_recursive();

            time.set(TimeState::Active);

            script.resume();
        }
    }
}

pub struct NewSpellPlugin;

impl Plugin for NewSpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (open, close).in_set(FixedUpdateInGameSet));
    }
}
