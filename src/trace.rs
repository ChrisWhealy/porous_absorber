// *********************************************************************************************************************
// Porous Absorber Trace Utility
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
use wasm_bindgen::prelude::*;

const ENTRY_ARROW  : &str = &"--->";
const EXIT_ARROW   : &str = &"<---";
const IN_OUT_ARROW : &str = &"<---";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Interface to browser functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}


/***********************************************************************************************************************
 * Trace execution flow at function boundaries and provide tool for general logging
 */
pub struct Trace {}

impl Trace {
  pub fn make_boundary_trace_fn<'a>(is_active: &'a bool, lib_name: &'a str, fn_name: &'a str) ->
    impl Fn(&'a Option<bool>) {
      move |is_entry| {
        if *is_active {
          let ptr = match is_entry {
            Some(b) => if *b { ENTRY_ARROW } else { EXIT_ARROW }
          , None    => IN_OUT_ARROW
          };

          log(&format!("{} {}.{}()", ptr, lib_name, fn_name));
        }
        else {
          ()
        }
      }
    }

  pub fn make_trace_fn<'a>(is_active: &'a bool, lib_name: &'a str, fn_name: &'a str) ->
    impl Fn(&str) + 'a {
      move |info| {
        if *is_active {
          log(&format!("     {}.{}() {}", lib_name, fn_name, info));
        }
        else {
          ()
        }
      }
    }
}

