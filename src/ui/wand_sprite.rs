use crate::asset::GameAssets;
use crate::controller::player::Player;
use crate::entity::actor::Actor;
use crate::states::GameMenuState;
use crate::states::GameState;
use crate::ui::floating::Floating;
use crate::ui::floating::FloatingContent;
use crate::ui::popup::PopUp;
use bevy::prelude::*;
use bevy::ui::Node;
use bevy_aseprite_ultra::prelude::*;

#[derive(Component)]
pub struct WandSprite {
    wand_index: usize,
}

pub fn spawn_wand_sprite_in_list(
    parent: &mut ChildBuilder,
    assets: &Res<GameAssets>,
    wand_index: usize,
) {
    parent.spawn((
        WandSprite { wand_index },
        Interaction::default(),
        BackgroundColor(Color::hsla(0.0, 0.3, 0.5, 0.1)),
        Node {
            width: Val::Px(64.),
            height: Val::Px(32.),
            ..default()
        },
        AseUiSlice {
            aseprite: assets.atlas.clone(),
            name: "empty".into(),
        },
    ));
}

fn update_wand_sprite(
    player_query: Query<&Actor, With<Player>>,
    mut sprite_query: Query<(&WandSprite, &mut AseUiSlice)>,
    floating_query: Query<&Floating>,
) {
    let floating = floating_query.single();

    if let Ok(actor) = player_query.get_single() {
        for (wand_sprite, mut aseprite) in sprite_query.iter_mut() {
            aseprite.name = match floating.content {
                Some(FloatingContent::Wand(wand_index)) if wand_index == wand_sprite.wand_index => {
                    "empty".into()
                }
                _ => match &actor.wands[wand_sprite.wand_index] {
                    Some(wand) => wand.wand_type.to_props().icon.into(),
                    None => "empty".into(),
                },
            }
        }
    }
}

fn interact_wand_sprite(
    mut interaction_query: Query<(&WandSprite, &Interaction), Changed<Interaction>>,
    mut floating_query: Query<&mut Floating>,
    state: Res<State<GameMenuState>>,
    mut popup_query: Query<&mut PopUp>,
) {
    if *state.get() != GameMenuState::WandEditOpen {
        return;
    }

    let mut floating = floating_query.single_mut();
    let mut popup = popup_query.single_mut();
    for (slot, interaction) in &mut interaction_query {
        let content = FloatingContent::Wand(slot.wand_index);
        match *interaction {
            Interaction::Pressed => {
                floating.content = Some(content);
            }
            Interaction::Hovered => {
                floating.target = Some(content);
                popup.set.insert(content);
                popup.hang = false;
            }
            Interaction::None => {
                floating.target = Some(content);
                popup.set.remove(&content);
            }
        }
    }
}

pub struct WandSpritePlugin;

impl Plugin for WandSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_wand_sprite, interact_wand_sprite).run_if(in_state(GameState::InGame)),
        );
    }
}
