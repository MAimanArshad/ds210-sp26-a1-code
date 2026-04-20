use std::fmt::{Debug, Display};

use crate::color;
use crate::layout::Layout;
use crate::player::Player;

// The contents of a single cell.
#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    X,
    O,
    Empty,
    Wall,
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::X => write!(f, "{}X{}", color::Fg(color::Green), color::Fg(color::Reset)),
            Cell::O => write!(f, "{}O{}", color::Fg(color::Blue), color::Fg(color::Reset)),
            Cell::Empty => write!(f, " "),
            Cell::Wall => write!(f, "{}W{}", color::Fg(color::Red), color::Fg(color::Reset)),
        }
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

// The board game state.
#[derive(Clone)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    // Create a new board given a layout.
    pub fn new<L: Layout>(layout: L) -> Board {
        Board { cells: layout.create_board() }
    }

    // Return all legal moves available on this board.
    pub fn moves(&self) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if let Cell::Empty = self.cells[i][j] {
                    moves.push((i, j));
                }
            }
        }
        return moves;
    }

    // Check if the game is over.
    pub fn game_over(&self) -> bool {
        // 3x3 board ends as soon as someone gets a 3 in a row.
        if self.moves().len() == 0 {
            return true;
        }
        if self.cells.len() == 3 {
            return self.score() != 0;
        }
        return false;
    }

    // Score the game. This counts the number of 3 consecutive Xs and 3 consecutive Os,
    // and returns their difference.
    // If score > 0, then there are more consecutive Xs than Os, and the X player wins.
    // If score < 0, then the O player wins.
    // Score == 0 indicates a draw.
    pub fn score(&self) -> i32 {
        let mut score: i32 = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                // Count row.
                if j + 2 < self.cells.len() {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i][j + 1];
                    let z = &self.cells[i][j + 2];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }
                // Count col.
                if i + 2 < self.cells.len() {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i + 1][j];
                    let z = &self.cells[i + 2][j];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }
                // 1st diagonal
                if i + 2 < self.cells.len() && j + 2 < self.cells.len() {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i + 1][j + 1];
                    let z = &self.cells[i + 2][j + 2];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }

                // 2nd diagonal
                if i + 2 < self.cells.len() && j >= 2 {
                    let x = &self.cells[i][j];
                    let y = &self.cells[i + 1][j - 1];
                    let z = &self.cells[i + 2][j - 2];
                    if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
                        score += 1;
                    } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
                        score -= 1;
                    }
                }
            }
        }

        return score;
    }

    pub fn heuristic_evaluation(&self) -> i32 {
        let mut score = 0;
        let n = self.cells.len();
        for i in 0..n {
            for j in 0..n {
                // 3's
                // Row
                if j + 2 < n {
                    score += Self::eval3([
                        &self.cells[i][j],
                        &self.cells[i][j + 1],
                        &self.cells[i][j + 2],
                    ]);
                }
                // Col
                if i + 2 < n {
                    score += Self::eval3([
                        &self.cells[i][j],
                        &self.cells[i + 1][j],
                        &self.cells[i + 2][j],
                    ]);
                }
                // Diag going right
                if i + 2 < n && j + 2 < n {
                    score += Self::eval3([
                        &self.cells[i][j],
                        &self.cells[i + 1][j + 1],
                        &self.cells[i + 2][j + 2],
                    ]);
                }
                // Diag going left
                if i + 2 < n && j >= 2 {
                    score += Self::eval3([
                        &self.cells[i][j],
                        &self.cells[i + 1][j - 1],
                        &self.cells[i + 2][j - 2],
                    ]);
                }
                // 4's
                // Row
                if j + 3 < n {
                    score += Self::eval4([
                        &self.cells[i][j],
                        &self.cells[i][j + 1],
                        &self.cells[i][j + 2],
                        &self.cells[i][j + 3],
                    ]);
                }
                // Col
                if i + 3 < n {
                    score += Self::eval4([
                        &self.cells[i][j],
                        &self.cells[i + 1][j],
                        &self.cells[i + 2][j],
                        &self.cells[i + 3][j],
                    ]);
                }
                // Diag going right
                if i + 3 < n && j + 3 < n {
                    score += Self::eval4([
                        &self.cells[i][j],
                        &self.cells[i + 1][j + 1],
                        &self.cells[i + 2][j + 2],
                        &self.cells[i + 3][j + 3],
                    ]);
                }
                // Diag going left
                if i + 3 < n && j >= 3 {
                    score += Self::eval4([
                        &self.cells[i][j],
                        &self.cells[i + 1][j - 1],
                        &self.cells[i + 2][j - 2],
                        &self.cells[i + 3][j - 3],
                    ]);
                }
            }
        }
        score
    }

    fn eval3(segment: [&Cell; 3]) -> i32 {
        let mut x = 0;
        let mut o = 0;
        for c in segment {
            match c {
                Cell::X => x += 1,
                Cell::O => o += 1,
                Cell::Empty => {},
                _ => {}
            }
        }
        if x > 0 && o > 0 {
            return 0;
        }
        match (x, o) {
            (3, 0) => 10,
            (2, 0) => 5,
            (1, 0) => 2,
            (0, 3) => -10,
            (0, 2) => -5,
            (0, 1) => -2,
            _ => 0,
        }
    }

    fn eval4(segment: [&Cell; 4]) -> i32 {
        let mut x = 0;
        let mut o = 0;
        for c in segment {
            match c {
                Cell::X => x += 1,
                Cell::O => o += 1,
                Cell::Empty => {},
                _ => {}
            }
        }
        if x > 0 && o > 0 {
            return 0;
        }
        match (x, o) {
            (4, 0) => 20,
            (3, 0) => 10,
            (2, 0) => 5,
            (0, 4) => -20,
            (0, 3) => -10,
            (0, 2) => -5,
            _ => 0,
        }
    }

    // Apply the given move to the board.
    // This modifies the board, it does not copy it.
    // To ensure independent moves do not interfer with each other in your min max
    // implementation either:
    //
    // (1) copy the board before applying a move, e.g.
    //        let board2 = board.clone();
    //        board2.apply_move(m, player);
    //
    // (2) undo the move after it is no longer needed, e.g.
    //        board.apply_move(m, player);
    //        ... code that uses board here ...
    //        board.undo_move(m, player);
    //     You must undo the move before applying any other *independent* moves
    //     to the board.
    //     The value of player should be the same when applying and undoing a move.
    pub fn apply_move(&mut self, m: (usize, usize), player: Player) {
        if let Cell::Empty = self.cells[m.0][m.1] {
            match player {
                Player::X => self.cells[m.0][m.1] = Cell::X,
                Player::O => self.cells[m.0][m.1] = Cell::O,
            };
        } else {
            panic!("Illegal move");
        }
    }

    // Undos the given move by the given player.
    // Sets the location of the cell to empty, if it's content matches the player.
    pub fn undo_move(&mut self, m: (usize, usize), player: Player) {
        match (player, &self.cells[m.0][m.1]) {
            (Player::X, Cell::X) => self.cells[m.0][m.1] = Cell::Empty,
            (Player::O, Cell::O) => self.cells[m.0][m.1] = Cell::Empty,
            _ => panic!("Illegal undo move"),
        }
    }

    // Returns a read-only reference to the underlying 2D vec.
    // You may use this in your heuristic to look at the values of specific cells,
    // or loop over the entire board, etc.
    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        return &self.cells;
    }
}

// Pretty-printing board.
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-----------")?;
        for row in &self.cells {
            write!(f, "  {} ", row[0])?;
            for cell in row.iter().skip(1) {
                write!(f, "| {} ", cell)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "-----------")
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}