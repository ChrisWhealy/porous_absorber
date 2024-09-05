/***********************************************************************************************************************
 * Porous Absorber Calculator - Public entry points from browser
 *
 * (c) Chris Whealy 2020
 */
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

mod calc_engine;
mod chart;
mod config;
mod devices;
mod trace;
mod utils;

use wasm_bindgen::prelude::*;

/***********************************************************************************************************************
 * Trace functionality
 */
use crate::{
    config::trace_flags::trace_flag_for,
    trace::*,
};

pub const MOD_NAME: &str = "lib";

/***********************************************************************************************************************
 * Rigid backed porous absorber
 */
#[derive(Deserialize)]
pub struct PorousAbsorberArgs {
    pub absorber_thickness_mm: String, // Parsed to u16
    pub flow_resistivity: String,      // Parsed to u32
    pub air_gap_mm: String,            // Parsed to u16
    pub angle: String,                 // Parsed to u16
    pub graph_start_freq: String,      // Parsed to f64
    pub smooth_curve: String,          // Parsed to bool
    pub subdivision: String,           // Parsed to u16
    pub show_diagram: String,          // Parsed to bool
    pub air_temp: String,              // Parsed to i16
    pub air_pressure: String,          // Parsed to f64
}

#[wasm_bindgen]
pub fn porous_absorber(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "porous_absorber";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    // Parse object received from JavaScript
    let arg_obj: PorousAbsorberArgs = wasm_arg_obj.into_serde().unwrap();

    // What values did we receive from JavaScript?
    trace(format!("absorber_thickness_mm = {}", arg_obj.absorber_thickness_mm));
    trace(format!("flow_resistivity      = {}", arg_obj.flow_resistivity));
    trace(format!("air_gap_mm            = {}", arg_obj.air_gap_mm));
    trace(format!("angle                 = {}", arg_obj.angle));
    trace(format!("graph_start_freq      = {}", arg_obj.graph_start_freq));
    trace(format!("smooth_curve          = {}", arg_obj.smooth_curve));
    trace(format!("subdivision           = {}", arg_obj.subdivision));
    trace(format!("show_diagram          = {}", arg_obj.show_diagram));
    trace(format!("air_temp              = {}", arg_obj.air_temp));
    trace(format!("air_pressure          = {}", arg_obj.air_pressure));

    devices::porous_absorber::calculate(arg_obj)
}

/***********************************************************************************************************************
 * Slotted panel
 */
#[derive(Deserialize)]
pub struct SlottedPanelArgs {
    pub panel_thickness_mm: String,    // Parsed to f64
    pub slot_distance_mm: String,      // Parsed to f64
    pub slot_width_mm: String,         // Parsed to f64
    pub slotted_porosity: String,      // Parsed to f64
    pub absorber_thickness_mm: String, // Parsed to u16
    pub flow_resistivity: String,      // Parsed to u32
    pub air_gap_mm: String,            // Parsed to u16
    pub graph_start_freq: String,      // Parsed to f64
    pub smooth_curve: String,          // Parsed to bool
    pub subdivision: String,           // Parsed to u16
    pub show_diagram: String,          // Parsed to bool
    pub air_temp: String,              // Parsed to i16
    pub air_pressure: String,          // Parsed to f64
}

#[wasm_bindgen]
pub fn slotted_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "slotted_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    // Parse object received from JavaScript
    let arg_obj: SlottedPanelArgs = wasm_arg_obj.into_serde().unwrap();

    // What values did we receive from JavaScript?
    trace(format!("panel_thickness_mm    = {}", arg_obj.panel_thickness_mm));
    trace(format!("slot_distance_mm      = {}", arg_obj.slot_distance_mm));
    trace(format!("slot_width_mm         = {}", arg_obj.slot_width_mm));
    trace(format!("slotted_porosity      = {}", arg_obj.slotted_porosity));
    trace(format!("absorber_thickness_mm = {}", arg_obj.absorber_thickness_mm));
    trace(format!("flow_resistivity      = {}", arg_obj.flow_resistivity));
    trace(format!("air_gap_mm            = {}", arg_obj.air_gap_mm));
    trace(format!("graph_start_freq      = {}", arg_obj.graph_start_freq));
    trace(format!("smooth_curve          = {}", arg_obj.smooth_curve));
    trace(format!("subdivision           = {}", arg_obj.subdivision));
    trace(format!("show_diagram          = {}", arg_obj.show_diagram));
    trace(format!("air_temp              = {}", arg_obj.air_temp));
    trace(format!("air_pressure          = {}", arg_obj.air_pressure));

    devices::slotted_panel::calculate(arg_obj)
}

