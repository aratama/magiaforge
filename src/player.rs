use crate::ldtk_util::*;
use crate::serialize::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Component)]
pub struct Person;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_data: &PlayerData,
) {
    let texture = asset_server.load("Pixel Art Top Down Basic/TX Player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 128), 4, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Person,
        SpriteBundle {
            transform: Transform::from_xyz(player_data.x, player_data.y, 1.0),
            texture,
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, 0.1)),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        KinematicCharacterController::default(),
        RigidBody::KinematicPositionBased,
        Collider::ball(10.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}

pub fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            &GlobalTransform,
            &mut TextureAtlas,
            &mut Sprite,
        ),
        (With<Person>, Without<Camera2d>),
    >,
    mut camera_query: Query<
        (&Camera, &mut Transform, &GlobalTransform),
        (With<Camera2d>, Without<Person>),
    >,

    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let speed = 3.0;

    let velocity = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero()
        * speed;

    let (mut player, mut controller, _, mut texture_atlas, mut player_sprite) =
        player_query.single_mut();

    player.translation.z = -player.translation.y;
    controller.translation = Some(velocity);

    let (camera, mut camera_transform, camera_global_transform) = camera_query.single_mut();

    camera_transform.translation.x += (player.translation.x - camera_transform.translation.x) * 0.1;
    camera_transform.translation.y += (player.translation.y - camera_transform.translation.y) * 0.1;

    // プレイヤーの向き
    let window = q_window.single();
    let cursor = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_global_transform, cursor))
        .map(|ray| ray.origin.truncate())
        .unwrap_or(Vec2::ZERO);

    let ray = cursor - player.translation.truncate();
    let angle = ray.y.atan2(ray.x);
    let dir = angle_to_direction(angle);
    match dir {
        PlayerDirection::PlayerUp => {
            texture_atlas.index = 1;
            player_sprite.flip_x = false;
        }
        PlayerDirection::PlayerDown => {
            texture_atlas.index = 0;
            player_sprite.flip_x = false;
        }
        PlayerDirection::PlayerLeft => {
            texture_atlas.index = 2;
            player_sprite.flip_x = false;
        }
        PlayerDirection::PlayerRight => {
            texture_atlas.index = 2;
            player_sprite.flip_x = true;
        }
    }

    // レベルの追従
    // https://trouv.github.io/bevy_ecs_ldtk/v0.10.0/how-to-guides/make-level-selection-follow-player.html
    // ただし上記の方法では読み込み済みのレベルを利用して現在のレベルを判定しているため、
    // 歩いて移動する場合は機能しますが、プレイヤーがワープしたり、セーブした位置から再開する場合は機能しません。
    // このため、常にアセット全体を検索して現在のレベルを判定しています
    let player_position = Vec2::new(player.translation.x, player.translation.y);
    if let Ok(project_handle) = ldtk_projects.get_single() {
        if let Some(ldtk_project) = ldtk_project_assets.get(project_handle) {
            let found = find_level(ldtk_project, player_position);
            if let Some(level_iid) = found {
                *level_selection = LevelSelection::Iid(level_iid.clone());
            }
        }
    }
}

fn to_s(keys: &Res<ButtonInput<KeyCode>>, code: bevy::input::keyboard::KeyCode) -> f32 {
    return if keys.pressed(code) { 1.0 } else { 0.0 };
}

enum PlayerDirection {
    PlayerUp,
    PlayerDown,
    PlayerLeft,
    PlayerRight,
}

impl Display for PlayerDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerDirection::PlayerUp => write!(f, "PlayerUp"),
            PlayerDirection::PlayerDown => write!(f, "PlayerDown"),
            PlayerDirection::PlayerLeft => write!(f, "PlayerLeft"),
            PlayerDirection::PlayerRight => write!(f, "PlayerRight"),
        }
    }
}

fn angle_to_direction(angle: f32) -> PlayerDirection {
    if PI * -0.75 <= angle && angle < PI * -0.25 {
        return PlayerDirection::PlayerDown;
    } else if PI * -0.25 <= angle && angle < 0.25 {
        return PlayerDirection::PlayerRight;
    } else if 0.25 <= angle && angle < PI * 0.75 {
        return PlayerDirection::PlayerUp;
    } else {
        return PlayerDirection::PlayerLeft;
    }
}
