use tic_tac_toe_stencil::board::Cell;
use tic_tac_toe_stencil::layout::Layout;

// Presaved board layouts.
pub enum Layout5x5 {
    ThreeByThree,
    Empty,
    Random(usize),
}

impl Layout for Layout5x5 {
    fn create_board(self) -> Vec<Vec<Cell>> {
        let board_cells = match self {
            // Walls all around, three by three empty cells in the middle.
            Layout5x5::ThreeByThree => vec![
                vec![Cell::Wall; 5],
                vec![
                    Cell::Wall,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Wall,
                ],
                vec![
                    Cell::Wall,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Wall,
                ],
                vec![
                    Cell::Wall,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Wall,
                ],
                vec![Cell::Wall; 5],
            ],

            // No walls.
            Layout5x5::Empty => vec![vec![Cell::Empty; 5]; 5],

            // Random walls, the count of walls is determined by `walls`.
            Layout5x5::Random(walls) => {
                use rand::Rng;

                let mut locations = std::collections::HashSet::new();
                while locations.len() < walls {
                    let i: usize = rand::thread_rng().gen_range(0..5);
                    let j: usize = rand::thread_rng().gen_range(0..5);
                    locations.insert((i, j));
                }
                let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Empty; 5]; 5];
                for (i, j) in locations {
                    cells[i][j] = Cell::Wall;
                }
                cells
            }
        };

        board_cells
    }
}