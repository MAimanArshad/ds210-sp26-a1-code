use std::io;
use clap::{Parser, ValueEnum};
use crate::part1::Part1;
use crate::part2::Part2;
use crate::player::{HumanPlayer, Player};
use crate::strategies::{Strategy, BadStrategy, RandomStrategy};

mod strategies;
mod player;
mod part1;
mod part2;
mod part3;

#[derive(ValueEnum, Clone, Debug)]
enum StrategyEnum {
    Bad,
    Random,
    Part1,
    Part2,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    strategy: StrategyEnum,
    #[arg(long, default_value_t = 0)]
    min: u32,
    #[arg(long, default_value_t = 16)]
    max: u32,
}

fn main() {
    // Read the command line arguments.
    let cli = Args::parse();
    let strategy = cli.strategy;
    let min = cli.min;
    let max = cli.max;

    // Create a human player.
    let mut player = Player::new(HumanPlayer {});

    println!("Choose a number between {min} (inclusive) and {max} (exclusive).");
    println!("Write down your number on a piece of paper so you remember what it is.");
    println!("Do not tell me what your number is. I will try to guess it.");
    println!("Did you choose a number? Hit enter when you have! ");

    // Wait until user types in enter.
    let _ = String::new();
    io::stdin().read_line(&mut String::new()).unwrap();

    // Game starts.
    println!("Commencing game!");
    let answer = match strategy {
        StrategyEnum::Bad => BadStrategy::guess_the_number(&mut player, min, max),
        StrategyEnum::Random => RandomStrategy::guess_the_number(&mut player, min, max),
        StrategyEnum::Part1 => Part1::guess_the_number(&mut player, min, max),
        StrategyEnum::Part2 => Part2::guess_the_number(&mut player, min, max),
    };

    // Print results!
    println!("Final answer: {answer}");
    println!("Took {} steps", player.steps());
}
