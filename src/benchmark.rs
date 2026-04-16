use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;
use csv::Writer;
use crate::grid::Grid;
use crate::seq_sim;
use crate::par_sim;

pub struct BenchmarkResult {
    pub mode: String,
    pub grid_width: usize,
    pub grid_height: usize,
    pub steps: usize,
    pub run: usize,
    pub duration_ms: f64,
}

// runs one simulation silently and returns how long it took
fn time_run(mode: &str, width: usize, height: usize, steps: usize) -> Duration {
    let mut grid = Grid::new_with_params(width, height, 0.7, 0.7);

    let start = Instant::now();
    match mode {
        "sequential" => seq_sim::step_n(&mut grid, steps),
        "parallel"   => par_sim::step_n(&mut grid, steps),
        _            => panic!("Unknown mode"),
    }
    start.elapsed()
}

pub fn run_benchmark(
    sizes: &[(usize, usize)],
    steps: usize,
    runs_per_config: usize,
    output_path: &str,
) {
    fs::create_dir_all(Path::new(output_path).parent().unwrap_or(Path::new(".")))
        .expect("Failed to create benchmark output directory");

    let mut writer = Writer::from_path(output_path)
        .expect("Failed to create CSV file");

    writer.write_record(&["mode", "width", "height", "steps", "run", "duration_ms"])
        .unwrap();

    let modes = ["sequential", "parallel"];

    for &(width, height) in sizes {
        for &mode in &modes {
            println!("Benchmarking {} on {}x{} grid ({} runs)...",
                mode, width, height, runs_per_config);

            for run in 0..runs_per_config {
                let duration = time_run(mode, width, height, steps);
                let duration_ms = duration.as_secs_f64() * 1000.0;

                writer.write_record(&[
                    mode,
                    &width.to_string(),
                    &height.to_string(),
                    &steps.to_string(),
                    &run.to_string(),
                    &format!("{:.3}", duration_ms),
                ]).unwrap();

                println!("  run {}/{}: {:.1}ms", run + 1, runs_per_config, duration_ms);
            }
        }
        writer.flush().unwrap();
    }

    println!("\nBenchmark complete. Results saved to: {}", output_path);
}