use tic_tac_toe_5x5::layout::Layout5x5;
use tic_tac_toe_5x5::solution::agent::SolutionAgent;
use tic_tac_toe_stencil::agents::TestAgent;
use tic_tac_toe_stencil::{game_loop, Outcome};

const TIME_LIMIT: u64 = 2000;

#[test]
fn beats_baseline_as_x_random_5() {
    let mut count_win = 0;
    for _ in 0..10 {
        let outcome = game_loop::<_, SolutionAgent, TestAgent>(Layout5x5::Random(5), TIME_LIMIT, true);
        match outcome {
            Outcome::X => count_win += 1,
            _ => {},
        }
    }
    println!("Your solution won {}/10 times", count_win);
    assert!(count_win >= 5, "Your solution should win more than half the times against random as X");
}

#[test]
fn beats_baseline_as_o_random_5() {
    let mut count_win = 0;
    for _ in 0..10 {
        let outcome = game_loop::<_, TestAgent, SolutionAgent>(Layout5x5::Random(5), TIME_LIMIT, true);
        match outcome {
            Outcome::O => count_win += 1,
            _ => {},
        }
    }
    println!("Your solution won {}/10 times", count_win);
    assert!(count_win >= 5, "Your solution should win more than half the times against random as O");
}