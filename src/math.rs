use crate::constants::*;
use rand::{thread_rng, Rng};

pub fn init_cells() -> Vec<(f64, f64)> {
    let mut cells = vec![];
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let step_1 = Box::new(move |x: f64, y: f64| {
                let transform = |n: f64| {
                    let sin = n.sin();
                    let adjusted = (sin + 1.0) * 0.5;
                    adjusted
                };
                (transform(x * 0.03), transform(y * 0.07))
            });

            let step_2 = Box::new(move |x: f64, y: f64| {
                let poly = |n: f64| (0.5 * n.powf(0.1)) + 4.0 * n;
                (poly(x), (poly(y).sin() + 3.0) * 0.25)
            });

            let mut transformed = (row as f64, col as f64);
            let pipeline: Vec<Box<dyn Fn(f64, f64) -> (f64, f64)>> = vec![step_1, step_2];
            pipeline.iter().for_each(|t| {
                let (x, y) = transformed;
                transformed = t(x, y);
            });
            let rads = transformed.0 * transformed.1;
            let (x, y) = rads_to_vector(rads);
            cells.push((x, y))
        }
    }
    cells
}

pub fn init_particles() -> Vec<(f64, f64)> {
    let mut particles = vec![];
    for _ in 0..PARTICLE_COUNT {
        particles.push(get_random_particle())
    }
    particles
}

pub fn get_random_particle() -> (f64, f64) {
    let mut rng = thread_rng();
    (
        rng.gen_range(0f64..=WINDOW_WIDTH),
        rng.gen_range(0f64..=WINDOW_HEIGHT),
    )
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

pub fn get_next_particle(x: &f64, y: &f64, cell: &(f64, f64)) -> (f64, f64) {
    // Do nothing
    // let next_x = (x + (cell.0 * CELL_INFLUENCE)).max(0.0).min(WINDOW_WIDTH);
    // let next_y = (y + (cell.1 * CELL_INFLUENCE)).max(0.0).min(WINDOW_HEIGHT);

    // Wrap around if they go offscreen
    // Swapping x for y can produce some interesting effects
    let next_x = (x + (cell.0 * CELL_INFLUENCE)) % WINDOW_WIDTH;
    let next_y = (y + (cell.1 * CELL_INFLUENCE)) % WINDOW_HEIGHT;

    // Randomly respawn if they go offscreen
    // let next_x = x + (cell.0 * CELL_INFLUENCE);
    // let next_y = y + (cell.1 * CELL_INFLUENCE);
    //
    // if next_x <= 0.0 || next_x >= WINDOW_WIDTH || next_y <= 0.0 || next_y >= WINDOW_HEIGHT {
    //     return get_random_particle();
    // }

    (next_x, next_y)
}
