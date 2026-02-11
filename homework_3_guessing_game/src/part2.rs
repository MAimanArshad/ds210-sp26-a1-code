use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut mid = (min + max) / 2;
        let guess = player.ask_to_compare(mid);
        if guess == 0{
            return mid;
        }
        else if guess == -1{
            return Self::guess_the_number(player, min, mid);
        }
        else{
            return Self::guess_the_number(player, mid, max);
        }
    }
}
