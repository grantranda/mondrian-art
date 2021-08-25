use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/res/script.js")]
extern "C" {
    #[wasm_bindgen(js_name = "renderImage")]
    pub fn render_image(resolution: u32);
}
