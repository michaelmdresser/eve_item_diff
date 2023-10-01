use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn diff(left: &str, right: &str) -> Result<JsValue, JsValue> {
    let diff = eve_item_diff::diff(left, right)?;
    return Ok(serde_wasm_bindgen::to_value(&diff)?);
}
