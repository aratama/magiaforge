use bevy::prelude::Image;

#[derive(PartialEq, Eq)]
pub enum Tile {
    Blank,
    Wall,
    StoneTile,
    BookShelf,
    Chest,
}

pub fn get_tile(img: &Image, x: i32, y: i32) -> Tile {
    let w = img.width() as i32;
    let h = img.height() as i32;

    if x < 0 || x >= w || y < 0 || y >= h {
        return Tile::Blank;
    }

    let i = 4 * (y * img.width() as i32 + x) as usize;
    let r = img.data[i + 0];
    let g = img.data[i + 1];
    let b = img.data[i + 2];
    let a = img.data[i + 3];
    match (r, g, b, a) {
        (203, 219, 252, 255) => Tile::StoneTile,
        (82, 75, 36, 255) => Tile::Wall,
        (118, 66, 138, 255) => Tile::BookShelf,
        (251, 242, 54, 255) => Tile::Chest,
        _ => Tile::Blank,
    }
}
