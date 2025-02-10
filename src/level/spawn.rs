use crate::actor::ActorType;
use crate::constant::*;
use crate::level::entities::Spawn;
use crate::level::entities::SpawnEvent;
use crate::level::world::GameLevel;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

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
