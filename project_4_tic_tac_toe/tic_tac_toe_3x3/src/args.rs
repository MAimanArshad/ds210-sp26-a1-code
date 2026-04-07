// This handles reading the command line arguments for configuring
// which agents to use for each player.
use clap::{Parser, ValueEnum};

use tic_tac_toe_stencil::player::Player;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    // X player solution
    #[arg(short, long, value_enum)]
    pub x: Agents,

    // Y player solution
    #[arg(short, long, value_enum)]
    pub o: Agents,
}

impl Args {
    pub fn get_agent(&self, player: Player) -> Agents {
        match player {
            Player::X => self.x,
            Player::O => self.o,
        }
    }
}

// The solution possibilities we have.
#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Agents {
    First,
    Random,
    Test,
    Solution,
    Manual,
}
