use extensor_coding::graph::Graph;
use plotters::prelude::*;
use rand::distributions::Bernoulli;
use rand::Rng;
use std::ops::Range;

pub fn rand_graph(n: usize, p: f64) -> Graph {
    let bernoulli = Bernoulli::new(p).unwrap();
    let rand: Vec<u8> = rand::thread_rng()
        .sample_iter(&bernoulli)
        .take(n * n)
        .map(|v| if v { 1 } else { 0 })
        .collect();
    Graph::from(n, rand)
}

#[allow(dead_code)]
pub fn join_runs(runs: Vec<Vec<f64>>) -> Vec<f64> {
    let mut res = vec![0.0; runs[0].len()];

    for tv in runs.iter() {
        for (i, v) in tv.iter().enumerate() {
            res[i] += *v;
        }
    }

    res.iter()
        .map(|t| *t / (runs.len() as f64))
        .collect::<Vec<f64>>()
}

#[allow(dead_code)]
pub fn plot_results(
    title: &str,
    axis: ((&str, Range<f32>), (&str, Range<f32>)),
    offset: usize,
    path: &str,
    results: &Vec<(String, RGBColor, Vec<Vec<f64>>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let run_path = format!("{}.png", path);
    let run_root = BitMapBackend::new(&run_path, (1024, 640)).into_drawing_area();
    run_root.fill(&WHITE)?;

    let (x, y) = axis;
    let (x_name, x_range) = x;
    let (y_name, y_range) = y;

    let mut run_chart = ChartBuilder::on(&run_root)
        .caption(title, ("sans-serif", 20).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

    run_chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc(x_name)
        .y_desc(y_name)
        .light_line_style(&WHITE.mix(0.8))
        .draw()?;

    for (name, col, res) in results.iter() {
        let run = join_runs(res.to_vec());
        run_chart
            .draw_series(LineSeries::new(
                (0..run.len()).map(|i| ((offset + i) as f32, run[i] as f32)),
                col.clone().to_owned(),
            ))?
            .label(name)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], col.clone().to_owned())
            });
    }

    run_chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.5))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
