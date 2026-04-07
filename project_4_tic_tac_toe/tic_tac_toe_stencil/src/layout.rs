use crate::board::Cell;


pub trait Layout {
    fn create_board(self) -> Vec<Vec<Cell>>;
}