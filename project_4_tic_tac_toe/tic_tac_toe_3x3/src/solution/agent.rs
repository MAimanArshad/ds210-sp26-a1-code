use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over() {
            return (board.score(), 0, 0);
        }
        else {
            let moves = board.moves();
            let mut exp_board = board.clone();
            let mut scored: Vec<(i32, usize, usize)> = vec![];
            for step in moves{
                exp_board.apply_move(step, player);
                let score: (i32, usize, usize) = SolutionAgent::solve(&mut exp_board, player.flip());
                exp_board.undo_move(step, player);
                scored.push((score.0, step.0, step.1));

            if player == Player::X {
                return scored.into_iter().max_by_key(|x|, x.0).unwrap();
        }
            else {
                return scored.into_iter().min_by_key(|x|, x.0).unwrap();
            }
        }
    }
}