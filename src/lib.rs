use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

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

    let canvas = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("art")
        .expect("canvas should exist")
        .dyn_into::<HtmlCanvasElement>()?;

    canvas.set_width(500);
    canvas.set_height(500);

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    ctx.fill_rect(10f64, 10f64, 100f64, 100f64);

    Ok(())
}
