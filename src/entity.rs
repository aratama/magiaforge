pub mod actor;
pub mod book_shelf;
pub mod bullet;
pub mod chest;
pub mod magic_circle;
pub mod slime;
pub mod witch;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameEntity {
    Chest,
    BookShelf,
    MagicCircle,
}
