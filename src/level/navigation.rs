use super::chunk::LevelChunk;
use super::world::GameWorld;
use super::world::LevelScoped;
use crate::level::chunk::index_to_position;
use crate::level::world::GameLevel;
use crate::states::GameState;
use bevy::prelude::*;
use vleue_navigator::prelude::NavMeshSettings;
use vleue_navigator::prelude::NavMeshUpdateMode;
use vleue_navigator::Triangulation;

#[derive(Component)]
pub struct ChunkNavMesh {
    pub level: GameLevel,
}

// ナビゲーションメッシュを作成します
// ナビメッシュ生成は重いためチャンクごとに生成します
// このため、敵キャラクターがレベル境界を越えて接近することはありません
pub fn spawn_navigation_mesh(commands: &mut Commands, chunk: &LevelChunk) {
    commands.spawn((
        StateScoped(GameState::InGame),
        LevelScoped(chunk.level.clone()),
        ChunkNavMesh {
            level: chunk.level.clone(),
        },
        NavMeshSettings {
            // デスクトップ版では問題ないが、ブラウザ版ではナビメッシュの生成がかなり重い
            // 4.0 で少し改善する？
            simplify: 4.0,
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&[
                // ここ半タイルぶんズレてる？
                index_to_position((chunk.bounds.min_x, chunk.bounds.min_y)),
                index_to_position((chunk.bounds.max_x, chunk.bounds.min_y)),
                index_to_position((chunk.bounds.max_x, chunk.bounds.max_y)),
                index_to_position((chunk.bounds.min_x, chunk.bounds.max_y)),
            ]),

            // 小さすぎると、わずかな隙間を通り抜けようとしたり、
            // 曲がり角で壁に近すぎて減速してしまいます
            // 大きすぎると、対象が壁際にいるときに移動不可能な目的地になってしまうので、
            // パスが見つけられなくなってしまいます
            // Actorのデフォルトが5なので、それに合わせています
            agent_radius: 5.0,
            ..default()
        },
        // todo
        // ナビゲーションメッシュ生成は重くDebouncedはWASMで実用的でない
        NavMeshUpdateMode::OnDemand(true),
        Transform::from_translation(Vec3::ZERO),
    ));
}

pub fn update_dirty_navmesh(
    mut world: ResMut<GameWorld>,
    mut navmesh_query: Query<(&ChunkNavMesh, &mut NavMeshUpdateMode)>,
) {
    for (chunk_navmesh, mut update_mode) in navmesh_query.iter_mut() {
        if let Some(chunk) = world.get_chunk_mut(&chunk_navmesh.level) {
            *update_mode = NavMeshUpdateMode::OnDemand(true);
            chunk.dirty_navmesh = false;
        }
    }
}
