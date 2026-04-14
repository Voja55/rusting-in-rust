mod grid;
mod rules;
mod output;
mod par_sim;
mod seq_sim;

use std::slice::GetDisjointMutError;

use grid::Grid;

fn main() {
    println!("Rustin in rust starting...");

    let grid = Grid::new_random(20, 10);

    grid.display();
}
