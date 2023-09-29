use crate::constants::*;
use crate::math::*;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub mod constants;
pub mod math;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
#[derive(Default)]
pub struct FieldRenderer {
    cells: Vec<(f64, f64)>,
    particles: Vec<(f64, f64)>,
    frame_count: i32,
}

#[wasm_bindgen]
impl FieldRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cells: init_cells(),
            particles: init_particles(),
            frame_count: 0,
        }
    }

    #[wasm_bindgen]
    pub fn render_frame(&mut self, ctx: &CanvasRenderingContext2d) -> i32 {
        self.frame_count += 1;

        // self.render_cells(ctx);
        self.render_particles(ctx);

        self.frame_count
    }

    fn render_cells(&mut self, ctx: &CanvasRenderingContext2d) {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let cell = &self.cells[row * GRID_SIZE + col];
                ctx.begin_path();
                let ((start_x, start_y), (end_x, end_y)) = get_cell_line(cell, row, col);
                let _ = ctx.ellipse(end_x, end_y, 2f64, 2f64, 0f64, 0f64, 2f64 * PI);
                ctx.move_to(start_x, start_y);
                ctx.line_to(end_x, end_y);
                ctx.stroke();
            }
        }
    }

    fn render_particles(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        self.particles = self
            .particles
            .iter()
            .map(|(x, y)| {
                ctx.begin_path();
                let _ = ctx.ellipse(
                    *x,
                    *y,
                    PARTICLE_SIZE / 2.0,
                    PARTICLE_SIZE / 2.0,
                    0f64,
                    0f64,
                    2f64 * PI,
                );
                ctx.stroke();

                let influence_cell_i = get_influence_cell(x, y);
                let cell = &self.cells[influence_cell_i];
                let next_x = (x + (cell.0 * CELL_INFLUENCE))
                    .min(WINDOW_WIDTH as f64)
                    .max(0f64);
                let next_y = (y + (cell.1 * CELL_INFLUENCE))
                    .min(WINDOW_HEIGHT as f64)
                    .max(0f64);
                (next_x, next_y)
            })
            .collect();

        ctx.set_stroke_style(&JsValue::from_str("#000000"));
    }
}
