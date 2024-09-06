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

use serde::Deserialize;
use wasm_bindgen::prelude::*;

/***********************************************************************************************************************
 * Trace functionality
 */
use crate::{
    config::trace_flags::trace_flag_for,
    trace::{make_boundary_trace_fn, make_trace_fn, TraceAction},
};

pub const MOD_NAME: &str = "lib";

/***********************************************************************************************************************
 * Rigid backed porous absorber
 */
#[derive(Debug, Deserialize)]
pub struct PorousAbsorberArgs {
    pub absorber_thickness_mm: u16,
    pub flow_resistivity: u32,
    pub air_gap_mm: u16,
    pub angle: u16,
    pub graph_start_freq: f64,
    pub smooth_curve: bool,
    pub subdivision: u16,
    pub show_diagram: bool,
    pub air_temp: i16,
    pub air_pressure: f64,
}

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
#[derive(Debug, Deserialize)]
pub struct SlottedPanelArgs {
    pub panel_thickness_mm: f64,
    pub slot_distance_mm: f64,
    pub slot_width_mm: f64,
    pub slotted_porosity: f64,
    pub absorber_thickness_mm: u16,
    pub flow_resistivity: u32,
    pub air_gap_mm: u16,
    pub graph_start_freq: f64,
    pub smooth_curve: bool,
    pub subdivision: u16,
    pub show_diagram: bool,
    pub air_temp: i16,
    pub air_pressure: f64,
}

#[wasm_bindgen]
pub fn slotted_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "slotted_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<SlottedPanelArgs>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, FN_NAME)(format!("{:?}", arg_obj));
            devices::slotted_panel::prepare(arg_obj)
        }
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        }
    }
}

/***********************************************************************************************************************
 * Perforated panel
 */
#[derive(Debug, Deserialize)]
pub struct PerforatedPanelArgs {
    pub panel_thickness_mm: f64,
    pub repeat_distance_mm: f64,
    pub hole_radius_mm: f64,
    pub porosity: f64,
    pub absorber_thickness_mm: u16,
    pub flow_resistivity: u32,
    pub air_gap_mm: u16,
    pub graph_start_freq: f64,
    pub smooth_curve: bool,
    pub subdivision: u16,
    pub show_diagram: bool,
    pub air_temp: i16,
    pub air_pressure: f64,
}

#[wasm_bindgen]
pub fn perforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "perforated_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<PerforatedPanelArgs>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, FN_NAME)(format!("{:?}", arg_obj));
            devices::perforated_panel::prepare(arg_obj)
        }
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        }
    }
}

/***********************************************************************************************************************
 * Microperforated panel
 */
#[derive(Debug, Deserialize)]
pub struct MicroperforatedPanelArgs {
    pub panel_thickness_mm: f64,
    pub repeat_distance_mm: f64,
    pub hole_radius_mm: f64,
    pub porosity: f64,
    pub air_gap_mm: u16,
    pub angle: u16,
    pub graph_start_freq: f64,
    pub smooth_curve: bool,
    pub subdivision: u16,
    pub show_diagram: bool,
    pub air_temp: i16,
    pub air_pressure: f64,
}

#[wasm_bindgen]
pub fn microperforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "microperforated_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    match serde_wasm_bindgen::from_value::<MicroperforatedPanelArgs>(wasm_arg_obj) {
        Ok(arg_obj) => {
            make_trace_fn(trace_active, MOD_NAME, FN_NAME)(format!("{:?}", arg_obj));
            devices::microperforated_panel::prepare(arg_obj)
        }
        Err(err) => {
            trace::error(err.to_string());
            JsValue::undefined()
        }
    }
}
