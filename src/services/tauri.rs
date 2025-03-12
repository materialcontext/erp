use js_sys::{Promise, Reflect};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

/// Generic function to invoke Tauri commands and handle the response
pub async fn invoke<A, R>(cmd: &str, args: &A) -> Result<R, String>
where
    A: Serialize + ?Sized,
    R: DeserializeOwned,
{
    // Get the window object
    let window = window().ok_or_else(|| "Failed to get window object".to_string())?;

    // Access the __TAURI__ object
    let tauri = Reflect::get(&window, &JsValue::from_str("__TAURI__"))
        .map_err(|_| "Failed to access __TAURI__ object".to_string())?;

    // Access the invoke function
    let invoke_fn = Reflect::get(&tauri, &JsValue::from_str("invoke"))
        .map_err(|_| "Failed to access invoke function".to_string())?;

    // Convert args to JsValue
    let js_args = match serde_wasm_bindgen::to_value(args) {
        Ok(val) => val,
        Err(e) => return Err(format!("Failed to serialize arguments: {}", e)),
    };

    // Call the invoke function
    let promise = Reflect::apply(
        &invoke_fn.dyn_into::<js_sys::Function>().unwrap(),
        &tauri,
        &js_sys::Array::of3(&JsValue::from_str(cmd), &js_args, &JsValue::undefined()),
    )
    .map_err(|e| format!("Failed to invoke Tauri command: {:?}", e))?
    .dyn_into::<Promise>()
    .map_err(|_| "Expected Promise from Tauri invoke".to_string())?;

    // Wait for the promise to resolve
    let result = JsFuture::from(promise)
        .await
        .map_err(|e| format!("Tauri command failed: {:?}", e))?;

    // Deserialize the result
    let ret: R = serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to deserialize response: {}", e))?;

    Ok(ret)
}
