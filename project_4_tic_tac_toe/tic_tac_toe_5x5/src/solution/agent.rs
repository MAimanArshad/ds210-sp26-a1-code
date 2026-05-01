use std::cmp::{max, min};
use std::time::{Duration, Instant};

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

const INF: i32 = 1_000_000_000;
const TERMINAL_WEIGHT: i32 = 100_000;
const CURRENT_SCORE_WEIGHT: i32 = 1_000;
const QUIESCENCE_DEPTH_BASE: i32 = 2;
const QUIESCENCE_DEPTH_FORCING: i32 = 3;
const MAX_CELLS: usize = 25;
const PV_BONUS: i32 = 1_000_000;

const PRIORITY_MY_SCORE: i32 = 32_000;
const PRIORITY_BLOCK: i32 = 25_000;
const PRIORITY_CONTESTED: i32 = 8_000;
const PRIORITY_MY_SETUP: i32 = 2_500;
const PRIORITY_BLOCK_SETUP: i32 = 1_400;
const PRIORITY_LIVE_DIFF: i32 = 180;
const PRIORITY_LIVE_TOTAL: i32 = 35;
const PRIORITY_BEHIND_BLOCK_BOOST: i32 = 9_000;
const PRIORITY_BEHIND_SETUP_DENIAL: i32 = 1_100;
const PRIORITY_BEHIND_FORK_DENIAL: i32 = 3_500;

