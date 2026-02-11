use crate::part1::Part1;
use crate::part2::Part2;
use crate::part3::SimulatedPlayer;
use crate::player::Player;
use crate::strategies::{Strategy, RandomStrategy};

use plotters::prelude::*;
use plotters::style::full_palette::ORANGE;

mod part1;
mod part2;
mod part3;
mod player;
mod strategies;

// The experiment uses Part1, Part2, and Random.
enum StrategyEnum {
    Part1,
    Part2,
    Random,
}
impl StrategyEnum {
    pub fn guess_the_number(&self, mut player: Player, min: u32, max: u32) -> u32 {
        match self {
            StrategyEnum::Part1 => {
                Part1::guess_the_number(&mut player, min, max);
            },
            StrategyEnum::Part2 => {
                Part2::guess_the_number(&mut player, min, max);
            },
            StrategyEnum::Random => {
                RandomStrategy::guess_the_number(&mut player, min, max);
            }
        }
        return player.steps();
    }
}

fn experiment(strategy: StrategyEnum, min: u32, max: u32) -> u32 {
    let options = [min, max-1, (min + max) / 2, (min + max) / 2 + (min + max) / 3];
    let mut max_steps = 0;

    for number in options {
        let player = Player::new(SimulatedPlayer::new(number));
        let steps = strategy.guess_the_number(player, min, max);
        if steps > max_steps {
            max_steps = steps;
        }
    }

    return max_steps;
}


// Experiment main loop.
fn main() {
    let mut random = Vec::new();
    let mut part1 = Vec::new();
    let mut part2 = Vec::new();

    for max in 1..=100 {
        let steps_random = experiment(StrategyEnum::Random, 0, max);
        let steps1 = experiment(StrategyEnum::Part1, 0, max);
        let steps2 = experiment(StrategyEnum::Part2, 0, max);
        random.push((max as f32, steps_random as f32));
        part1.push((max as f32, steps1 as f32));
        part2.push((max as f32, steps2 as f32));
    }

    let root = BitMapBackend::new("plot.png", (600, 600)).into_drawing_area();
    let root = root.margin(10, 10, 10, 10);
    root.fill(&WHITE).unwrap();

    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Number of guesses to find solution", ("sans-serif", 30).into_font())
        // Set the size of the label region
        .x_label_area_size(50)
        .y_label_area_size(70)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..103f32, 0f32..103f32).unwrap();

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(10)
        .x_label_style(("sans-serif", 25))
        .y_labels(20)
        .y_label_style(("sans-serif", 25))
        // We can also change the format of the label text
        //.y_label_formatter(&|x| format!("{:}", x))
        .draw().unwrap();

    let style1 = ShapeStyle::from(&ORANGE).stroke_width(1).filled();
    let style2 = ShapeStyle::from(&BLUE).stroke_width(1).filled();
    let style3 = ShapeStyle::from(&BLACK).stroke_width(1).filled();

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(part1, style1.clone()).point_size(2)).unwrap()
        .label("Part 1")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style1));
    chart.draw_series(LineSeries::new(part2, style2.clone()).point_size(2)).unwrap()
        .label("Part 2")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style2));
    chart.draw_series(LineSeries::new(random, style3.clone()).point_size(2)).unwrap()
        .label("Random")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style3));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .label_font(("sans-serif", 30))
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    root.present().unwrap();
}
