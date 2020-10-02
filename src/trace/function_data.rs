/***********************************************************************************************************************
 * Trace Utility -Trace data being used within a function
 *
 * FYI: The rust-analyzer might flag the calls to the `log` function as unsafe; however, the code compiles fine
 *
 * (c) Chris Whealy 2020
 */
use wasm_bindgen::prelude::*;

/***********************************************************************************************************************
 * Interface to browser console.log() function
 */
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: String);
}

/*********************************************************************************************************************
 * Trace data during execution flow
 */
pub fn make_trace_fn(is_active: bool, lib_name: String, fn_name: String) -> impl Fn(String) {
  move |info| {
    if is_active {
      log(format!("WASM      {}.{}() {}", lib_name, fn_name, info));
    }
  }
}
