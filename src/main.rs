mod grid;
mod rules;
mod output;
mod par_sim;
mod seq_sim;

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
    random: bool 
}

fn main() {
    let args = Args::parse();

    println!("Rustin in rust starting...");
    println!("Grid: {}x{}  Steps: {}  Output: {}\n",
        args.width, args.height, args.steps, args.output);

    let mut grid = if args.random {
        Grid::new_random(args.width, args.height)
    } else {
        Grid::new_with_params(args.width, args.height, args.humidity, args.oxygen)
    };

    //par_sim::run(&mut grid, 50, 100);
    seq_sim::run(&mut grid, args.steps, args.delay, &args.output);
}
