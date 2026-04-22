use std::cmp::{max, min};
use std::time::{Duration, Instant};

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

const INF: i32 = 1_000_000_000;
const TERMINAL_WEIGHT: i32 = 100_000;
const CURRENT_SCORE_WEIGHT: i32 = 1_000;
const QUIESCENCE_DEPTH: i32 = 2;

type Move = (usize, usize);
type Pattern = [(usize, usize); 3];

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        let moves = board.moves();
        if moves.is_empty() {
            return (0, 0, 0);
        }
        if moves.len() == 1 {
            let m = moves[0];
            return (0, m.0, m.1);
        }

        let n = board.get_cells().len();
        let patterns = generate_patterns(n);

        let safety_ms = if time_limit > 60 { 25 } else { 1 };
        let deadline = Instant::now() + Duration::from_millis(time_limit.saturating_sub(safety_ms));

        // Always keep a legal fallback ready.
        let ordered = order_moves(board, &moves, player, player, &patterns);
        let mut best_move = ordered[0].1;
        let mut best_value = 0;

        let max_depth = moves.len() as i32;
        let mut depth = 1;
        while depth <= max_depth && Instant::now() < deadline {
            match search_root(board, player, depth, deadline, &patterns) {
                Some((value, mv)) => {
                    best_value = value;
                    best_move = mv;
                    depth += 1;
                }
                None => break,
            }
        }

        (best_value, best_move.0, best_move.1)
    }
}

fn search_root(
    board: &mut Board,
    root: Player,
    depth: i32,
    deadline: Instant,
    patterns: &[Pattern],
) -> Option<(i32, Move)> {
    if Instant::now() >= deadline {
        return None;
    }

    let moves = board.moves();
    let ordered = order_moves(board, &moves, root, root, patterns);

    let mut alpha = -INF;
    let beta = INF;
    let mut best_value = -INF;
    let mut best_move = ordered[0].1;

    for &(_, mv) in &ordered {
        board.apply_move(mv, root);
        let value = alphabeta(board, depth - 1, 0, alpha, beta, root.flip(), root, deadline, patterns)?;
        board.undo_move(mv, root);

        if value > best_value {
            best_value = value;
            best_move = mv;
        }
        alpha = max(alpha, best_value);
    }

    Some((best_value, best_move))
}

fn alphabeta(
    board: &mut Board,
    depth: i32,
    qdepth: i32,
    mut alpha: i32,
    mut beta: i32,
    turn: Player,
    root: Player,
    deadline: Instant,
    patterns: &[Pattern],
) -> Option<i32> {
    if Instant::now() >= deadline {
        return None;
    }

    if board.game_over() {
        return Some(terminal_eval(board, root));
    }

    let mut moves = if depth > 0 {
        board.moves()
    } else {
        urgent_moves(board, patterns)
    };

    if depth == 0 {
        if qdepth >= QUIESCENCE_DEPTH || moves.is_empty() {
            return Some(heuristic(board, root, turn, patterns));
        }
    }

    if moves.is_empty() {
        return Some(terminal_eval(board, root));
    }

    let ordered = order_moves(board, &moves, turn, root, patterns);

    if turn == root {
        let mut best = -INF;
        for &(_, mv) in &ordered {
            board.apply_move(mv, turn);
            let value = alphabeta(
                board,
                if depth > 0 { depth - 1 } else { 0 },
                if depth > 0 { 0 } else { qdepth + 1 },
                alpha,
                beta,
                turn.flip(),
                root,
                deadline,
                patterns,
            )?;
            board.undo_move(mv, turn);

            best = max(best, value);
            alpha = max(alpha, best);
            if alpha >= beta {
                break;
            }
        }
        Some(best)
    } else {
        let mut best = INF;
        for &(_, mv) in &ordered {
            board.apply_move(mv, turn);
            let value = alphabeta(
                board,
                if depth > 0 { depth - 1 } else { 0 },
                if depth > 0 { 0 } else { qdepth + 1 },
                alpha,
                beta,
                turn.flip(),
                root,
                deadline,
                patterns,
            )?;
            board.undo_move(mv, turn);

            best = min(best, value);
            beta = min(beta, best);
            if alpha >= beta {
                break;
            }
        }
        Some(best)
    }
}

