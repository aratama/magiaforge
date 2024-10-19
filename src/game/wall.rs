use super::tile::*;
use bevy::prelude::*;
use std::collections::HashMap;

/// 壁タイルから衝突矩形を計算します
/// チェストや本棚なども侵入不可能ですが、それらは個別に衝突形状を持つため、ここでは壁のみを扱います
/// TODO: 本棚などのエンティティもここで一括で生成したほうが効率はいい？
/// でもエンティティが個別に削除されることも多そうなので、その場合はエンティティは別のほうがいいかも
/// https://github.com/Trouv/bevy_ecs_ldtk/blob/main/examples/platformer/walls.rs
pub fn get_wall_collisions(image: &Image) -> Vec<Rect> {
    let width = image.width();
    let height = image.height();

    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    // combine wall tiles into flat "plates" in each individual row
    let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

    for y in 0..height {
        let mut row_plates: Vec<Plate> = Vec::new();
        let mut plate_start = None;

        // + 1 to the width so the algorithm "terminates" plates that touch the right edge
        for x in 0..width + 1 {
            match (
                plate_start,
                get_tile(&image, x as i32, y as i32) == Tile::Wall,
            ) {
                (Some(s), false) => {
                    row_plates.push(Plate {
                        left: s,
                        right: (x - 1) as i32,
                    });
                    plate_start = None;
                }
                (None, true) => plate_start = Some(x as i32),
                _ => (),
            }
        }

        plate_stack.push(row_plates);
    }

    // combine "plates" into rectangles across multiple rows
    let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
    let mut prev_row: Vec<Plate> = Vec::new();
    let mut wall_rects: Vec<Rect> = Vec::new();

    // an extra empty row so the algorithm "finishes" the rects that touch the top edge
    plate_stack.push(Vec::new());

    for (y, current_row) in plate_stack.into_iter().enumerate() {
        for prev_plate in &prev_row {
            if !current_row.contains(prev_plate) {
                // remove the finished rect so that the same plate in the future starts a new rect
                if let Some(rect) = rect_builder.remove(prev_plate) {
                    wall_rects.push(rect);
                }
            }
        }
        for plate in &current_row {
            rect_builder
                .entry(plate.clone())
                .and_modify(|e| e.max.y += 1.0)
                .or_insert(Rect::new(
                    plate.left as f32,
                    y as f32,
                    plate.right as f32,
                    y as f32,
                ));
        }
        prev_row = current_row;
    }

    wall_rects
}

// pub fn image_to_collision_map(image: &Image) -> (Vec<bool>, i32, i32) {
//     let width = image.width() as i32;
//     let height = image.height() as i32;

//     /// Represents a wide wall that is 1 tile tall
//     /// Used to spawn wall collisions
//     #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
//     struct Plate {
//         left: i32,
//         right: i32,
//     }

//     let mut vec: Vec<bool> = Vec::new();

//     for y in 0..height {
//         for x in 0..width + 1 {
//             let tile = get_tile(&image, x, y);
//             vec[(width * y + x) as usize] = match tile {
//                 Tile::Wall => true,
//                 _ => false,
//             }
//         }
//     }

//     (vec, width, height)
// }

// fn peek_tile(img: &Vec<bool>, w: i32, h: i32, x: i32, y: i32) -> bool {
//     if x < 0 || x >= w || y < 0 || y >= h {
//         return false;
//     }
//     img[w as usize * y as usize + x as usize]
// }

// pub fn get_collisions(image: &Vec<bool>, width: i32, height: i32) -> Vec<Rect> {
//     /// Represents a wide wall that is 1 tile tall
//     /// Used to spawn wall collisions
//     #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
//     struct Plate {
//         left: i32,
//         right: i32,
//     }

//     // combine wall tiles into flat "plates" in each individual row
//     let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

//     for y in 0..height {
//         let mut row_plates: Vec<Plate> = Vec::new();
//         let mut plate_start = None;

//         // + 1 to the width so the algorithm "terminates" plates that touch the right edge
//         for x in 0..width + 1 {
//             match (plate_start, peek_tile(&image, width, height, x, y)) {
//                 (Some(s), false) => {
//                     row_plates.push(Plate {
//                         left: s,
//                         right: x - 1,
//                     });
//                     plate_start = None;
//                 }
//                 (None, true) => plate_start = Some(x),
//                 _ => (),
//             }
//         }

//         plate_stack.push(row_plates);
//     }

//     // combine "plates" into rectangles across multiple rows
//     let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
//     let mut prev_row: Vec<Plate> = Vec::new();
//     let mut wall_rects: Vec<Rect> = Vec::new();

//     // an extra empty row so the algorithm "finishes" the rects that touch the top edge
//     plate_stack.push(Vec::new());

//     for (y, current_row) in plate_stack.into_iter().enumerate() {
//         for prev_plate in &prev_row {
//             if !current_row.contains(prev_plate) {
//                 // remove the finished rect so that the same plate in the future starts a new rect
//                 if let Some(rect) = rect_builder.remove(prev_plate) {
//                     wall_rects.push(rect);
//                 }
//             }
//         }
//         for plate in &current_row {
//             rect_builder
//                 .entry(plate.clone())
//                 .and_modify(|e| e.max.y += 1.0)
//                 .or_insert(Rect::new(
//                     plate.left as f32,
//                     y as f32,
//                     plate.right as f32,
//                     y as f32,
//                 ));
//         }
//         prev_row = current_row;
//     }

//     wall_rects
// }
