use crate::Wall;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Default, Component)]
pub struct Tree;

#[derive(Bundle, LdtkEntity)]
pub struct TreeBundle {
    tree: Tree,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    rigit_body: RigidBody,
    collider: Collider,
}

impl Default for TreeBundle {
    fn default() -> Self {
        Self {
            tree: Tree,
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
            collider: Collider::ball(5.0),
        }
    }
}

fn update_tree(mut tree_query: Query<(&mut Sprite, &mut Transform), (With<Tree>, Without<Wall>)>) {
    for s in &mut tree_query {
        let (mut sprite, mut transform) = s;
        sprite.anchor = bevy::sprite::Anchor::Custom(Vec2::new(0.12, -0.42));
        // TreeはLdtkWorldBundleの子であり、LdtkWorldBundleのzが-1000になっているので、
        // 1000を足したところが座標になる
        transform.translation.z = 1000.0 - transform.translation.y;
    }
}

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tree);
    }
}
