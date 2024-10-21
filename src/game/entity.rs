pub mod book_shelf;
pub mod chest;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameEntity {
    Chest,
    BookShelf,
}
