use crate::registry::Registry;
use crate::spell::Spell;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct ItemPanel(pub Option<Spell>);

#[derive(Component)]
struct ItemFrame;

#[derive(Component)]
struct FriendMarker;

pub fn spawn_item_panel<T: Component>(
    builder: &mut ChildBuilder,
    registry: &Registry,
    marker: T,
    x: f32,
    y: f32,
    globa_z_index: Option<GlobalZIndex>,
    background: Option<BackgroundColor>,
) {
    let mut b = builder.spawn((
        marker,
        ItemPanel(None),
        Interaction::default(),
        ZIndex(1),
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            // gird layoutだと、兄弟要素の大きさに左右されてレイアウトが崩れてしまう場合があるので、
            // Absoluteでずれないようにする
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            ..default()
        },
        AseUiSlice {
            aseprite: registry.assets.atlas.clone(),
            name: "empty".into(),
        },
    ));

    if let Some(globa_z_index) = globa_z_index {
        b.insert(globa_z_index);
    }

    if let Some(background) = background {
        b.insert(background);
    }

    b.with_children(|builder| {
        builder.spawn((
            FriendMarker,
            AseUiSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "friend".into(),
            },
            ZIndex(-10),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
        ));
        builder.spawn((
            ItemFrame,
            AseUiSlice {
                aseprite: registry.assets.atlas.clone(),
                name: "spell_frame".into(),
            },
            ZIndex(1),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
        ));
    });
}

fn update_inventory_slot(registry: Registry, mut slot_query: Query<(&ItemPanel, &mut AseUiSlice)>) {
    for (slot, mut aseprite) in slot_query.iter_mut() {
        if let Some(spell) = &slot.0 {
            aseprite.name = registry.get_spell_props(spell).icon.clone();
        } else {
            aseprite.name = "empty".into();
        }
    }
}

fn update_panel_width(
    mut frame_query: Query<(&Parent, &mut Node), With<ItemFrame>>,
    mut slot_query: Query<(&ItemPanel, &mut Node), Without<ItemFrame>>,
) {
    for (parent, mut node) in frame_query.iter_mut() {
        let (slot, mut aseprite) = slot_query.get_mut(parent.get()).unwrap();
        if let Some(_) = &slot.0 {
            let width = Val::Px(32.0);
            node.width = width;
            aseprite.width = width;
        } else {
            node.width = Val::Px(32.0);
            node.height = Val::Px(32.0);
        }
    }
}

fn update_item_frame(
    slot_query: Query<&ItemPanel>,
    mut children_query: Query<(&Parent, &mut AseUiSlice), With<ItemFrame>>,
) {
    for (parent, mut aseprite) in children_query.iter_mut() {
        let item_optional = slot_query.get(parent.get()).unwrap();
        match item_optional.0 {
            Some(_) => {
                aseprite.name = "spell_frame".into();
            }
            _ => {
                aseprite.name = "empty".into();
            }
        }
    }
}

fn update_friend_marker(
    slot_query: Query<&ItemPanel>,
    mut children_query: Query<(&Parent, &mut Node), With<FriendMarker>>,
) {
    for (parent, mut aseprite) in children_query.iter_mut() {
        let slot = slot_query.get(parent.get()).unwrap();
        let visible = match &slot.0 {
            Some(spell) => {
                *spell == Spell::new("SummonFriendSlime")
                    || *spell == Spell::new("SummonFriendEyeball")
            }
            _ => false,
        };
        aseprite.display = if visible {
            Display::DEFAULT
        } else {
            Display::None
        };
    }
}

pub struct ItemPanelPlugin;

impl Plugin for ItemPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_inventory_slot,
                update_item_frame,
                update_panel_width,
                update_friend_marker,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
