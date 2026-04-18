use std::fs;
use std::path::Path;
use serde_json::json;
use crate::grid::{CellState, Grid};

impl CellState {
    pub fn to_str(&self) -> &str {
        match self {
            CellState::Clean => "clean",
            CellState::SurfaceRust => "surface",
            CellState::HeavyRust => "heavy",
            CellState::Rotten => "rotten"
        }
    }
}

pub fn save_step(grid: &Grid, step: usize, output_dir: &str) {
    fs::create_dir_all(output_dir).expect("Failed to create dir");

    let cells:  Vec<Vec<&str>> = grid.rust.iter()
        .map(|row| row.iter().map(|c| c.to_str()).collect())
        .collect();

    let data = json!({
        "step": step,
        "width": grid.width,
        "height": grid.height,
        "grid": cells,
        "humidity": grid.humidity,
        "oxygen": grid.oxygen
    });

    let path = Path::new(output_dir).join(format!("step_{:04}.json", step));
    fs::write(path, serde_json::to_string_pretty(&data).unwrap())
        .expect("Failed to write steps");
}

pub fn save_summary(total_steps: usize, width: usize, height: usize, output_dir: &str) {
    let data = json!({
        "total_steps": total_steps,
        "width": width,
        "height": height,
        "output_dir": output_dir
    });

    let path = Path::new(output_dir).join("summary.json");
    fs::write(path, serde_json::to_string_pretty(&data).unwrap())
        .expect("Failed to write summary")
}