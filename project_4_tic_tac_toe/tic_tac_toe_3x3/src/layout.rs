use tic_tac_toe_stencil::board::Cell;
use tic_tac_toe_stencil::layout::Layout;

// Layout for 3x3 is all empty!
pub struct Layout3x3 {}
impl Layout for Layout3x3 {
    fn create_board(self) -> Vec<Vec<Cell>> {
        return vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty]
        ];
    }
}