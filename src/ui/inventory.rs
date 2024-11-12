use crate::{
    asset::GameAssets,
    constant::{MAX_ITEMS_IN_INVENTORY, WAND_EDITOR_FLOATING_Z_INDEX, WAND_EDITOR_Z_INDEX},
    controller::player::Player,
    inventory_item::InventoryItem,
    spell_props::spell_to_props,
    states::GameState,
    wand::wand_to_props,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::{AsepriteSlice, AsepriteSliceUiBundle};

#[derive(Component)]
struct InventoryItemSlot(usize);

#[derive(Component)]
struct InventoryItemFloating;

pub fn spawn_inventory(builder: &mut ChildBuilder, assets: &Res<GameAssets>) {
    builder
        .spawn((
            StateScoped(GameState::InGame),
            NodeBundle {
                style: Style {
                    // Make the height of the node fill its parent
                    height: Val::Percent(100.0),
                    // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                    // As the height is set explicitly, this means the width will adjust to match the height
                    aspect_ratio: Some(1.0),
                    // Use grid layout for this node
                    display: Display::Grid,
                    // Add 24px of padding around the grid
                    padding: UiRect::all(Val::Px(2.0)),
                    // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized columns
                    grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
                    // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                    // This creates 4 exactly evenly sized rows
                    grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
                    // Set a 12px gap/gutter between rows and columns
                    row_gap: Val::Px(2.0),
                    column_gap: Val::Px(2.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::hsla(0.0, 0.0, 0.5, 0.2)),
                ..default()
            },
        ))
        .with_children(|builder| {
            for i in 0..MAX_ITEMS_IN_INVENTORY {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            padding: UiRect::all(Val::Px(3.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK.into()),
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            InventoryItemSlot(i),
                            StateScoped(GameState::MainMenu),
                            Interaction::default(),
                            ImageBundle {
                                style: Style {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                    ..default()
                                },
                                ..default()
                            },
                            AsepriteSliceUiBundle {
                                aseprite: assets.atlas.clone(),
                                slice: "empty".into(),
                                ..default()
                            },
                        ));
                    });
            }
        });
}

pub fn spawn_inventory_floating(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands.spawn((
        InventoryItemFloating,
        StateScoped(GameState::MainMenu),
        Interaction::default(),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(500.0),
                left: Val::Px(500.0),
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            z_index: ZIndex::Global(WAND_EDITOR_FLOATING_Z_INDEX),
            background_color: Color::WHITE.into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        AsepriteSliceUiBundle {
            aseprite: assets.atlas.clone(),
            slice: "empty".into(),
            ..default()
        },
    ));
}

fn update_inventory_floaing(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Style, With<InventoryItemFloating>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut floating_query: Query<&mut Visibility, With<InventoryItemFloating>>,
) {
    let mut style = query.single_mut();
    if let Ok(window) = windows_query.get_single() {
        if let Some(position) = window.cursor_position() {
            style.left = Val::Px(position.x - 16.0);
            style.top = Val::Px(position.y - 16.0);
        }
    }

    if mouse.just_released(MouseButton::Left) {
        let mut floating = floating_query.single_mut();
        *floating = Visibility::Hidden;
    }
}

fn update_inventory(
    query: Query<&Player>,
    mut slot_query: Query<(&InventoryItemSlot, &mut AsepriteSlice)>,
) {
    if let Ok(player) = query.get_single() {
        for (slot, mut aseprite) in slot_query.iter_mut() {
            let item = &player.inventory[slot.0];
            let slice: &'static str = match item {
                None => "empty",
                Some(InventoryItem::Wand(wand)) => {
                    let props = wand_to_props(*wand);
                    props.slice
                }
                Some(InventoryItem::Spell(spell)) => {
                    let props = spell_to_props(*spell);
                    props.slice
                }
            };
            *aseprite = slice.into();
        }
    }
}

fn interaction(
    mut interaction_query: Query<
        (&InventoryItemSlot, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>),
    >,
    player_query: Query<&Player>,
    mut floating_query: Query<(&mut AsepriteSlice, &mut Visibility), With<InventoryItemFloating>>,
) {
    for (slot, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::hsla(0.0, 0.0, 0.5, 0.3).into();

                if let Ok(player) = player_query.get_single() {
                    let item = player.inventory[slot.0];
                    let slice: Option<&'static str> = match item {
                        None => None,
                        Some(InventoryItem::Wand(wand)) => {
                            let props = wand_to_props(wand);
                            Some(props.slice)
                        }
                        Some(InventoryItem::Spell(spell)) => {
                            let props = spell_to_props(spell);
                            Some(props.slice)
                        }
                    };
                    if let Some(slice) = slice {
                        let (mut floating_slice, mut visibility) = floating_query.single_mut();
                        *floating_slice = slice.into();
                        *visibility = Visibility::Visible;
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::hsla(0.0, 0.0, 0.5, 0.2).into();
            }
            Interaction::None => {
                *color = Color::hsla(0.0, 0.0, 0.5, 0.1).into();
            }
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_inventory, interaction, update_inventory_floaing)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
