use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
       let depth = 5;
       minimax(board, player, depth, i32::MIN, i32::MAX)
   }
}

fn minimax(board: &mut Board, player: Player, depth: usize, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {
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
   if player == Player::X {
       let mut best_score = i32::MIN;
       for m in moves {
           board.apply_move(m, player);
           let score = minimax(board, player.flip(), depth - 1, alpha, beta).0;
           board.undo_move(m, player);
           if score > best_score {
               best_score = score;
               best_move = m;
           }
           alpha = alpha.max(score);
           if beta <= alpha {
               break;
           }
       }
       (best_score, best_move.0, best_move.1)


   } else {
       let mut best_score = i32::MAX;
       for m in moves {
           board.apply_move(m, player);
           let score = minimax(board, player.flip(), depth - 1, alpha, beta).0;
           board.undo_move(m, player);
           if score < best_score {
               best_score = score;
               best_move = m;
           }
           beta = beta.min(score);
           if beta <= alpha {
               break;
           }
       }
       (best_score, best_move.0, best_move.1)
   }
}

