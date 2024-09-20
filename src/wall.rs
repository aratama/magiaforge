use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Bundle, LdtkEntity)]
pub struct WallBundle {
    wall: Wall,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    rigit_body: RigidBody,
    collider: Collider,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            sprite_bundle: LdtkSpriteSheetBundle {
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        // この anchor がなぜか適用されないようなので、いまは update で設定している
                        anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.12, -0.42)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            grid_coords: GridCoords::default(),
            rigit_body: RigidBody::Fixed,
            collider: Collider::cuboid(8.0, 8.0),
        }
    }
}