type Move = (usize, usize);
type Pattern = [(usize, usize); 3];

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        let patterns = generate_patterns(board.get_cells().len());
        let analysis = analyze(board, &patterns);

        if analysis.empty_count == 0 {
            return (0, 0, 0);
        }
        if analysis.empty_count == 1 {
            let mv = analysis.empty_cells[0];
            return (0, mv.0, mv.1);
        }

        let safety_ms = if time_limit > 80 { 25 } else { 1 };
        let deadline = Instant::now() + Duration::from_millis(time_limit.saturating_sub(safety_ms));

        let mut best_move = best_fallback_move(&analysis, player);
        let mut best_value = 0;
        let mut depth = 1;
        let max_depth = analysis.empty_count as i32;

        while depth <= max_depth && Instant::now() < deadline {
            match search_root(board, player, depth, deadline, &patterns, Some(best_move)) {
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

fn best_fallback_move(analysis: &Analysis, mover: Player) -> Move {
    let mut best_move = analysis.empty_cells[0];
    let mut best_priority = -INF;

    for i in 0..analysis.empty_count {
        let mv = analysis.empty_cells[i];
        let priority = move_priority(analysis, mv, mover);
        if priority > best_priority {
            best_priority = priority;
            best_move = mv;
        }
    }

    best_move
}

fn search_root(
    board: &mut Board,
    root: Player,
    depth: i32,
    deadline: Instant,
    patterns: &[Pattern],
    pv_move: Option<Move>,
) -> Option<(i32, Move)> {
    if Instant::now() >= deadline {
        return None;
    }

    let analysis = analyze(board, patterns);
    let ordered = ordered_moves(&analysis, root, pv_move);

    let mut alpha = -INF;
    let beta = INF;
    let mut best_value = -INF;
    let mut best_move = ordered[0];

    for mv in ordered {
        board.apply_move(mv, root);
        let value = alphabeta(board, depth - 1, 0, alpha, beta, root.flip(), root, deadline, patterns, None)?;
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
    pv_move: Option<Move>,
) -> Option<i32> {
    if Instant::now() >= deadline {
        return None;
    }

    let analysis = analyze(board, patterns);

    if analysis.empty_count == 0 {
        return Some(terminal_eval_from_score(analysis.score_diff, root));
    }

    if depth == 0 {
        let (urgent, forcing) = urgent_moves_info(&analysis);
        let qlimit = if forcing && urgent.len() <= 5 {
            QUIESCENCE_DEPTH_FORCING
        } else {
            QUIESCENCE_DEPTH_BASE
        };

        if qdepth >= qlimit || urgent.is_empty() {
            return Some(heuristic_from_analysis(&analysis, root, turn));
        }

        let ordered = ordered_subset_moves(&analysis, &urgent, turn, pv_move);
        return if turn == root {
            let mut best = -INF;
            for mv in ordered {
                board.apply_move(mv, turn);
                let value = alphabeta(board, 0, qdepth + 1, alpha, beta, turn.flip(), root, deadline, patterns, None)?;
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
            for mv in ordered {
                board.apply_move(mv, turn);
                let value = alphabeta(board, 0, qdepth + 1, alpha, beta, turn.flip(), root, deadline, patterns, None)?;
                board.undo_move(mv, turn);

                best = min(best, value);
                beta = min(beta, best);
                if alpha >= beta {
                    break;
                }
            }
            Some(best)
        };
    }

    let ordered = ordered_moves(&analysis, turn, pv_move);

    if turn == root {
        let mut best = -INF;
        for mv in ordered {
            board.apply_move(mv, turn);
            let value = alphabeta(board, depth - 1, 0, alpha, beta, turn.flip(), root, deadline, patterns, None)?;
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
        for mv in ordered {
            board.apply_move(mv, turn);
            let value = alphabeta(board, depth - 1, 0, alpha, beta, turn.flip(), root, deadline, patterns, None)?;
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

fn ordered_moves(analysis: &Analysis, mover: Player, pv_move: Option<Move>) -> Vec<Move> {
    let mut scored = Vec::with_capacity(analysis.empty_count);

    for i in 0..analysis.empty_count {
        let mv = analysis.empty_cells[i];
        let mut priority = move_priority(analysis, mv, mover);
        if Some(mv) == pv_move {
            priority += PV_BONUS;
        }
        scored.push((priority, mv));
    }

    scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, mv)| mv).collect()
}

fn ordered_subset_moves(
    analysis: &Analysis,
    moves: &[Move],
    mover: Player,
    pv_move: Option<Move>,
) -> Vec<Move> {
    let mut scored = Vec::with_capacity(moves.len());

    for &mv in moves {
        let mut priority = move_priority(analysis, mv, mover);
        if Some(mv) == pv_move {
            priority += PV_BONUS;
        }
        scored.push((priority, mv));
    }

    scored.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, mv)| mv).collect()
}

fn move_priority(analysis: &Analysis, mv: Move, mover: Player) -> i32 {
    let idx = index(mv.0, mv.1, analysis.n);

    let (my_u, opp_u, my_v, opp_v, my_w, opp_w) = match mover {
        Player::X => (
            analysis.ux[idx],
            analysis.uo[idx],
            analysis.vx[idx],
            analysis.vo[idx],
            analysis.wx[idx],
            analysis.wo[idx],
        ),
        Player::O => (
            analysis.uo[idx],
            analysis.ux[idx],
            analysis.vo[idx],
            analysis.vx[idx],
            analysis.wo[idx],
            analysis.wx[idx],
        ),
    };

    let (my_count, opp_count) = match mover {
        Player::X => (analysis.x_count, analysis.o_count),
        Player::O => (analysis.o_count, analysis.x_count),
    };

    let contested = my_u + opp_u;
    let live_total = my_w + opp_w;
    let behind = my_count < opp_count;
    let early = 100 * analysis.empty_count as i32 / max(analysis.playable, 1) > 55;

    let mut priority = PRIORITY_MY_SCORE * my_u
        + PRIORITY_BLOCK * opp_u
        + PRIORITY_CONTESTED * ((contested >= 2) as i32)
        + PRIORITY_MY_SETUP * my_v
        + PRIORITY_BLOCK_SETUP * opp_v
        + PRIORITY_LIVE_DIFF * (my_w - opp_w)
        + PRIORITY_LIVE_TOTAL * live_total * live_total;

    // If we are the second mover, we are usually one tempo behind.
    // In that case, blocking future forks matters more than building a cute setup.
    if behind {
        priority += PRIORITY_BEHIND_BLOCK_BOOST * opp_u;
        priority += PRIORITY_BEHIND_SETUP_DENIAL * opp_v * opp_v;
        priority += 180 * opp_v * opp_w;
        priority += PRIORITY_BEHIND_FORK_DENIAL * ((opp_v >= 2 && opp_w >= 4) as i32);

        if early {
            priority += 80 * opp_w * opp_w;
            priority += 700 * ((opp_v >= 3) as i32);
        }
    }

    priority
}

fn urgent_moves_info(analysis: &Analysis) -> (Vec<Move>, bool) {
    let mut urgent = Vec::new();
    let mut forcing = false;

    for i in 0..analysis.empty_count {
        let mv = analysis.empty_cells[i];
        let idx = index(mv.0, mv.1, analysis.n);
        let ux = analysis.ux[idx];
        let uo = analysis.uo[idx];
        let vx = analysis.vx[idx];
        let vo = analysis.vo[idx];

        let is_urgent = ux > 0
            || uo > 0
            || ux + uo >= 2
            || vx >= 3
            || vo >= 3
            || (ux > 0 && vx >= 2)
            || (uo > 0 && vo >= 2);

        if is_urgent {
            urgent.push(mv);
        }

        if ux > 0 || uo > 0 || ux + uo >= 2 {
            forcing = true;
        }
    }

    (urgent, forcing)
}

fn terminal_eval_from_score(score_diff: i32, root: Player) -> i32 {
    let value = score_diff * TERMINAL_WEIGHT;
    if root == Player::X { value } else { -value }
}

fn heuristic_from_analysis(analysis: &Analysis, root: Player, to_move: Player) -> i32 {
    let mut pressure_x = 0;
    let mut pressure_o = 0;
    let mut fork_x = 0;
    let mut fork_o = 0;
    let mut best_x = 0;
    let mut best_o = 0;
    let mut cut_x = 0;
    let mut cut_o = 0;
    let mut hot_x = 0;
    let mut hot_o = 0;
    let mut future_x = 0;
    let mut future_o = 0;

    for i in 0..analysis.empty_count {
        let (r, c) = analysis.empty_cells[i];
        let idx = index(r, c, analysis.n);

        let ux = analysis.ux[idx];
        let uo = analysis.uo[idx];
        let vx = analysis.vx[idx];
        let vo = analysis.vo[idx];
        let wx = analysis.wx[idx];
        let wo = analysis.wo[idx];

        pressure_x += 20 * ux * ux + 8 * vx * vx + 16 * ux * vx + wx * wx;
        pressure_o += 20 * uo * uo + 8 * vo * vo + 16 * uo * vo + wo * wo;

        if ux >= 2 {
            fork_x += 4;
        }
        if ux >= 1 && vx >= 2 {
            fork_x += 2;
        }
        if vx >= 3 {
            fork_x += 1;
        }

        if uo >= 2 {
            fork_o += 4;
        }
        if uo >= 1 && vo >= 2 {
            fork_o += 2;
        }
        if vo >= 3 {
            fork_o += 1;
        }

        if ux >= 2 || (ux >= 1 && vx >= 2) || wx >= 5 {
            hot_x += 1;
        }
        if uo >= 2 || (uo >= 1 && vo >= 2) || wo >= 5 {
            hot_o += 1;
        }

        future_x += vx * vx + vx * wx + 4 * ((vx >= 2) as i32) + 4 * ((ux >= 1 && vx >= 2) as i32);
        future_o += vo * vo + vo * wo + 4 * ((vo >= 2) as i32) + 4 * ((uo >= 1 && vo >= 2) as i32);

        let contested = ux + uo;
        cut_x += uo * (2 * uo + vo + contested + 1);
        cut_o += ux * (2 * ux + vx + contested + 1);

        let swing_x = 70 * ux + 56 * uo + 14 * vx + 4 * (wx - wo) + 28 * contested * contested;
        let swing_o = 70 * uo + 56 * ux + 14 * vo + 4 * (wo - wx) + 28 * contested * contested;

        best_x = max(best_x, swing_x);
        best_o = max(best_o, swing_o);
    }

    let empties = analysis.empty_count as i32;
    let playable = max(analysis.playable, 1);
    let empty_pct = 100 * empties / playable;

    let (w2, w1, wp, wf, wc, wd, ws, wh, wfu) = if empty_pct > 60 {
        (150, 24, 1, 90, 42, 12, 22, 10, 26)
    } else if empty_pct > 30 {
        (240, 34, 2, 140, 58, 9, 18, 14, 22)
    } else {
        (360, 46, 3, 210, 76, 6, 12, 18, 12)
    };

    let base_eval_x = CURRENT_SCORE_WEIGHT * analysis.score_diff
        + w2 * (analysis.live2_x - analysis.live2_o)
        + w1 * (analysis.live1_x - analysis.live1_o)
        + wp * (pressure_x - pressure_o)
        + wf * (fork_x - fork_o)
        + wc * (cut_x - cut_o)
        + wd * (analysis.owned_degree_x - analysis.owned_degree_o)
        + ws * (analysis.span_x - analysis.span_o)
        + wh * (hot_x - hot_o)
        + wfu * (future_x - future_o);

    let tempo_for_root = match root {
        Player::X => {
            if to_move == Player::X {
                3 * best_x - best_o + 4 * cut_x - 10 * hot_o
            } else {
                best_x - 2 * best_o + 8 * cut_x - 18 * hot_o
            }
        }
        Player::O => {
            if to_move == Player::O {
                3 * best_o - best_x + 4 * cut_o - 10 * hot_x
            } else {
                best_o - 2 * best_x + 8 * cut_o - 18 * hot_x
            }
        }
    };

    let eval_x = base_eval_x + if root == Player::X { tempo_for_root } else { -tempo_for_root };
    let mut root_value = if root == Player::X { eval_x } else { -eval_x };

    // Second-player correction: when the root player is behind in move count,
    // punish positions where the first player is still growing future forks.
    let root_behind = match root {
        Player::X => analysis.x_count < analysis.o_count,
        Player::O => analysis.o_count < analysis.x_count,
    };

    if root_behind {
        let second_guard = match root {
            Player::X => {
                34 * (future_x - future_o)
                    + 54 * (hot_x - hot_o)
                    + 20 * (analysis.live1_x - analysis.live1_o)
                    + 42 * (analysis.live2_x - analysis.live2_o)
            }
            Player::O => {
                34 * (future_o - future_x)
                    + 54 * (hot_o - hot_x)
                    + 20 * (analysis.live1_o - analysis.live1_x)
                    + 42 * (analysis.live2_o - analysis.live2_x)
            }
        };
        root_value += second_guard;
    }

    root_value
}

struct Analysis {
    n: usize,
    playable: i32,
    score_diff: i32,
    x_count: i32,
    o_count: i32,
    live2_x: i32,
    live2_o: i32,
    live1_x: i32,
    live1_o: i32,
    owned_degree_x: i32,
    owned_degree_o: i32,
    span_x: i32,
    span_o: i32,
    empty_cells: [Move; MAX_CELLS],
    empty_count: usize,
    ux: [i32; MAX_CELLS],
    uo: [i32; MAX_CELLS],
    vx: [i32; MAX_CELLS],
    vo: [i32; MAX_CELLS],
    wx: [i32; MAX_CELLS],
    wo: [i32; MAX_CELLS],
}

fn analyze(board: &Board, patterns: &[Pattern]) -> Analysis {
    let cells = board.get_cells();
    let n = cells.len();

    let mut analysis = Analysis {
        n,
        playable: 0,
        score_diff: 0,
        x_count: 0,
        o_count: 0,
        live2_x: 0,
        live2_o: 0,
        live1_x: 0,
        live1_o: 0,
        owned_degree_x: 0,
        owned_degree_o: 0,
        span_x: 0,
        span_o: 0,
        empty_cells: [(0, 0); MAX_CELLS],
        empty_count: 0,
        ux: [0; MAX_CELLS],
        uo: [0; MAX_CELLS],
        vx: [0; MAX_CELLS],
        vo: [0; MAX_CELLS],
        wx: [0; MAX_CELLS],
        wo: [0; MAX_CELLS],
    };

    for r in 0..n {
        for c in 0..n {
            match cells[r][c] {
                Cell::Wall => {}
                Cell::Empty => {
                    analysis.playable += 1;
                    analysis.empty_cells[analysis.empty_count] = (r, c);
                    analysis.empty_count += 1;
                }
                Cell::X => {
                    analysis.playable += 1;
                    analysis.x_count += 1;
                }
                Cell::O => {
                    analysis.playable += 1;
                    analysis.o_count += 1;
                }
            }
        }
    }

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
            analysis.score_diff += 1;
            continue;
        }
        if o == 3 {
            analysis.score_diff -= 1;
            continue;
        }

        for &(r, c) in pattern.iter() {
            match cells[r][c] {
                Cell::X => analysis.owned_degree_x += 1,
                Cell::O => analysis.owned_degree_o += 1,
                _ => {}
            }
        }

        if o == 0 {
            analysis.span_x += x * x;
            if x == 2 {
                analysis.span_x += 2;
                analysis.live2_x += 1;
                for i in 0..empty_count {
                    let idx = empties[i];
                    analysis.ux[idx] += 1;
                    analysis.wx[idx] += 1;
                }
            } else if x == 1 {
                analysis.live1_x += 1;
                for i in 0..empty_count {
                    let idx = empties[i];
                    analysis.vx[idx] += 1;
                    analysis.wx[idx] += 1;
                }
            } else {
                for i in 0..empty_count {
                    analysis.wx[empties[i]] += 1;
                }
            }
        }

        if x == 0 {
            analysis.span_o += o * o;
            if o == 2 {
                analysis.span_o += 2;
                analysis.live2_o += 1;
                for i in 0..empty_count {
                    let idx = empties[i];
                    analysis.uo[idx] += 1;
                    analysis.wo[idx] += 1;
                }
            } else if o == 1 {
                analysis.live1_o += 1;
                for i in 0..empty_count {
                    let idx = empties[i];
                    analysis.vo[idx] += 1;
                    analysis.wo[idx] += 1;
                }
            } else {
                for i in 0..empty_count {
                    analysis.wo[empties[i]] += 1;
                }
            }
        }
    }

    analysis
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
