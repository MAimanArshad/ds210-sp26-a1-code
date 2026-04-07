use clap::Parser;
use tic_tac_toe_stencil::agents::*;
use tic_tac_toe_stencil::game_loop;
use tic_tac_toe_stencil::player::Player;

use crate::args::{Agents, Args};
use tic_tac_toe_3x3::layout::Layout3x3;
use tic_tac_toe_3x3::solution::agent::SolutionAgent;

mod args;

// Time limit for solution to make a move
const TIME_LIMIT: u64 = 1000;

fn main() {
    // Parse arguments.
    let args = Args::parse();
    
    let x_agent = args.get_agent(Player::X);
    let o_agent = args.get_agent(Player::O);
    let layout = Layout3x3 {};

    let _ = match (x_agent, o_agent) {
        // First vs the world.
        (Agents::First, Agents::First) => game_loop::<_, FirstMoveAgent, FirstMoveAgent>(layout, TIME_LIMIT, false),
        (Agents::First, Agents::Manual) => game_loop::<_, FirstMoveAgent, ManualAgent>(layout, TIME_LIMIT, false),
        (Agents::First, Agents::Random) => game_loop::<_, FirstMoveAgent, RandomAgent>(layout, TIME_LIMIT, false),
        (Agents::First, Agents::Solution) => game_loop::<_, FirstMoveAgent, SolutionAgent>(layout, TIME_LIMIT, false),
        (Agents::First, Agents::Test) => game_loop::<_, FirstMoveAgent, TestAgent>(layout, TIME_LIMIT, false),
        // Manual vs the world.
        (Agents::Manual, Agents::First) => game_loop::<_, ManualAgent, FirstMoveAgent>(layout, TIME_LIMIT, false),
        (Agents::Manual, Agents::Manual) => game_loop::<_, ManualAgent, ManualAgent>(layout, TIME_LIMIT, false),
        (Agents::Manual, Agents::Random) => game_loop::<_, ManualAgent, RandomAgent>(layout, TIME_LIMIT, false),
        (Agents::Manual, Agents::Solution) => game_loop::<_, ManualAgent, SolutionAgent>(layout, TIME_LIMIT, false),
        (Agents::Manual, Agents::Test) => game_loop::<_, ManualAgent, TestAgent>(layout, TIME_LIMIT, false),
        // Random vs the world.
        (Agents::Random, Agents::First) => game_loop::<_, RandomAgent, FirstMoveAgent>(layout, TIME_LIMIT, false),
        (Agents::Random, Agents::Manual) => game_loop::<_, RandomAgent, ManualAgent>(layout, TIME_LIMIT, false),
        (Agents::Random, Agents::Random) => game_loop::<_, RandomAgent, RandomAgent>(layout, TIME_LIMIT, false),
        (Agents::Random, Agents::Solution) => game_loop::<_, RandomAgent, SolutionAgent>(layout, TIME_LIMIT, false),
        (Agents::Random, Agents::Test) => game_loop::<_, RandomAgent, TestAgent>(layout, TIME_LIMIT, false),
        // Solution vs the world.
        (Agents::Solution, Agents::First) => game_loop::<_, SolutionAgent, FirstMoveAgent>(layout, TIME_LIMIT, false),
        (Agents::Solution, Agents::Manual) => game_loop::<_, SolutionAgent, ManualAgent>(layout, TIME_LIMIT, false),
        (Agents::Solution, Agents::Random) => game_loop::<_, SolutionAgent, RandomAgent>(layout, TIME_LIMIT, false),
        (Agents::Solution, Agents::Solution) => game_loop::<_, SolutionAgent, SolutionAgent>(layout, TIME_LIMIT, false),
        (Agents::Solution, Agents::Test) => game_loop::<_, SolutionAgent, TestAgent>(layout, TIME_LIMIT, false),
        // Test vs the world.
        (Agents::Test, Agents::First) => game_loop::<_, TestAgent, FirstMoveAgent>(layout, TIME_LIMIT, false),
        (Agents::Test, Agents::Manual) => game_loop::<_, TestAgent, ManualAgent>(layout, TIME_LIMIT, false),
        (Agents::Test, Agents::Random) => game_loop::<_, TestAgent, RandomAgent>(layout, TIME_LIMIT, false),
        (Agents::Test, Agents::Solution) => game_loop::<_, TestAgent, SolutionAgent>(layout, TIME_LIMIT, false),
        (Agents::Test, Agents::Test) => game_loop::<_, TestAgent, TestAgent>(layout, TIME_LIMIT, false),
    };
}
