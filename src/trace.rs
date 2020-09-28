// *********************************************************************************************************************
// Porous Absorber Trace Utility
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************
use wasm_bindgen::prelude::*;

const ENTRY_ARROW: &str = &"WASM --->";
const EXIT_ARROW: &str = &"WASM <---";
const IN_OUT_ARROW: &str = &"WASM <-->";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Interface to browser functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: String);
}

// *********************************************************************************************************************
// The Trace struct is simply a container for the required functionality
// *********************************************************************************************************************
pub struct Trace {}

impl Trace {
  // *******************************************************************************************************************
  // Trace execution flow at function boundaries
  // *******************************************************************************************************************
  pub fn make_boundary_trace_fn<'a>(
    is_active: &'a bool,
    lib_name: &'a str,
    fn_name: &'a str,
  ) -> impl Fn(&'a Option<bool>) {
    move |is_entry| {
      if *is_active {
        let ptr = match is_entry {
          Some(b) => {
            if *b {
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

  // *******************************************************************************************************************
  // Trace data during execution flow
  // *******************************************************************************************************************
  pub fn make_trace_fn<'a>(
    is_active: &'a bool,
    lib_name: &'a str,
    fn_name: &'a str,
  ) -> impl Fn(&str) + 'a {
    move |info| {
      if *is_active {
        log(format!("WASM      {}.{}() {}", lib_name, fn_name, info));
      }
    }
  }
}
