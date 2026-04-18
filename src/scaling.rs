use std::time::Instant;
use std::fs;
use std::path::Path;
use csv::Writer;
use rayon::ThreadPoolBuilder;
use crate::grid::Grid;
use crate::{par_sim, seq_sim};


pub struct ScalingResult {
    pub experiment: String,
    pub threads: usize,
    pub width: usize,
    pub height: usize,
    pub steps: usize,
    pub run: usize,
    pub duration_ms: f64
}

fn time_seq(width: usize, height: usize, steps: usize) -> f64 {
    let mut grid = Grid::new_with_params(width, height, 0.7, 0.7);
    let start = Instant::now();
    seq_sim::step_n(&mut grid, steps);
    start.elapsed().as_secs_f64() * 1000.0
}


fn time_par(width: usize, height: usize, steps: usize, threads: usize) -> f64 {
    // build a rayon thread pool with exactly `threads` threads
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("Failed to build thread pool");

    let mut grid = Grid::new_with_params(width, height, 0.7, 0.7);
    let start = Instant::now();
    pool.install(|| par_sim::step_n(&mut grid, steps));
    start.elapsed().as_secs_f64() * 1000.0
}

pub fn run_strong_scaling(
    width: usize,
    height: usize,
    steps: usize,
    max_threads: usize,
    runs: usize,
    output_path: &str,
) {
    fs::create_dir_all(Path::new(output_path).parent().unwrap_or(Path::new(".")))
        .expect("Failed to create output directory");

    let mut writer = Writer::from_path(output_path)
        .expect("Failed to create CSV");

    writer.write_record(&[
        "experiment", "threads", "width", "height", "steps", "run", "duration_ms"
    ]).unwrap();

    println!("\nStrong scaling — fixed grid {}x{}, {} steps", width, height, steps);
    println!("(same problem size, more threads)\n");

    // sequential baseline — recorded as threads=1
    for run in 0..runs {
        let ms = time_seq(width, height, steps);
        writer.write_record(&[
            "strong_sequential",
            "1",
            &width.to_string(),
            &height.to_string(),
            &steps.to_string(),
            &run.to_string(),
            &format!("{:.3}", ms),
        ]).unwrap();
    }
    println!("  sequential baseline: done ({} runs)", runs);

    // parallel with 2..=max_threads
    for threads in 2..=max_threads {
        for run in 0..runs {
            let ms = time_par(width, height, steps, threads);
            writer.write_record(&[
                "strong_parallel",
                &threads.to_string(),
                &width.to_string(),
                &height.to_string(),
                &steps.to_string(),
                &run.to_string(),
                &format!("{:.3}", ms),
            ]).unwrap();
        }
        println!("  {} threads: done ({} runs)", threads, runs);
        writer.flush().unwrap();
    }

    println!("Strong scaling results saved to: {}", output_path);
}

pub fn run_weak_scaling(
    base_width: usize,
    base_height: usize,
    steps: usize,
    max_threads: usize,
    runs: usize,
    output_path: &str,
) {
    fs::create_dir_all(Path::new(output_path).parent().unwrap_or(Path::new(".")))
        .expect("Failed to create output directory");

    let mut writer = Writer::from_path(output_path)
        .expect("Failed to create CSV");

    writer.write_record(&[
        "experiment", "threads", "width", "height", "steps", "run", "duration_ms"
    ]).unwrap();

    println!("\nWeak scaling — base grid {}x{}, {} steps", base_width, base_height, steps);
    println!("(problem grows with thread count, constant work per thread)\n");

    // sequential baseline at base size
    for run in 0..runs {
        let ms = time_seq(base_width, base_height, steps);
        writer.write_record(&[
            "weak_sequential",
            "1",
            &base_width.to_string(),
            &base_height.to_string(),
            &steps.to_string(),
            &run.to_string(),
            &format!("{:.3}", ms),
        ]).unwrap();
    }
    println!("  1 thread ({}x{}): done", base_width, base_height);

    // for weak scaling: multiply height by thread count to keep work-per-thread constant
    for threads in 2..=max_threads {
        let width  = base_width;
        let height = base_height * threads; // scale problem with threads

        for run in 0..runs {
            let ms = time_par(width, height, steps, threads);
            writer.write_record(&[
                "weak_parallel",
                &threads.to_string(),
                &width.to_string(),
                &height.to_string(),
                &steps.to_string(),
                &run.to_string(),
                &format!("{:.3}", ms),
            ]).unwrap();
        }
        println!("  {} threads ({}x{}): done", threads, width, height);
        writer.flush().unwrap();
    }

    println!("Weak scaling results saved to: {}", output_path);
}