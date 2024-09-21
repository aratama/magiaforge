use crate::ldtk_util::*;
use crate::serialize::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Person;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_data: PlayerData,
) {
    commands.spawn((
        Person,
        SpriteBundle {
            transform: Transform::from_xyz(player_data.x, player_data.y, 1.0),
            texture: asset_server.load("Pixel Art Top Down Basic/TX Player.png"),
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.35)),
                rect: Some(Rect::new(0.0, 0.0, 32.0, 58.0)),
                ..default()
            },
            ..default()
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
        ),
        (With<Person>, Without<Camera2d>),
    >,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Person>)>,

    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    let speed = 3.0;

    let velocity = Vec2::new(
        to_s(&keys, KeyCode::KeyD) - to_s(&keys, KeyCode::KeyA),
        to_s(&keys, KeyCode::KeyW) - to_s(&keys, KeyCode::KeyS),
    )
    .normalize_or_zero()
        * speed;

    let (mut player, mut controller, _) = player_query.single_mut();

    player.translation.z = -player.translation.y;
    controller.translation = Some(velocity);

    let mut camera = camera_query.single_mut();

    camera.translation.x += (player.translation.x - camera.translation.x) * 0.1;
    camera.translation.y += (player.translation.y - camera.translation.y) * 0.1;

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
