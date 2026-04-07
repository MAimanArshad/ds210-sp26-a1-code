use tic_tac_toe_3x3::layout::Layout3x3;
use tic_tac_toe_3x3::solution::agent::SolutionAgent;
use tic_tac_toe_stencil::agents::FirstMoveAgent;
use tic_tac_toe_stencil::{game_loop, Outcome};

const TIME_LIMIT: u64 = 1000;

#[test]
fn beats_first_as_x() {
    for _ in 0..3 {
        let outcome = game_loop::<_, SolutionAgent, FirstMoveAgent>(Layout3x3 {}, TIME_LIMIT, true);
        assert_eq!(outcome, Outcome::X, "Your solution should always win against first");
    }
}

#[test]
fn beats_first_as_o() {
    for _ in 0..3 {
        let outcome = game_loop::<_, FirstMoveAgent, SolutionAgent>(Layout3x3 {}, TIME_LIMIT, true);
        assert_eq!(outcome, Outcome::O, "Your solution should always win against first");
    }
}