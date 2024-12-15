use crate::{asset::GameAssets, controller::player::Player};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSpriteSlice;

#[derive(Component)]
pub struct InteractionMarker;

pub fn spawn_interaction_marker(builder: &mut ChildBuilder, assets: &Res<GameAssets>, offset: f32) {
    builder.spawn((
        InteractionMarker,
        Visibility::Visible,
        Transform::from_xyz(0.0, offset, 500.0),
        AseSpriteSlice {
            aseprite: assets.atlas.clone(),
            name: "interactive".into(),
        },
        Sprite {
            color: Color::hsla(0.0, 0.0, 1.0, 0.2),
            ..default()
        },
    ));
}

fn update_interactive_marker_visibility(
    player_query: Query<&Transform, With<Player>>,
    mut marker_query: Query<(&Parent, &mut Visibility), (With<InteractionMarker>, Without<Player>)>,
    marker_parent_query: Query<&Transform, (Without<InteractionMarker>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (parent, mut visibility) in marker_query.iter_mut() {
            if let Ok(parent_transform) = marker_parent_query.get(parent.get()) {
                let distance = parent_transform
                    .translation
                    .truncate()
                    .distance(player_transform.translation.truncate());
                if distance < 32.0 {
                    *visibility = Visibility::Inherited;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

pub struct InteractionMarkerPlugin;

impl Plugin for InteractionMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_interactive_marker_visibility);
    }
}
