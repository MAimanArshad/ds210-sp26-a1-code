// This handles reading the command line arguments for configuring the board layout
// and which agents to use for each player.
use clap::{Parser, ValueEnum};

use tic_tac_toe_stencil::player::Player;

use tic_tac_toe_5x5::layout::Layout5x5;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    // X player solution
    #[arg(short, long, value_enum)]
    pub x: Agents,

    // Y player solution
    #[arg(short, long, value_enum)]
    pub o: Agents,

    // Layout
    #[arg(short, long)]
    layout: String,
}

impl Args {
    pub fn get_layout(&self) -> Layout5x5 {
        if self.layout == "3x3" {
            return Layout5x5::ThreeByThree;
        } else if self.layout == "0" {
            return Layout5x5::Empty;
        } else {
            return Layout5x5::Random(self.layout.parse().unwrap());
        }
    }

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
