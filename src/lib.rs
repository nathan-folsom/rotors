use crate::constants::*;
use crate::math::*;
use std::collections::VecDeque;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod constants;
mod math;

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
    rotors: (Rotor, Rotor),
    frame_count: i32,
    prev_point: Option<(f64, f64)>,
    points: VecDeque<(f64, f64)>,
}

#[wasm_bindgen]
impl FieldRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            rotors: init_rotors(),
            frame_count: 0,
            prev_point: None,
            points: VecDeque::new(),
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self, ctx: &CanvasRenderingContext2d) {
        self.render_background(ctx);
        self.compute_points();
    }

    #[wasm_bindgen]
    pub fn render_frame(&mut self, ctx: &CanvasRenderingContext2d) -> i32 {
        self.frame_count += 1;
        self.render_line(ctx);
        self.frame_count
    }

    #[wasm_bindgen]
    pub fn render_overlay(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
        ctx.set_stroke_style(&JsValue::from_str("#ff000055"));
        ctx.set_fill_style(&JsValue::from_str("#000000"));

        let render_rim = |rotor: &Rotor| {
            ctx.begin_path();
            let _ = ctx.ellipse(rotor.cx, rotor.cy, rotor.r, rotor.r, 0.0, 0.0, 2.0 * PI);
            ctx.stroke();
        };
        render_rim(&self.rotors.0);
        render_rim(&self.rotors.1);

        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        let points = (self.rotors.0.get_point(), self.rotors.1.get_point());
        let render_point = |point: (f64, f64)| {
            let (x, y) = point;
            ctx.begin_path();
            let _ = ctx.ellipse(x, y, 2.0, 2.0, 0.0, 0.0, 2.0 * PI);
            ctx.stroke();
        };
        render_point(points.0);
        render_point(points.1);

        ctx.set_stroke_style(&JsValue::from_str("#ff000055"));
        let intersections = get_intersection(&self.rotors.0, &self.rotors.1);
        ctx.begin_path();
        ctx.move_to(points.0 .0, points.0 .1);
        ctx.line_to(intersections.1 .0, intersections.1 .1);
        ctx.line_to(points.1 .0, points.1 .1);
        ctx.stroke();

        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        ctx.begin_path();
        let _ = ctx.ellipse(
            intersections.1 .0,
            intersections.1 .1,
            2.0,
            2.0,
            0.0,
            0.0,
            2.0 * PI,
        );
        ctx.stroke();
    }

    fn render_background(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str("#03000f"));
        ctx.fill_rect(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
        ctx.set_stroke_style(&JsValue::from("#ffffff"));
    }

    fn compute_points(&mut self) {
        for _ in 0..ITERATIONS_PER_FRAME {
            let intersections = get_intersection(&self.rotors.0, &self.rotors.1);
            self.points
                .push_back((intersections.1 .0, intersections.1 .1));
            self.rotors.0.advance();
            self.rotors.1.advance();
        }
    }

    fn render_line(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("#ffffff"));
        let (x, y) = self.points.pop_front().unwrap();
        ctx.begin_path();
        ctx.move_to(x, y);
        let last = self.points.pop_back().unwrap();
        while let Some((x, y)) = self.points.pop_front() {
            ctx.line_to(x, y);
        }
        let (x, y) = last;
        ctx.line_to(x, y);
        ctx.stroke();
        self.points.push_back(last);
        self.compute_points();
    }
}
