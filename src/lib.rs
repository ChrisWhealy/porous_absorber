// *********************************************************************************************************************
// Porous Absorber Calculator
//
// This top-level library acts simply a list of public entry points
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Submodules
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
mod calc_engine;
mod render;
mod structs;
mod trace;

mod device_microperforated_panel;
mod device_perforated_panel;
mod device_rb_porous_absorber;
mod device_slotted_panel;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Usage
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use device_microperforated_panel::*;
use device_perforated_panel::*;
use device_rb_porous_absorber::*;
use device_slotted_panel::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use trace::Trace;

const LIB_NAME: &str = "lib";
const TRACE_ACTIVE: bool = false;

// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************

// *********************************************************************************************************************
// Default and main entry points
//
// The public function main() prefixed with the #[wasm_bindgen(start)] directive becomes the default enrty point and
// must be present since it will be called automatically when the WASM module is initialised; however, we do not require
// any specific functionality to run at this point in time, so this function simply returns "ok"
//
// The names of the public functions exposed by the #[wasm_bindgen] directive must exactly match the tab names listed in
// the tabConfig JavaScript object
// *********************************************************************************************************************
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  Ok(())
}

// *********************************************************************************************************************
// Rigid backed porous absorber
// *********************************************************************************************************************
#[wasm_bindgen]
pub fn rb_porous_absorber(wasm_arg_obj: JsValue) -> JsValue {
  (Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "rb_porous_absorber".to_string()))(None);
  do_porous_absorber_device(wasm_arg_obj)
}

// *********************************************************************************************************************
// Slotted panel
// *********************************************************************************************************************
#[wasm_bindgen]
pub fn slotted_panel(wasm_arg_obj: JsValue) -> JsValue {
  (Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "slotted_panel".to_string()))(None);
  do_slotted_panel_device(wasm_arg_obj)
}

// *********************************************************************************************************************
// Perforated panel
// *********************************************************************************************************************
#[wasm_bindgen]
pub fn perforated_panel(wasm_arg_obj: JsValue) -> JsValue {
  (Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "perforated_panel".to_string()))(None);
  do_perforated_panel_device(wasm_arg_obj)
}

// *********************************************************************************************************************
// Microperforated panel
// *********************************************************************************************************************
#[wasm_bindgen]
pub fn microperforated_panel(wasm_arg_obj: JsValue) -> JsValue {
  (Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "microperforated_panel".to_string()))(None);
  do_microperforated_panel_device(wasm_arg_obj)
}
