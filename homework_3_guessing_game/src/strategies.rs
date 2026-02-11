use rand::random_range;
use crate::player::{Player, PlayerTrait};

pub trait Strategy {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32;
}


// Some strategies we provided to you as examples.
// These are not good strategies!
pub struct BadStrategy {}
pub struct RandomStrategy {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for BadStrategy {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        if player.ask_if_equal(min) {
            return min;
        } else {
            return max - 1;
        }
    }
}

// Another Terrible strategy: randomly guess until you find the right answer.
impl Strategy for RandomStrategy {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        loop {
            let random_guess = random_range(min..max);
            if player.ask_if_equal(random_guess) {
                return random_guess;
            }
        }
    }
}
