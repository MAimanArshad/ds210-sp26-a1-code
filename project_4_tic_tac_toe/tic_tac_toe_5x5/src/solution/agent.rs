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
        unimplemented!("Not yet implemented")
    }
}
