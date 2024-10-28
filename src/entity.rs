pub mod actor;
pub mod bullet;
pub mod witch;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameEntity {
    Chest,
    BookShelf,
}
