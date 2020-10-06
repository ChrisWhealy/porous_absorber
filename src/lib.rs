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
use crate::{
  config::trace_flags::trace_flag_for,
  trace::function_boundaries::{make_boundary_trace_fn, TraceAction},
};

pub const MOD_NAME: &str = "lib";

/***********************************************************************************************************************
 * Rigid backed porous absorber
 */
#[wasm_bindgen]
pub fn porous_absorber(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(
    trace_flag_for(MOD_NAME),
    MOD_NAME.to_string(),
    "porous_absorber".to_string(),
  ))(TraceAction::EnterExit);
  do_porous_absorber_device(wasm_arg_obj)
}

/***********************************************************************************************************************
 * Slotted panel
 */
#[wasm_bindgen]
pub fn slotted_panel(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(
    trace_flag_for(MOD_NAME),
    MOD_NAME.to_string(),
    "slotted_panel".to_string(),
  ))(TraceAction::EnterExit);
  do_slotted_panel_device(wasm_arg_obj)
}

/***********************************************************************************************************************
 * Perforated panel
 */
#[wasm_bindgen]
pub fn perforated_panel(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(
    trace_flag_for(MOD_NAME),
    MOD_NAME.to_string(),
    "perforated_panel".to_string(),
  ))(TraceAction::EnterExit);
  do_perforated_panel_device(wasm_arg_obj)
}

/***********************************************************************************************************************
 * Microperforated panel
 */
#[wasm_bindgen]
pub fn microperforated_panel(wasm_arg_obj: JsValue) -> JsValue {
  (make_boundary_trace_fn(
    trace_flag_for(MOD_NAME),
    MOD_NAME.to_string(),
    "microperforated_panel".to_string(),
  ))(TraceAction::EnterExit);
  do_microperforated_panel_device(wasm_arg_obj)
}
