use tic_tac_toe_5x5::layout::Layout5x5;
use tic_tac_toe_5x5::solution::agent::SolutionAgent;
use tic_tac_toe_stencil::agents::FirstMoveAgent;
use tic_tac_toe_stencil::{game_loop, Outcome};

const TIME_LIMIT: u64 = 2000;

#[test]
fn beats_first_as_x_random_5() {
    let mut count_win = 0;
    for _ in 0..20 {
        let outcome = game_loop::<_, SolutionAgent, FirstMoveAgent>(Layout5x5::Random(5), TIME_LIMIT, true);
        match outcome {
            Outcome::X => count_win += 1,
            _ => {},
        }
    }
    println!("Your solution won {}/10 times", count_win);
    assert!(count_win >= 10, "Your solution should win more than half the times against first as X");
}

#[test]
fn beats_first_as_o_random_5() {
    let mut count_win = 0;
    for _ in 0..20 {
        let outcome = game_loop::<_, FirstMoveAgent, SolutionAgent>(Layout5x5::Random(5), TIME_LIMIT, true);
        match outcome {
            Outcome::O => count_win += 1,
            _ => {},
        }
    }
    println!("Your solution won {}/10 times", count_win);
    assert!(count_win >= 10, "Your solution should win more than half the times against first as O");
}

#[test]
fn beats_first_as_x_empty() {
    for _ in 0..3 {
        let outcome = game_loop::<_, SolutionAgent, FirstMoveAgent>(Layout5x5::Empty, TIME_LIMIT * 3, true);
        assert_eq!(outcome, Outcome::X, "Your solution should always win against first");
    }
}

#[test]
fn beats_first_as_o_empty() {
    for _ in 0..3 {
        let outcome = game_loop::<_, FirstMoveAgent, SolutionAgent>(Layout5x5::Empty, TIME_LIMIT * 3, true);
        assert_eq!(outcome, Outcome::O, "Your solution should always win against first");
    }
}