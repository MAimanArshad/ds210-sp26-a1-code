use tic_tac_toe_3x3::layout::Layout3x3;
use tic_tac_toe_3x3::solution::agent::SolutionAgent;
use tic_tac_toe_stencil::agents::RandomAgent;
use tic_tac_toe_stencil::{game_loop, Outcome};

const TIME_LIMIT: u64 = 1000;

#[test]
fn beats_random_as_x() {
    let mut count_win = 0;
    for _ in 0..10 {
        let outcome = game_loop::<_, SolutionAgent, RandomAgent>(Layout3x3 {}, TIME_LIMIT, true);
        assert_ne!(outcome, Outcome::O, "Your solution should never lose!");
        match outcome {
            Outcome::X => count_win += 1,
            _ => {},
        }
    }
    println!("Your solution won {}/10 times", count_win);
    assert!(count_win > 8, "Your solution should win more than 8 times out of 10 against random as X");
}

#[test]
fn beats_random_as_o() {
    let mut count_win = 0;
    for _ in 0..100 {
        let outcome = game_loop::<_, RandomAgent, SolutionAgent>(Layout3x3 {}, TIME_LIMIT, true);
        assert_ne!(outcome, Outcome::X, "Your solution should never lose!");
        match outcome {
            Outcome::O => count_win += 1,
            _ => {},
        }
    }
    println!("Your solution won {}/100 times", count_win);
    assert!(count_win >= 50, "Your solution should win more than half the times against random as O");
}