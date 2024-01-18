use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/prediction_markets/js/mod.js")]
extern "C" {
    pub fn copy_text_to_clipboard(text: &str);
}