/***********************************************************************************************************************
 * Perforated panel
 */
#[derive(Deserialize)]
pub struct PerforatedPanelArgs {
    pub panel_thickness_mm: String,    // Parsed to f64
    pub repeat_distance_mm: String,    // Parsed to f64
    pub hole_radius_mm: String,        // Parsed to f64
    pub porosity: String,              // Parsed to f64
    pub absorber_thickness_mm: String, // Parsed to u16
    pub flow_resistivity: String,      // Parsed to u32
    pub air_gap_mm: String,            // Parsed to u16
    pub graph_start_freq: String,      // Parsed to f64
    pub smooth_curve: String,          // Parsed to bool
    pub subdivision: String,           // Parsed to u16
    pub show_diagram: String,          // Parsed to bool
    pub air_temp: String,              // Parsed to i16
    pub air_pressure: String,          // Parsed to f64
}

#[wasm_bindgen]
pub fn perforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "perforated_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    // Parse object received from JavaScript
    let arg_obj: PerforatedPanelArgs = wasm_arg_obj.into_serde().unwrap();

    // What values did we receive from JavaScript?
    trace(format!("panel_thickness_mm    = {}", arg_obj.panel_thickness_mm));
    trace(format!("repeat_distance_mm    = {}", arg_obj.repeat_distance_mm));
    trace(format!("hole_radius_mm        = {}", arg_obj.hole_radius_mm));
    trace(format!("porosity              = {}", arg_obj.porosity));
    trace(format!("absorber_thickness_mm = {}", arg_obj.absorber_thickness_mm));
    trace(format!("flow_resistivity      = {}", arg_obj.flow_resistivity));
    trace(format!("air_gap_mm            = {}", arg_obj.air_gap_mm));
    trace(format!("graph_start_freq      = {}", arg_obj.graph_start_freq));
    trace(format!("smooth_curve          = {}", arg_obj.smooth_curve));
    trace(format!("subdivision           = {}", arg_obj.subdivision));
    trace(format!("show_diagram          = {}", arg_obj.show_diagram));
    trace(format!("air_temp              = {}", arg_obj.air_temp));
    trace(format!("air_pressure          = {}", arg_obj.air_pressure));

    devices::perforated_panel::calculate(arg_obj)
}

/***********************************************************************************************************************
 * Microperforated panel
 */
#[derive(Deserialize)]
pub struct MicroperforatedPanelArgs {
    pub panel_thickness_mm: String, // Parsed to f64
    pub repeat_distance_mm: String, // Parsed to f64
    pub hole_radius_mm: String,     // Parsed to f64
    pub porosity: String,           // Parsed to f64
    pub air_gap_mm: String,         // Parsed to u16
    pub angle: String,              // Parsed to u16
    pub graph_start_freq: String,   // Parsed to f64
    pub smooth_curve: String,       // Parsed to bool
    pub subdivision: String,        // Parsed to u16
    pub show_diagram: String,       // Parsed to bool
    pub air_temp: String,           // Parsed to i16
    pub air_pressure: String,       // Parsed to f64
}

#[wasm_bindgen]
pub fn microperforated_panel(wasm_arg_obj: JsValue) -> JsValue {
    const FN_NAME: &str = "microperforated_panel";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME)(TraceAction::EnterExit);

    // Parse object received from JavaScript
    let arg_obj: MicroperforatedPanelArgs = wasm_arg_obj.into_serde().unwrap();

    // What values did we receive from JavaScript?
    trace(format!("panel_thickness_mm    = {}", arg_obj.panel_thickness_mm));
    trace(format!("repeat_distance_mm    = {}", arg_obj.repeat_distance_mm));
    trace(format!("hole_radius_mm        = {}", arg_obj.hole_radius_mm));
    trace(format!("porosity              = {}", arg_obj.porosity));
    trace(format!("air_gap_mm            = {}", arg_obj.air_gap_mm));
    trace(format!("angle                 = {}", arg_obj.angle));
    trace(format!("graph_start_freq      = {}", arg_obj.graph_start_freq));
    trace(format!("smooth_curve          = {}", arg_obj.smooth_curve));
    trace(format!("subdivisions          = {}", arg_obj.subdivision));
    trace(format!("show_diagram          = {}", arg_obj.show_diagram));
    trace(format!("air_temp              = {}", arg_obj.air_temp));
    trace(format!("air_pressure          = {}", arg_obj.air_pressure));

    devices::microperforated_panel::calculate(arg_obj)
}
