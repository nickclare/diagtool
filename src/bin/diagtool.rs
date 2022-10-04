use diagtool::solver::{fontmetrics::*, *};

fn main() -> anyhow::Result<()> {
    let mc = MetricsCalculator::new()?;
    let width = mc.compute_width("ffk")?;
    println!("calculated width is: {width}");

    Ok(())
}
