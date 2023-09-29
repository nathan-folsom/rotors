use crate::constants::*;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub fn init_cells() -> Vec<(f64, f64)> {
    let mut cells = vec![];
    let mut rng = thread_rng();
    let max = 2f64 * PI;
    for _ in 0..CELL_COUNT {
        let rads = rng.gen_range(0f64..max);
        let (x, y) = rads_to_vector(rads);
        cells.push((x, y))
    }
    cells
}

pub fn init_particles() -> Vec<(f64, f64)> {
    let mut particles = vec![];
    let mut rng = thread_rng();
    for _ in 0..CELL_COUNT {
        particles.push((
            rng.gen_range(0f64..=(WINDOW_WIDTH as f64)),
            rng.gen_range(0f64..=(WINDOW_HEIGHT as f64)),
        ))
    }
    particles
}

pub fn get_cell_line(cell: &(f64, f64), row: usize, col: usize) -> ((f64, f64), (f64, f64)) {
    let (center_x, center_y) = get_cell_center(row, col);

    let start = (center_x - cell.0, center_y - cell.1);
    let end = (center_x + cell.0, center_y + cell.1);
    (start, end)
}

pub fn rads_to_vector(rads: f64) -> (f64, f64) {
    // cos(rads) = x / CELL_WIDTH
    let x = rads.cos() * (CELL_WIDTH / 2f64);

    // sin(rads) = y / CELL_WIDTH
    let y = rads.sin() * (CELL_WIDTH / 2f64);
    (x, y)
}

pub fn get_cell_center(row: usize, col: usize) -> (f64, f64) {
    let center_x = col as f64 * CELL_WIDTH + (CELL_WIDTH / 2f64);
    let center_y = row as f64 * CELL_WIDTH + (CELL_WIDTH / 2f64);
    (center_x, center_y)
}

pub fn get_influence_cell(x: &f64, y: &f64) -> usize {
    let col = (x / CELL_WIDTH).floor().min((GRID_SIZE - 1) as f64) as usize;
    let row = (y / CELL_WIDTH).floor().min((GRID_SIZE - 1) as f64) as usize;
    row * GRID_SIZE + col
}
