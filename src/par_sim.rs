use crate::grid::{self, Grid};
use crate::rules::next_cell_state;
use std::{thread, time::Duration};

pub fn step(grid: &mut Grid){
    let new_rust: Vec<Vec<_>> = (0..grid.height)
        .map(|y|(0..grid.width)
            .map(|x| next_cell_state(grid, x, y))
            .collect())
        .collect();

    grid.rust = new_rust;
}

pub fn run(grid: &mut Grid, steps: usize, delay_ms: u64){
    for i in 0..steps {
        clear_terminal();
        println!("Step {}/{}", i + 1, steps);
        println!("  . = Clean  # = Surface rust  @ = Heavy rust  X = Rotten\n");
        grid.display();
        step(grid);
        thread::sleep(Duration::from_millis(delay_ms));
    }

    clear_terminal();
    println!("Step {}/{} — simulation complete\n", steps, steps);
    grid.display();
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}