fn order_moves(
    board: &mut Board,
    moves: &[Move],
    mover: Player,
    root: Player,
    patterns: &[Pattern],
) -> Vec<(i32, Move)> {
    let before = board.score();
    let mut scored = Vec::with_capacity(moves.len());

    for &mv in moves {
        board.apply_move(mv, mover);
        let after = board.score();
        let h = heuristic(board, root, mover.flip(), patterns);
        board.undo_move(mv, mover);

        let point_swing = match mover {
            Player::X => after - before,
            Player::O => before - after,
        };

        let priority = h + 20_000 * point_swing;
        scored.push((priority, mv));
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored
}

fn urgent_moves(board: &Board, patterns: &[Pattern]) -> Vec<Move> {
    let a = analyze(board, patterns);
    let mut urgent = Vec::new();

    for &(r, c) in &a.empty_cells {
        let idx = index(r, c, a.n);
        let ux = a.ux[idx];
        let uo = a.uo[idx];
        let vx = a.vx[idx];
        let vo = a.vo[idx];

        if ux > 0
            || uo > 0
            || ux + uo >= 2
            || vx >= 3
            || vo >= 3
            || (ux > 0 && vx >= 2)
            || (uo > 0 && vo >= 2)
        {
            urgent.push((r, c));
        }
    }

    urgent
}

fn terminal_eval(board: &Board, root: Player) -> i32 {
    let value = board.score() * TERMINAL_WEIGHT;
    if root == Player::X { value } else { -value }
}

fn heuristic(board: &Board, root: Player, to_move: Player, patterns: &[Pattern]) -> i32 {
    let a = analyze(board, patterns);

    let mut pressure_x = 0;
    let mut pressure_o = 0;
    let mut fork_x = 0;
    let mut fork_o = 0;
    let mut best_x = 0;
    let mut best_o = 0;

    for &(r, c) in &a.empty_cells {
        let idx = index(r, c, a.n);

        let ux = a.ux[idx];
        let uo = a.uo[idx];
        let vx = a.vx[idx];
        let vo = a.vo[idx];
        let wx = a.wx[idx];
        let wo = a.wo[idx];

        pressure_x += 18 * ux * ux + 7 * vx * vx + 14 * ux * vx + wx * wx;
        pressure_o += 18 * uo * uo + 7 * vo * vo + 14 * uo * vo + wo * wo;

        if ux >= 2 {
            fork_x += 3;
        }
        if ux >= 1 && vx >= 2 {
            fork_x += 2;
        }
        if vx >= 3 {
            fork_x += 1;
        }

        if uo >= 2 {
            fork_o += 3;
        }
        if uo >= 1 && vo >= 2 {
            fork_o += 2;
        }
        if vo >= 3 {
            fork_o += 1;
        }

        let contested = ux + uo;
        let swing_x = 60 * ux + 45 * uo + 12 * vx + 4 * (wx - wo) + 25 * contested * contested;
        let swing_o = 60 * uo + 45 * ux + 12 * vo + 4 * (wo - wx) + 25 * contested * contested;

        best_x = max(best_x, swing_x);
        best_o = max(best_o, swing_o);
    }

    let empties = a.empty_cells.len() as i32;
    let playable = max(a.playable, 1);
    let empty_pct = 100 * empties / playable;

    let (w2, w1, wp, wf, wi) = if empty_pct > 60 {
        (140, 20, 1, 80, 1)
    } else if empty_pct > 30 {
        (220, 30, 2, 120, 1)
    } else {
        (320, 40, 3, 180, 2)
    };

    let eval_x = CURRENT_SCORE_WEIGHT * a.score_diff
        + w2 * (a.live2_x - a.live2_o)
        + w1 * (a.live1_x - a.live1_o)
        + wp * (pressure_x - pressure_o)
        + wf * (fork_x - fork_o)
        + match to_move {
            Player::X => wi * (best_x - best_o / 2),
            Player::O => wi * (best_x / 2 - best_o),
        };

    if root == Player::X { eval_x } else { -eval_x }
}

struct Analysis {
    n: usize,
    playable: i32,
    score_diff: i32,
    live2_x: i32,
    live2_o: i32,
    live1_x: i32,
    live1_o: i32,
    empty_cells: Vec<Move>,
    ux: Vec<i32>,
    uo: Vec<i32>,
    vx: Vec<i32>,
    vo: Vec<i32>,
    wx: Vec<i32>,
    wo: Vec<i32>,
}

fn analyze(board: &Board, patterns: &[Pattern]) -> Analysis {
    let cells = board.get_cells();
    let n = cells.len();
    let mut playable = 0;
    let mut empty_cells = Vec::new();

    for r in 0..n {
        for c in 0..n {
            match cells[r][c] {
                Cell::Wall => {}
                Cell::Empty => {
                    playable += 1;
                    empty_cells.push((r, c));
                }
                _ => playable += 1,
            }
        }
    }

    let size = n * n;
    let mut score_diff = 0;
    let mut live2_x = 0;
    let mut live2_o = 0;
    let mut live1_x = 0;
    let mut live1_o = 0;

    let mut ux = vec![0; size];
    let mut uo = vec![0; size];
    let mut vx = vec![0; size];
    let mut vo = vec![0; size];
    let mut wx = vec![0; size];
    let mut wo = vec![0; size];

    for pattern in patterns {
        let mut x = 0;
        let mut o = 0;
        let mut empties = [0usize; 3];
        let mut empty_count = 0;
        let mut blocked = false;

        for &(r, c) in pattern.iter() {
            match cells[r][c] {
                Cell::X => x += 1,
                Cell::O => o += 1,
                Cell::Empty => {
                    empties[empty_count] = index(r, c, n);
                    empty_count += 1;
                }
                Cell::Wall => {
                    blocked = true;
                    break;
                }
            }
        }

        if blocked {
            continue;
        }

        if x == 3 {
            score_diff += 1;
            continue;
        }
        if o == 3 {
            score_diff -= 1;
            continue;
        }

        if o == 0 {
            if x == 2 {
                live2_x += 1;
                for &idx in empties.iter().take(empty_count) {
                    ux[idx] += 1;
                    wx[idx] += 1;
                }
            } else if x == 1 {
                live1_x += 1;
                for &idx in empties.iter().take(empty_count) {
                    vx[idx] += 1;
                    wx[idx] += 1;
                }
            } else {
                for &idx in empties.iter().take(empty_count) {
                    wx[idx] += 1;
                }
            }
        }

        if x == 0 {
            if o == 2 {
                live2_o += 1;
                for &idx in empties.iter().take(empty_count) {
                    uo[idx] += 1;
                    wo[idx] += 1;
                }
            } else if o == 1 {
                live1_o += 1;
                for &idx in empties.iter().take(empty_count) {
                    vo[idx] += 1;
                    wo[idx] += 1;
                }
            } else {
                for &idx in empties.iter().take(empty_count) {
                    wo[idx] += 1;
                }
            }
        }
    }

    Analysis {
        n,
        playable,
        score_diff,
        live2_x,
        live2_o,
        live1_x,
        live1_o,
        empty_cells,
        ux,
        uo,
        vx,
        vo,
        wx,
        wo,
    }
}

fn generate_patterns(n: usize) -> Vec<Pattern> {
    let mut patterns = Vec::new();

    for r in 0..n {
        for c in 0..n {
            if c + 2 < n {
                patterns.push([(r, c), (r, c + 1), (r, c + 2)]);
            }
            if r + 2 < n {
                patterns.push([(r, c), (r + 1, c), (r + 2, c)]);
            }
            if r + 2 < n && c + 2 < n {
                patterns.push([(r, c), (r + 1, c + 1), (r + 2, c + 2)]);
            }
            if r + 2 < n && c >= 2 {
                patterns.push([(r, c), (r + 1, c - 1), (r + 2, c - 2)]);
            }
        }
    }

    patterns
}

#[inline]
fn index(r: usize, c: usize, n: usize) -> usize {
    r * n + c
}
