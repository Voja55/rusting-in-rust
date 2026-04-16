use crate::grid::Grid;
use crate::output;
use crate::rules::next_cell_state;
use std::{thread, time::Duration};

pub fn step(grid: &mut Grid) {
    let new_rust: Vec<Vec<_>> = (0..grid.height)
        .map(|y| (0..grid.width)
            .map(|x| next_cell_state(grid, x, y))
            .collect())
        .collect();

    grid.rust = new_rust;
}

pub fn run(grid: &mut Grid, steps: usize, delay_ms: u64, output_dir: &str) {
    for i in 0..steps {
        clear_terminal();
        println!("Step {}/{}", i + 1, steps);
        grid.display();

        output::save_step(grid, i, output_dir);
        step(grid);

        thread::sleep(Duration::from_millis(delay_ms));
    }

    output::save_step(grid, steps, output_dir);
    output::save_summary(steps, grid.width, grid.height, output_dir);

    clear_terminal();
    println!("Step {}/{} — simulation complete\n", steps, steps);
    println!("Output saved to: {}\n", output_dir);
    grid.display();
}

pub fn step_n(grid: &mut Grid, steps: usize) {
    for _ in 0..steps {
        step(grid);
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}