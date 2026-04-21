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


fn heuristic_evaluation(board: &Board) -> i32 {
    let mut score = 0;
    let n = board.get_cells().len();
        for i in 0..n {
            for j in 0..n {
                // 3's
                // Row
                if j + 2 < n {
                    score += board.get_cells().eval3([
                        board.get_cells()[i][j],
                        board.get_cells()[i][j + 1],
                        board.get_cells()[i][j + 2],
                    ]);
                }
                // Col
                if i + 2 < n {
                    score += board.get_cells().eval3([
                        board.get_cells()[i][j],
                        board.get_cells()[i + 1][j],
                        board.get_cells()[i + 2][j],
                    ]);
                }
                // Diag going right
                if i + 2 < n && j + 2 < n {
                    score += board.get_cells().eval3([
                        board.get_cells()[i][j],
                        board.get_cells()[i + 1][j + 1],
                        board.get_cells()[i + 2][j + 2],
                    ]);
                }
                // Diag going left
                if i + 2 < n && j >= 2 {
                    score += board.get_cells().eval3([
                        board.get_cells()[i][j],
                        board.get_cells()[i + 1][j - 1],
                        board.get_cells()[i + 2][j - 2],
                    ]);
                }
                // 4's
                // Row
                if j + 3 < n {
                    score += board.get_cells().eval4([
                        board.get_cells()[i][j],
                        board.get_cells()[i][j + 1],
                        board.get_cells()[i][j + 2],
                        board.get_cells()[i][j + 3],
                    ]);
                }
                // Col
                if i + 3 < n {
                    score += board.get_cells().eval4([
                        board.get_cells()[i][j],
                        board.get_cells()[i + 1][j],
                        board.get_cells()[i + 2][j],
                        board.get_cells()[i + 3][j],
                    ]);
                }
                // Diag going right
                if i + 3 < n && j + 3 < n {
                    score += board.get_cells().eval4([
                        board.get_cells()[i][j],
                        board.get_cells()[i + 1][j + 1],
                        board.get_cells()[i + 2][j + 2],
                        board.get_cells()[i + 3][j + 3],
                    ]);
                }
                // Diag going left
                if i + 3 < n && j >= 3 {
                    score += board.get_cells().eval4([
                        board.get_cells()[i][j],
                        board.get_cells()[i + 1][j - 1],
                        board.get_cells()[i + 2][j - 2],
                        board.get_cells()[i + 3][j - 3],
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