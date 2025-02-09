use super::chunk::LevelChunk;
use super::world::LevelScoped;
use crate::actor::ActorType;
use crate::constant::*;
use crate::level::chunk::index_to_position;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::level::world::GameLevel;
use crate::states::GameState;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use vleue_navigator::prelude::NavMeshSettings;
use vleue_navigator::prelude::NavMeshUpdateMode;
use vleue_navigator::Triangulation;

#[derive(Component)]
pub struct ChunkNavMesh {
    pub level: GameLevel,
}

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

pub fn spawn_random_enemies(
    level: &GameLevel,
    empties: &Vec<(i32, i32)>,
    mut rng: &mut StdRng,
    _safe_zone_centers: Vec<Vec2>,
    spaw_enemy_count: u8,
    enemy_types: &Vec<ActorType>,
    spawn: &mut EventWriter<SpawnEvent>,
) {
    let mut empties = empties.clone();
    empties.shuffle(&mut rng);

    let mut enemies = 0;

    for (x, y) in empties {
        if spaw_enemy_count <= enemies {
            break;
        }

        // todo
        // 魔法陣の付近には敵を出現させない
        // let mut ok = true;
        // for safe_zone_center in safe_zone_centers.iter() {
        //     if Vec2::new(safe_zone_center.0 as f32, safe_zone_center.1 as f32)
        //         .distance(Vec2::new(x as f32, y as f32))
        //         < 8.0
        //     {
        //         ok = false;
        //     }
        // }
        // if !ok {
        //     continue;
        // }

        let position = Vec2::new(
            TILE_SIZE * x as f32 + TILE_HALF,
            TILE_SIZE * -y as f32 - TILE_HALF,
        );

        match enemy_types.choose(&mut rng) {
            Some(enemy_type) => {
                spawn.send(SpawnEvent {
                    position,
                    spawn: Spawn::Actor {
                        actor_type: enemy_type.0.clone(),
                    },
                });
            }
            None => {
                warn!("No enemy type found in {:?}", level);
                return;
            }
        }

        enemies += 1;
    }
}
