use game::{ClientMsg, RegicideAction, ServerMsg};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn deserialize_payload(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let payload: ServerMsg = postcard::from_bytes(bytes)
        .map_err(|e| JsValue::from_str(&format!("bincode decode failed: {e}")))?;
    serde_wasm_bindgen::to_value(&payload)
        .map_err(|e| JsValue::from_str(&format!("serde-wasm-bindgen encode failed: {e}")))
}

#[wasm_bindgen]
pub fn serialize_payload(value: JsValue) -> Result<Box<[u8]>, JsValue> {
    let payload: ClientMsg<RegicideAction> = serde_wasm_bindgen::from_value(value)
        .map_err(|e| JsValue::from_str(&format!("serde-wasm-bindgen decode failed: {e}")))?;
    let bytes = postcard::to_stdvec(&payload)
        .map_err(|e| JsValue::from_str(&format!("bincode encode failed: {e}")))?;
    Ok(bytes.into_boxed_slice())
}
