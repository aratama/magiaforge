use super::enemy;
use super::states::GameState;
use bevy::asset::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::*;

fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO:
    // world.ldtk をバイナリに埋め込む場合は、src/game/asset 以下などに配置します
    // ただし、world.ldtk をバイナリに埋め込もうとすると、 world.ldtk はさらに asset.png に依存しているため、
    // bevy-wasm-example\assets\my_bevy_game\asset\asset.png を参照しようとします
    // これによりファイルが見つからなくなってしまうため、world.ldtk をバイナリに埋め込む良い方法がありません
    //
    // let path = Path::new(CRATE_NAME).join("asset/world.ldtk");
    // let asset_path = AssetPath::from_path(&path).with_source(AssetSourceId::from("embedded"));

    commands.spawn((
        StateScoped(GameState::InGame),
        LdtkWorldBundle {
            // ldtk_handle: asset_server.load(asset_path),
            ldtk_handle: asset_server.load("world.ldtk"),

            // タイルマップは最も背後に配置したいので z: -1000にする
            // サンプルコードでは get_tilemap_center_transform を使って画面中央に揃えているが、
            // 座標計算が面倒になるので x:0, y:0 に配置している
            // また、アンカーがタイルの中央になっているので、タイルの大きさの半分だけずらして原点に揃える
            // transform: Transform::from_xyz(0.0, 0.0, -1000.0),
            ..Default::default()
        },
    ));
    // commands.spawn((
    //     Collider::cuboid(400.0, 20.0),
    //     TransformBundle::from(Transform::from_xyz(0.0, -20.0, 0.0)),
    // ));
}

fn update_world(
    enemy_query: Query<&enemy::Enemy>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if enemy_query.is_empty() {
        next_game_state.set(GameState::MainMenu);
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world);
        app.add_systems(
            FixedUpdate,
            update_world.run_if(in_state(GameState::InGame)),
        );
    }
}
