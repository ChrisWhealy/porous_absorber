/***********************************************************************************************************************
 * Porous Absorber Calculator - Public entry points from browser
 *
 * (c) Chris Whealy 2020
 */
mod chart;
mod config;
mod devices;
mod trace;
mod utils;

use wasm_bindgen::prelude::*;

use {
    devices::microperforated_panel::MicroperforatedPanelArgs,
    devices::perforated_panel::PerforatedPanelArgs,
    devices::porous_absorber::PorousAbsorberArgs,
    devices::slotted_panel::SlottedPanelArgs,
    trace::{make_boundary_trace_fn, make_trace_fn, trace_flags::trace_flag_for, TraceAction},
};

pub const MOD_NAME: &str = "lib";

/***********************************************************************************************************************
 * Rigid backed porous absorber
 */
#[wasm_bindgen]
pub fn porous_absorber(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "porous_absorber";
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<PorousAbsorberArgs>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, FN_NAME)(format!("{:?}", arg_obj));
            devices::porous_absorber::prepare(arg_obj)
        },
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        },
    }
}

/***********************************************************************************************************************
 * Slotted panel
 */
#[wasm_bindgen]
pub fn slotted_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "slotted_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<SlottedPanelArgs>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, FN_NAME)(format!("{:?}", arg_obj));
            devices::slotted_panel::prepare(arg_obj)
        },
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        },
    }
}

/***********************************************************************************************************************
 * Perforated panel
 */
#[wasm_bindgen]
pub fn perforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "perforated_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<PerforatedPanelArgs>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, FN_NAME)(format!("{:?}", arg_obj));
            devices::perforated_panel::prepare(arg_obj)
        },
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        },
    }
}

/***********************************************************************************************************************
 * Microperforated panel
 */
#[wasm_bindgen]
pub fn microperforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "microperforated_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<MicroperforatedPanelArgs>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, FN_NAME)(format!("{:?}", arg_obj));
            devices::microperforated_panel::prepare(arg_obj)
        },
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        },
    }
}
