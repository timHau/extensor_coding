use plotters::prelude::*;
use std::ops::Range;

pub fn join_runs(runs: Vec<Vec<u128>>) -> Vec<f64> {
    let mut res = vec![0.0; runs[0].len()];

    for tv in runs.iter() {
        for (i, v) in tv.iter().enumerate() {
            res[i] += *v as f64;
        }
    }

    res.iter()
        .map(|t| *t / (runs.len() as f64))
        .collect::<Vec<f64>>()
}

pub fn plot_results(
    title: &str,
    axis: ((&str, Range<f32>), (&str, Range<f32>)),
    path: &str,
    results: &Vec<(String, RGBColor, Vec<f64>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (1024, 640)).into_drawing_area();
    root.fill(&WHITE)?;

    let (x, y) = axis;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 20).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(x.1, y.1)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .y_desc(x.0)
        .x_desc(y.0)
        .light_line_style(&WHITE.mix(0.8))
        .draw()?;

    for (name, col, res) in results.iter() {
        chart
            .draw_series(LineSeries::new(
                (0..res.len()).map(|i| (i as f32, res[i] as f32)),
                col.clone().to_owned(),
            ))?
            .label(name)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], col.clone().to_owned())
            });
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.5))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
