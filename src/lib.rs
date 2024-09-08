/***********************************************************************************************************************
 * Porous Absorber Calculator - Public entry points from browser
 *
 * (c) Chris Whealy 2020, 2024
 */
mod chart;
mod config;
mod devices;
mod trace;
mod utils;

use std::fmt::Debug;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use {
    devices::DeviceTypeArgs,
    trace::{make_boundary_trace_fn, make_trace_fn, trace_flags::trace_flag_for, TraceAction},
};

pub const MOD_NAME: &str = "lib";

/***********************************************************************************************************************
 * Invoke calculation for generic absorption device
 */
fn handle_device<T>(wasm_arg_obj: JsValue, fn_name: &str, device_fn: Box<dyn Fn(T) -> JsValue>) -> JsValue
where
    T: DeviceTypeArgs +  for<'a> Deserialize<'a> + Debug,
{
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, fn_name)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<T>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, fn_name)(format!("{:?}", arg_obj));
            device_fn(arg_obj)
        },
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        },
    }
}

/***********************************************************************************************************************
 * Rigid backed porous absorber
 */
#[wasm_bindgen]
pub fn porous_absorber(wasm_arg_obj: JsValue) -> JsValue {
    handle_device(wasm_arg_obj, "porous_absorber", Box::new(devices::porous_absorber::prepare))
}

/***********************************************************************************************************************
 * Slotted panel
 */
#[wasm_bindgen]
pub fn slotted_panel(wasm_arg_obj: JsValue) -> JsValue {
    handle_device(wasm_arg_obj, "slotted_panel", Box::new(devices::slotted_panel::prepare))
}

/***********************************************************************************************************************
 * Perforated panel
 */
#[wasm_bindgen]
pub fn perforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    handle_device(wasm_arg_obj, "perforated_panel", Box::new(devices::perforated_panel::prepare))
}

/***********************************************************************************************************************
 * Microperforated panel
 */
#[wasm_bindgen]
pub fn microperforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    handle_device(wasm_arg_obj, "microperforated_panel", Box::new(devices::microperforated_panel::prepare))
}
