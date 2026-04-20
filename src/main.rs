mod grid;
mod rules;
mod output;
mod par_sim;
mod seq_sim;
mod benchmark;
mod scaling;
mod visualizer;

use grid::Grid;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rusting-in-rust")]
#[command(about = "CA simulation of corrosion spread")]
struct Args {
    #[arg(short = 'W', long, default_value_t = 40)]
    width: usize,

    #[arg(short = 'H', long, default_value_t = 20)]
    height: usize,

    #[arg(short, long, default_value_t = 50)]
    steps: usize,

    #[arg(short, long, default_value_t = 100)]
    delay: u64,

    #[arg(short, long, default_value = "output/run")]
    output: String,

    #[arg(long, default_value_t = 0.5)]
    humidity: f32,

    #[arg(long, default_value_t = 0.5)]
    oxygen: f32,

    #[arg(short, long)]
    random: bool,

    #[arg(short, long)]
    parallel: bool,

    #[arg(short, long)]
    benchmark: bool,

    #[arg(long, default_value_t=10)]
    bench_runs: usize,

    #[arg(long)]
    scaling: bool,

    #[arg(long, default_value_t = 8)]
    max_threads: usize,

    #[arg(long, default_value_t = 10)]
    scaling_runs: usize,

    #[arg(short, long)]
    gui: bool
}
#[macroquad::main("Rusting in Rust")]
async fn main() {
    let args = Args::parse();

    if args.gui {
        visualizer::run_gui(args.width, args.height, args.humidity, args.oxygen).await;
        return;
    }

    if args.scaling {
        println!("Rusting in Rust — Scaling experiments\n");

        scaling::run_strong_scaling(
            400, 200,       // fixed grid size
            args.steps,
            args.max_threads,
            args.scaling_runs,
            "output/scaling/strong_scaling_rust.csv",
        );

        scaling::run_weak_scaling(
            100, 50,        // base grid size per thread
            args.steps,
            args.max_threads,
            args.scaling_runs,
            "output/scaling/weak_scaling_rust.csv",
        );

        return;
    }

    if args.benchmark {
        println!("Rustin in rust benchmark starting...");
        
        let sizes = vec![
            (50,  25),
            (100, 50),
            (200, 100),
            (400, 200),
            (600, 300),
        ];

        benchmark::run_benchmark(
            &sizes,
            args.steps,
            args.bench_runs,
            "output/benchmark/results.csv",
        );
        return;
    }

    let _mode = if args.parallel { "parallel" } else { "sequential" };

    println!("Rustin in rust starting...");
    println!("Grid: {}x{}  Steps: {}  Output: {}\n",
        args.width, args.height, args.steps, args.output);

    let mut grid = if args.random {
        Grid::new_random(args.width, args.height)
    } else {
        Grid::new_with_params(args.width, args.height, args.humidity, args.oxygen)
    };

    if args.parallel {
        par_sim::run(&mut grid, args.steps, args.delay, &args.output);
    } else {
        seq_sim::run(&mut grid, args.steps, args.delay, &args.output);
    }
}
