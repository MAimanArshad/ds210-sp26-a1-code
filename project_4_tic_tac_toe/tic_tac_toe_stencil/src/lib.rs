use std::process;
use std::thread;
use std::time::{Duration, SystemTime};

use crate::agents::SealedAgent;
use crate::board::Board;
use crate::layout::Layout;
use crate::player::Player;

pub mod agents;
pub mod board;
pub mod color;
pub mod layout;
pub mod player;

// How long it takes.
//const TIME_LIMIT: u64 = 1500;
const WAKE_UP_COUNT: u64 = 100;
const PRINTING_DELAY: u64 = 1000; // in ms

// Invoke the solution with a timer.
// If the solution exceeds the timer, it will be killed and the game will be forfeited.
fn invoke_agent<A: SealedAgent>(mut board: Board, player: Player, time_limit: u64) -> Result<(f32, usize, usize), String> {
    let handler = thread::spawn(move || {
        let timer = SystemTime::now();

        let (_, row, col) = A::solve(&mut board, player);

        let elapsed = timer.elapsed().unwrap();
        let time_in_seconds = (elapsed.as_millis() as f32) / 1000.0;

        return (time_in_seconds, row, col);
    });

    // Manual solution does not have a time limit.
    if A::manual() {
        return Ok(handler.join().unwrap());
    }

    // Sleep for time limit, periodically waking up to check if solution is done.
    // If it was never done by the end, kill it.
    let sleep_time = Duration::from_millis(time_limit / WAKE_UP_COUNT);
    for _ in 0..WAKE_UP_COUNT {
        thread::sleep(sleep_time);

        if handler.is_finished() {
            return Ok(handler.join().unwrap());
        }
    }

    return Err(format!(
        "{} Agent is taking too long. Game forfeited",
        player.to_string()
    ));
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    X,
    O,
    Draw
}
impl From<Player> for Outcome {
    fn from(value: Player) -> Self {
        match value {
            Player::X => Outcome::X,
            Player::O => Outcome::O,
        }
    }
}

// The game's main loop function.
pub fn game_loop<L: Layout, X: SealedAgent, O: SealedAgent>(layout: L, time_limit: u64, background: bool) -> Outcome {
    // Create the board.
    let mut board = Board::new(layout);
    println!("Game begins");
    println!("{board}");

    let mut max_x_time: f32 = 0.0;
    let mut max_o_time: f32 = 0.0;

    let mut player = Player::X;
    while !board.game_over() {
        let result = match player {
            Player::X => invoke_agent::<X>(board.clone(), player, time_limit),
            Player::O => invoke_agent::<O>(board.clone(), player, time_limit),
        };

        if result.is_err() {
            println!("{}", result.unwrap_err());
            return player.flip().into();
        }

        let (time, row, col) = result.unwrap();
        board.apply_move((row, col), player);
        println!("{board}");

        println!("Agent took {:.2} seconds to move", time);
        match player {
            Player::X => {
                if time > max_x_time {
                    max_x_time = time;
                }
            }
            Player::O => {
                if time > max_o_time {
                    max_o_time = time;
                }
            }
        }

        player = player.flip();
        if !background {
            thread::sleep(Duration::from_millis(PRINTING_DELAY));
        }
    }

    // Print stats about speed.
    println!("X's slowest move took {} seconds", max_x_time);
    println!("O's slowest move took {} seconds", max_o_time);

    // Print score.
    let score = board.score();
    if score > 0 {
        println!("X wins. Final score {}", score);
        return Outcome::X;
    } else if score < 0 {
        println!("O wins. Final score {}", score);
        return Outcome::O;
    } else {
        println!("Draw");
        return Outcome::Draw;
    }
}
