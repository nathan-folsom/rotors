extern crate vectart;

use vectart::constants::WINDOW_WIDTH;
use vectart::math::get_influence_cell;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_get_cell_index() {
    let i = get_influence_cell(&0.0, &0.0);
    assert_eq!(i, 0);
}

#[wasm_bindgen_test]
fn should_get_max_cell_index() {
    let i = get_influence_cell(&0.0, &(WINDOW_WIDTH as f64));
    assert_eq!(i, 0);
}
