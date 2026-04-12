use crate::board::{Board, Cell};
use crate::color;
use crate::player::Player;

// Agent trait that students will implement for their solutions.
pub trait Agent {
    // Must return (score, x coordinate of move, y coordinate of move).
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize);
}

// A dumb solution that makes the first possible move.
pub struct FirstMoveAgent {}
impl Agent for FirstMoveAgent {
    fn solve(board: &mut Board, _player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let moves = board.moves();
        return (0, moves[0].0, moves[0].1);
    }
}

// A dumb solution that makes moves randomly.
pub struct RandomAgent {}
impl Agent for RandomAgent {
    fn solve(board: &mut Board, _player: Player, _time_limit: u64) -> (i32, usize, usize) {
        use rand::Rng;

        let moves = board.moves();
        let i: usize = rand::thread_rng().gen_range(0..moves.len());
        return (0, moves[i].0, moves[i].1);
    }
}

// Slightly better solution based on a heuristic but without any min-max.
pub struct TestAgent {}
impl Agent for TestAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let moves = board.moves();

        // Assume first move is best move for now.
        let mut best_move = moves[0];
        board.apply_move(best_move, player);
        let mut best_score = board.score();
        board.undo_move(best_move, player);

        // Look at the remaining moves, see if any seem better.
        for i in 1..moves.len() {
            let m = moves[i];
            board.apply_move(m, player);
            let score = board.score();
            board.undo_move(m, player);

            match player {
                Player::X => {
                    if score > best_score {
                        best_move = m;
                        best_score = score;
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_move = m;
                        best_score = score;
                    }
                }
            }
        }
        return (best_score, best_move.0, best_move.1);
    }
}

// Agent that just prompts users to play the game via the terminal.
pub struct ManualAgent {}
impl ManualAgent {
    pub fn solve(board: &mut Board, _player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let cells = board.get_cells();

        println!("");
        println!("");
        println!("");
        println!("");
        println!("Enter your move below by typing in the number shown in the desired cell:");
        println!("");

        // Print the board with the possible move numbers to the terminal.
        let mut counter = 0;
        for i in 0..cells.len() {
            for j in 0..cells.len() {
                match &cells[i][j] {
                    Cell::Empty => {
                        print!(
                            " {}{:02}{} |",
                            color::Fg(color::Yellow),
                            counter,
                            color::Fg(color::Reset)
                        );
                        counter += 1;
                    }
                    _ => print!("  {} |", cells[i][j].to_string()),
                }
            }
            println!("");
        }

        // Read the input number from the terminal.
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let input: usize = line.trim().parse().expect("Input not an integer");

        // Make the move.
        let m = board.moves()[input];

        println!("");
        println!("");
        println!("");

        return (0, m.0, m.1);
    }
}


// Private trait for sealing.
mod sealed {
    pub trait SealedAgentTrait {}
    impl SealedAgentTrait for super::ManualAgent {}
    impl<A: super::Agent> SealedAgentTrait for A {}
}

// Public sealed trait.
pub trait SealedAgent: sealed::SealedAgentTrait {
    fn manual() -> bool { false }
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize);
}
impl SealedAgent for ManualAgent {
    fn manual() -> bool { true }
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        ManualAgent::solve(board, player, _time_limit)
    }
}
impl<A: Agent> SealedAgent for A {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        A::solve(board, player, _time_limit)
    }
}
