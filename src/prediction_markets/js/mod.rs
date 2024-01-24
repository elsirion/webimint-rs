use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/prediction_markets/js/mod.js")]
extern "C" {
    pub fn copy_text_to_clipboard(text: &str);
    pub fn create_chart() -> JsValue;
    pub fn set_chart_data(chart_ctx: JsValue, data: JsValue);
    pub fn update_chart_data(chart_ctx: JsValue, data: JsValue);
}