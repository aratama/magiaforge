use crate::actor::ActorType;
use crate::constant::*;
use crate::entity::dropped_item::spawn_dropped_item;
use crate::inventory::InventoryItem;
use crate::level::chunk::index_to_position;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::registry::Registry;
use crate::spell::Spell;
use crate::states::GameState;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use vleue_navigator::prelude::NavMeshSettings;
use vleue_navigator::prelude::NavMeshUpdateMode;
use vleue_navigator::Triangulation;

use super::chunk::LevelChunk;

pub fn spawn_navigation_mesh(commands: &mut Commands, chunk: &LevelChunk) {
    commands.spawn((
        StateScoped(GameState::InGame),
        NavMeshSettings {
            // デスクトップ版では問題ないが、ブラウザ版ではナビメッシュの生成がかなり重い
            // 4.0 で少し改善する？
            simplify: 4.0,
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&[
                // ここ半タイルぶんズレてる？
                index_to_position((chunk.min_x, chunk.min_y)),
                index_to_position((chunk.max_x, chunk.min_y)),
                index_to_position((chunk.max_x, chunk.max_y)),
                index_to_position((chunk.min_x, chunk.max_y)),
            ]),

            // 小さすぎると、わずかな隙間を通り抜けようとしたり、
            // 曲がり角で壁に近すぎて減速してしまいます
            // 大きすぎると、対象が壁際にいるときに移動不可能な目的地になってしまうので、
            // パスが見つけられなくなってしまいます
            // Actorのデフォルトが5なので、それに合わせています
            agent_radius: 5.0,
            ..default()
        },
        NavMeshUpdateMode::Debounced(5.0),
        Transform::from_translation(Vec3::ZERO),
    ));
}

pub fn spawn_random_enemies(
    empties: &Vec<(i32, i32)>,
    mut rng: &mut StdRng,
    safe_zone_center: (i32, i32),
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

        if Vec2::new(safe_zone_center.0 as f32, safe_zone_center.1 as f32)
            .distance(Vec2::new(x as f32, y as f32))
            < 8.0
        {
            continue;
        }

        let position = Vec2::new(
            TILE_SIZE * x as f32 + TILE_HALF,
            TILE_SIZE * -y as f32 - TILE_HALF,
        );

        match enemy_types.choose(&mut rng) {
            Some(enemy_type) => {
                spawn.send(SpawnEvent {
                    position,
                    spawn: Spawn::Actor(enemy_type.clone()),
                });
            }
            None => {
                warn!("No enemy type found");
            }
        }

        enemies += 1;
    }
}

pub fn spawn_dropped_items(
    mut commands: &mut Commands,
    registry: &Registry,
    item_map: &HashMap<(i32, i32), Spell>,
) {
    for ((x, y), spell) in item_map.iter() {
        let position = index_to_position((*x, *y));
        spawn_dropped_item(
            &mut commands,
            &registry,
            position,
            &InventoryItem {
                spell: spell.clone(),
                price: 0,
            },
        );
    }
}
