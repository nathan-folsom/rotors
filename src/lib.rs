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

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub struct FieldRenderer {
    cells: Vec<(f64, f64)>,
    rotors: (Rotor, Rotor),
    particles: Vec<Particle>,
    frame_count: i32,
}

#[wasm_bindgen]
impl FieldRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cells: init_cells(),
            particles: init_particles(),
            rotors: init_rotors(),
            frame_count: 0,
        }
    }

    #[wasm_bindgen]
    pub fn init(&self, ctx: &CanvasRenderingContext2d) {
        self.render_background(ctx);
    }

    #[wasm_bindgen]
    pub fn render_frame(&mut self, ctx: &CanvasRenderingContext2d) -> i32 {
        self.frame_count += 1;
        self.render_background(ctx);
        self.render_rotors(ctx);
        self.frame_count
    }

    fn render_background(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str("#03000f"));
        ctx.fill_rect(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
        ctx.set_stroke_style(&JsValue::from("#ffffff"));

        let render_rim = |rotor: &Rotor| {
            ctx.begin_path();
            let _ = ctx.ellipse(rotor.cx, rotor.cy, rotor.r, rotor.r, 0.0, 0.0, 2.0 * PI);
            ctx.stroke();
        };

        render_rim(&self.rotors.0);
        render_rim(&self.rotors.1);
    }

    fn render_rotors(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        let points = (self.rotors.0.get_point(), self.rotors.1.get_point());
        let render_point = |point: (f64, f64)| {
            let (x, y) = point;
            ctx.begin_path();
            let _ = ctx.ellipse(x, y, 2.0, 2.0, 0.0, 0.0, 2.0 * PI);
            ctx.stroke();
        };
        self.rotors.0.advance();
        self.rotors.1.advance();
        render_point(points.0);
        render_point(points.1);
    }
}
