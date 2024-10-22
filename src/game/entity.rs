pub mod book_shelf;
pub mod bullet;
pub mod chest;
pub mod enemy;
pub mod player;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameEntity {
    Chest,
    BookShelf,
}
