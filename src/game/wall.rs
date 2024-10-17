use bevy::prelude::*;
use std::collections::HashMap;

// https://github.com/Trouv/bevy_ecs_ldtk/blob/main/examples/platformer/walls.rs
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
            match (plate_start, get_tile(&image, x, y) == Tile::Wall) {
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

#[derive(PartialEq, Eq)]
pub enum Tile {
    Blank,
    Wall,
    Empty,
    BookShelf,
    Chest,
}

pub fn get_tile(img: &Image, x: u32, y: u32) -> Tile {
    let w = img.width();
    let h = img.height();

    if x >= w || y >= h {
        return Tile::Blank;
    }

    let i = 4 * (y * img.width() + x) as usize;
    let r = img.data[i + 0];
    let g = img.data[i + 1];
    let b = img.data[i + 2];
    let a = img.data[i + 3];
    match (r, g, b, a) {
        (203, 219, 252, 255) => Tile::Empty,
        (82, 75, 36, 255) => Tile::Wall,
        (118, 66, 138, 255) => Tile::BookShelf,
        (251, 242, 54, 255) => Tile::Chest,
        _ => Tile::Blank,
    }
}
