use std::io;

pub trait PlayerTrait {
    /**
     * This function compares the guess to the number that the player has in mind.
     * The function should return true if the guess and number are equal.
     * The function should return false otherwise.
     */
    fn ask_if_equal(&mut self, guess: u32) -> bool;

    /**
     * This function compares the guess to the number that the player has in mind.
     * The function should return one of these three cases:
     * 0 if the number == guess
     * -1 if the number < guess
     * 1 if the number > guess
     */
    fn ask_to_compare(&mut self, guess: u32) -> i32;
}

// A player of an unknown implementation (could be human or computer).
pub struct Player {
    player: Box<dyn PlayerTrait>,
    steps: u32,
}
impl Player {
    pub fn new<P: PlayerTrait + 'static>(player: P) -> Player {
        Player {
            player: Box::new(player),
            steps: 0
        }
    }
    pub fn steps(&self) -> u32 {
        self.steps
    }
}
impl PlayerTrait for Player {
    fn ask_if_equal(&mut self, guess: u32) -> bool {
        self.steps += 1;
        self.player.ask_if_equal(guess)
    }
    fn ask_to_compare(&mut self, guess: u32) -> i32 {
        self.steps += 1;
        self.player.ask_to_compare(guess)
    }
}

// This type represents a human player.
pub struct HumanPlayer {}

// The human player works by asking the user to answer each question!
impl PlayerTrait for HumanPlayer {
    fn ask_if_equal(&mut self, guess: u32) -> bool {
        // Ask the human a question
        println!("Is your number equal to {guess} [y/n]? ");

        // Get their answer
        let mut answer = String::new();
        loop {
            io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

            let answer = answer.trim();
            if answer == "y" {
                return true;
            } else if answer == "n" {
                return false;
            } else {
                println!("Invalid answer, please type in y (yes; equal) or n (no; not equal): ")
            }
        }
    }
    fn ask_to_compare(&mut self, guess: u32) -> i32 {
        // Ask the human a question
        println!("Is your number equal, less than, or greater than {guess} [e/l/g]? ");

        // Get their answer
        let mut answer = String::new();
        loop {
            io::stdin()
                .read_line( & mut answer)
                .expect("Failed to read line");

            let answer = answer.trim();
            if answer == "e" {
                return 0;
            } else if answer == "l" {
                return -1;
            } else if answer == "g" {
                return 1;
            } else {
                println!("Invalid answer, please type in e (equal), l (less than) or g (greater than): ")
            }
        }
    }
}