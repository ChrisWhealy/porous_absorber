/***********************************************************************************************************************
 * Trace Utility
 *
 * FYI: The rust-analyzer might flag the calls to the `log` function as unsafe; however, the code compiles fine
 *
 * (c) Chris Whealy 2020
 */
use wasm_bindgen::prelude::*;

const ENTRY_ARROW: &str = "WASM --->";
const EXIT_ARROW: &str = "WASM <---";
const IN_OUT_ARROW: &str = "WASM <-->";

/***********************************************************************************************************************
 * Interface to browser console.log() function
 */
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: String);
}

pub struct Trace {}

impl Trace {
  /*********************************************************************************************************************
   * Trace execution flow at function boundaries
   */
  pub fn make_boundary_trace_fn(is_active: bool, lib_name: String, fn_name: String) -> impl Fn(Option<bool>) {
    move |is_entry: Option<bool>| {
      if is_active {
        let ptr = match is_entry {
          Some(b) => {
            if b {
              ENTRY_ARROW
            } else {
              EXIT_ARROW
            }
          }
          None => IN_OUT_ARROW,
        };

        log(format!("{} {}.{}()", ptr, lib_name, fn_name));
      }
    }
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
}
