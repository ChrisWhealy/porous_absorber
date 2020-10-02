/***********************************************************************************************************************
 * Porous Absorber Calculator - Public entry points from browser
 *
 * (c) Chris Whealy 2020
 */
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

mod calc_engine;
mod chart;
mod config;
mod devices;
mod trace;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use devices::{
  microperforated_panel::do_microperforated_panel_device, perforated_panel::do_perforated_panel_device,
  porous_absorber::do_porous_absorber_device, slotted_panel::do_slotted_panel_device,
};

/***********************************************************************************************************************
 * Trace functionality
 */
use crate::trace::function_boundaries::{make_boundary_trace_fn, TraceAction};

const LIB_NAME: &str = "lib";
const TRACE_ACTIVE: bool = false;

/***********************************************************************************************************************
 * Default and main entry points
 *
 * The public function main() prefixed with the #[wasm_bindgen(start)] directive becomes the default enrty point and
 * must be present since it will be called automatically when the WASM module is initialised; however, we do not require
 * any specific functionality to run at this point in time, so this function simply returns "ok"
 *
 * The names of the public functions exposed by the #[wasm_bindgen] directive must exactly match the tab names listed in
 * the tabConfig JavaScript object
 */
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  (make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "main".to_string()))(TraceAction::EnterExit);
  Ok(())
}

/***********************************************************************************************************************
 * Rigid backed porous absorber
 */
#[wasm_bindgen]
pub fn porous_absorber(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "porous_absorber".to_string()))(TraceAction::EnterExit);
  do_porous_absorber_device(wasm_arg_obj)
}

/***********************************************************************************************************************
 * Slotted panel
 */
#[wasm_bindgen]
pub fn slotted_panel(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "slotted_panel".to_string()))(TraceAction::EnterExit);
  do_slotted_panel_device(wasm_arg_obj)
}

/***********************************************************************************************************************
 * Perforated panel
 */
#[wasm_bindgen]
pub fn perforated_panel(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "perforated_panel".to_string()))(TraceAction::EnterExit);
  do_perforated_panel_device(wasm_arg_obj)
}

/***********************************************************************************************************************
 * Microperforated panel
 */
#[wasm_bindgen]
pub fn microperforated_panel(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), "microperforated_panel".to_string()))(
    TraceAction::EnterExit,
  );
  do_microperforated_panel_device(wasm_arg_obj)
}
