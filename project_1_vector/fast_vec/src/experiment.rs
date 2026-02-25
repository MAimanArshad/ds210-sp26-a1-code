use std::time::Instant;
use slow_vec::SlowVec;
use fast_vec::FastVec;

use plotters::prelude::*;
use plotters::style::full_palette::ORANGE;

fn fast_vec() -> Vec<(f32, f32)> {
    let mut result = Vec::new();
    let mut fast_vec = FastVec::from_vec(vec![10; 50_000]);
    for i in 50_000..=1_000_000 {
        if i % 50_000 == 0 {
            let start = Instant::now();
            fast_vec.push(20);
            let elapsed = start.elapsed();
            result.push((i as f32, elapsed.as_micros() as f32));
        } else {
            fast_vec.push(30);
        }
    }

    return result;
}

fn slow_vec() -> Vec<(f32, f32)> {
    let RUNS = 3;
    let mut result = Vec::new();
    for i in 50_000..=1_000_000 {
        if i % 50_000 == 0 {
            let mut sum = 0f32;
            for _ in 0..RUNS {
                let mut slow_vec = SlowVec::from_vec(vec![10; i]);
                let start = Instant::now();
                slow_vec.push(20);
                let elapsed = start.elapsed();
                sum += (elapsed.as_micros() / 100) as f32;
            }
            result.push((i as f32, sum / RUNS as f32));
        }
    }

    return result;
}

// Experiment main loop.
fn main() {
    let fast_vec = fast_vec();
    let slow_vec = slow_vec();

    let root = BitMapBackend::new("plot.png", (600, 600)).into_drawing_area();
    let root = root.margin(10, 10, 10, 10);
    root.fill(&WHITE).unwrap();

    let max_fast_y = fast_vec.iter().map(|tup| tup.1 as u32).max().unwrap();
    let max_slow_y = slow_vec.iter().map(|tup| tup.1 as u32).max().unwrap();
    let max_y = (std::cmp::max(max_fast_y, max_slow_y) + 40) as f32;

    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Number of guesses to find solution", ("sans-serif", 30).into_font())
        // Set the size of the label region
        .x_label_area_size(50)
        .y_label_area_size(70)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..1100000f32, 0f32..max_y).unwrap();

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

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(fast_vec, style1.clone()).point_size(2)).unwrap()
        .label("FastVec (micro)")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style1));
    chart.draw_series(LineSeries::new(slow_vec, style2.clone()).point_size(2)).unwrap()
        .label("SlowVec\n(100 micro)")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style2));


    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft) // Move to top center
        .border_style(&BLACK)
        .label_font(("sans-serif", 30))
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
    root.present().unwrap();
}