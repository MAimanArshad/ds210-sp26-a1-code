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
       let depth = 3;
       minimax(board, player, depth)
   }
}


fn minimax(board: &mut Board, player: Player, depth: usize) -> (i32, usize, usize) {
   if board.game_over() {
       return (board.score(), 0, 0);
   }
   if depth == 0 {
       return (board.heuristic_evaluation(), 0, 0);
   }
   let moves = board.moves();
   if moves.is_empty() {
       return (board.heuristic_evaluation(), 0, 0);
   }
   let mut best_move = moves[0];
   board.apply_move(best_move, player);
   let mut best_score = minimax(board, player.flip(), depth - 1).0;
   board.undo_move(best_move, player);
  
   for i in 1..moves.len() {
       let m = moves[i];
       board.apply_move(m, player);
       let score = minimax(board, player.flip(), depth - 1).0;
       board.undo_move(m, player);
       if player == Player::X {
           if score > best_score {
               best_score = score;
               best_move = m;
           }
       }
       else {
           if score < best_score {
               best_score = score;
               best_move = m;
           }
       }
   }
   return (best_score, best_move.0, best_move.1);
}
