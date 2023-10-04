// Window Config
pub const GRID_SIZE: usize = 200;
pub const CELL_COUNT: usize = GRID_SIZE.pow(2);
pub const CELL_WIDTH: f64 = WINDOW_WIDTH / GRID_SIZE as f64;
pub const WINDOW_WIDTH: f64 = 1000.0;
pub const WINDOW_HEIGHT: f64 = 1000.0;

// Particle Config
pub const PARTICLE_SIZE: f64 = 0.5;
pub const PARTICLE_COUNT: i32 = 15_000;
pub const CELL_INFLUENCE: f64 = 0.1;
