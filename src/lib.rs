// *********************************************************************************************************************
// Porous Absorber Calculator
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Submodules
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
mod structs;
mod trace;
mod render;
mod calc_engine;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Usage
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use std::error::Error;

use trace::Trace;

use structs::air::{AirConfig, AirError};
use structs::cavity::{CavityConfig, CavityError};
use structs::display::{DisplayConfig, DisplayError};
use structs::sound::{SoundConfig, SoundError};

use structs::porous_absorber::{PorousAbsorberConfig, PorousError};
use structs::slotted_panel::{SlottedPanelConfig, SlottedError};
use structs::perforated_panel::{PerforatedPanelConfig, PerforatedError};
use structs::microperforated_panel::{MicroperforatedPanelConfig, MicroperforatedError};

use calc_engine::{
  calculate_porous_absorber
, calculate_perforated_panel
, calculate_microperforated_panel
, calculate_slotted_panel
};


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const LIB_NAME     : &str  = &"lib";
const TRACE_ACTIVE : &bool = &false;


// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Default entry point
// No specific functionality needs to attached to the default entry point.
// This entry point will be called automatically when the WASM module is first invoked and must be present
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Main entry points
// The names of the public functions listed below must match the tab names listed in the tabConfig JavaScript object
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Rigid backed porous absorber
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
pub fn rb_porous_absorber(
  absorber_thickness_mm : u32
, flow_resistivity      : u32
, air_gap_mm            : u32
, angle                 : u32
, graph_start_freq      : f64
, smooth_curve          : bool
, subdivisions          : u32
, air_temp              : f64
, air_pressure          : f64
) -> JsValue{
  const FN_NAME : &str = &"rb_porous_absorber";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // What values did we receive from JavaScript?
  trace(&format!("absorber_thickness_mm = {}", absorber_thickness_mm));
  trace(&format!("flow_resistivity      = {}", flow_resistivity));
  trace(&format!("air_gap_mm            = {}", air_gap_mm));
  trace(&format!("graph_start_freq      = {}", graph_start_freq));
  trace(&format!("smooth_curve          = {}", smooth_curve));
  trace(&format!("subdivisions          = {}", subdivisions));
  trace(&format!("air_temp              = {}", air_temp));
  trace(&format!("air_pressure          = {}", air_pressure));

  // Empty return data structure
  let mut error_msgs: Vec<String> = vec!();

  // Construct configuration structs
  let air_cfg = AirConfig::new(air_temp, air_pressure).unwrap_or_else(|err: AirError| {
    error_msgs.push(String::from(err.description()));
    AirConfig::default()
  });

  let cavity_cfg = CavityConfig::new(air_gap_mm).unwrap_or_else(|err: CavityError| {
    error_msgs.push(String::from(err.description()));
    CavityConfig::default()
  });

  let display_cfg = DisplayConfig::new(graph_start_freq, smooth_curve, subdivisions).unwrap_or_else(|err: DisplayError| {
    error_msgs.push(String::from(err.description()));
    DisplayConfig::default()
  });

  let sound_cfg = SoundConfig::new(angle).unwrap_or_else(|err: SoundError| {
    error_msgs.push(String::from(err.description()));
    SoundConfig::default()
  });

  let porous_cfg = PorousAbsorberConfig::new(absorber_thickness_mm, flow_resistivity).unwrap_or_else(|err: PorousError| {
    error_msgs.push(String::from(err.description()));
    PorousAbsorberConfig::default()
  });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.len() == 0 {
    let absorber_info = calculate_porous_absorber(&air_cfg, &cavity_cfg, &display_cfg, &sound_cfg, &porous_cfg);
    
    // Plot the graph
    let chart_info = render::plot_porous_absorber(&absorber_info, &display_cfg, &sound_cfg);

    JsValue::from_serde(&chart_info).unwrap()
  }
  else {
    // Serialize the error message(s)
    JsValue::from_serde(&error_msgs).unwrap()
  };

  trace_boundary(&Some(true));

  // Return either the {X,Y} values of plot points or the error messages back to JavaScript
  return_value
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Perforated panel
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
pub fn perforated_panel(
  panel_thickness_mm    : f64
, repeat_distance_mm    : f64
, hole_radius_mm        : f64
, porosity              : f64
, absorber_thickness_mm : u32
, flow_resistivity      : u32
, air_gap_mm            : u32
, graph_start_freq      : f64
, smooth_curve          : bool
, subdivisions          : u32
, air_temp              : f64
, air_pressure          : f64
) -> JsValue{
  const FN_NAME : &str = &"perforated_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // What values did we receive from JavaScript?
  trace(&format!("panel_thickness_mm    = {}", panel_thickness_mm));
  trace(&format!("repeat_distance_mm    = {}", repeat_distance_mm));
  trace(&format!("hole_radius_mm        = {}", hole_radius_mm));
  trace(&format!("porosity              = {}", porosity));
  trace(&format!("absorber_thickness_mm = {}", absorber_thickness_mm));
  trace(&format!("flow_resistivity      = {}", flow_resistivity));
  trace(&format!("air_gap_mm            = {}", air_gap_mm));
  trace(&format!("graph_start_freq      = {}", graph_start_freq));
  trace(&format!("smooth_curve          = {}", smooth_curve));
  trace(&format!("subdivisions          = {}", subdivisions));
  trace(&format!("air_temp              = {}", air_temp));
  trace(&format!("air_pressure          = {}", air_pressure));

  // Empty return data structure
  let mut error_msgs: Vec<String> = vec!();

  // Construct configuration structs
  let air_cfg = AirConfig::new(air_temp, air_pressure).unwrap_or_else(|err: AirError| {
    error_msgs.push(String::from(err.description()));
    AirConfig::default()
  });

  let cavity_cfg = CavityConfig::new(air_gap_mm).unwrap_or_else(|err: CavityError| {
    error_msgs.push(String::from(err.description()));
    CavityConfig::default()
  });

  let display_cfg = DisplayConfig::new(graph_start_freq, smooth_curve, subdivisions).unwrap_or_else(|err: DisplayError| {
    error_msgs.push(String::from(err.description()));
    DisplayConfig::default()
  });

  let panel_cfg = PerforatedPanelConfig::new(panel_thickness_mm, repeat_distance_mm, hole_radius_mm, porosity).unwrap_or_else(|err: PerforatedError| {
    error_msgs.push(String::from(err.description()));
    PerforatedPanelConfig::default()
  });

  let porous_cfg = PorousAbsorberConfig::new(absorber_thickness_mm, flow_resistivity).unwrap_or_else(|err: PorousError| {
    error_msgs.push(String::from(err.description()));
    PorousAbsorberConfig::default()
  });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.len() == 0 {
    let absorber_info = calculate_perforated_panel(&air_cfg, &cavity_cfg, &display_cfg, &panel_cfg, &porous_cfg);
    
    // Plot the graph
    let chart_info = render::plot_perforated_panel(&absorber_info, &display_cfg);

    JsValue::from_serde(&chart_info).unwrap()
  }
  else {
    // Serialize the error message(s)
    JsValue::from_serde(&error_msgs).unwrap()
  };

  trace_boundary(&Some(false));

  // Return either the {X,Y} values of plot points or the error messages back to JavaScript
  return_value
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Slotted panel
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
pub fn slotted_panel(
  panel_thickness_mm    : f64
, slot_distance_mm      : f64
, slot_width_mm         : f64
, porosity              : f64
, absorber_thickness_mm : u32
, flow_resistivity      : u32
, air_gap_mm            : u32
, graph_start_freq      : f64
, smooth_curve          : bool
, subdivisions          : u32
, air_temp              : f64
, air_pressure          : f64
) -> JsValue{
  const FN_NAME : &str = &"slotted_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // What values did we receive from JavaScript?
  trace(&format!("panel_thickness_mm    = {}", panel_thickness_mm));
  trace(&format!("slot_distance_mm      = {}", slot_distance_mm));
  trace(&format!("slot_width_mm         = {}", slot_width_mm));
  trace(&format!("porosity              = {}", porosity));
  trace(&format!("absorber_thickness_mm = {}", absorber_thickness_mm));
  trace(&format!("flow_resistivity      = {}", flow_resistivity));
  trace(&format!("air_gap_mm            = {}", air_gap_mm));
  trace(&format!("graph_start_freq      = {}", graph_start_freq));
  trace(&format!("smooth_curve          = {}", smooth_curve));
  trace(&format!("subdivisions          = {}", subdivisions));
  trace(&format!("air_temp              = {}", air_temp));
  trace(&format!("air_pressure          = {}", air_pressure));

  // Empty return data structure
  let mut error_msgs: Vec<String> = vec!();

  // Construct configuration structs
  let air_cfg = AirConfig::new(air_temp, air_pressure).unwrap_or_else(|err: AirError| {
    error_msgs.push(String::from(err.description()));
    AirConfig::default()
  });

  let cavity_cfg = CavityConfig::new(air_gap_mm).unwrap_or_else(|err: CavityError| {
    error_msgs.push(String::from(err.description()));
    CavityConfig::default()
  });

  let display_cfg = DisplayConfig::new(graph_start_freq, smooth_curve, subdivisions).unwrap_or_else(|err: DisplayError| {
    error_msgs.push(String::from(err.description()));
    DisplayConfig::default()
  });

  let panel_cfg = SlottedPanelConfig::new(panel_thickness_mm, slot_distance_mm, slot_width_mm, porosity).unwrap_or_else(|err: SlottedError| {
    error_msgs.push(String::from(err.description()));
    SlottedPanelConfig::default()
  });

  let porous_cfg = PorousAbsorberConfig::new(absorber_thickness_mm, flow_resistivity).unwrap_or_else(|err: PorousError| {
    error_msgs.push(String::from(err.description()));
    PorousAbsorberConfig::default()
  });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.len() == 0 {
    let absorber_info = calculate_slotted_panel(&air_cfg, &cavity_cfg, &display_cfg, &panel_cfg, &porous_cfg);
    
    // Plot the graph
    let chart_info = render::plot_slotted_panel(&absorber_info, &display_cfg);

    JsValue::from_serde(&chart_info).unwrap()
  }
  else {
    // Serialize the error message(s)
    JsValue::from_serde(&error_msgs).unwrap()
  };

  trace_boundary(&Some(false));

  // Return either the {X,Y} values of plot points or the error messages back to JavaScript
  return_value
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Microperforated panel
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
pub fn microperforated_panel(
  panel_thickness_mm    : f64
, repeat_distance_mm    : f64
, hole_radius_mm        : f64
, porosity              : f64
, air_gap_mm            : u32
, angle                 : u32
, graph_start_freq      : f64
, smooth_curve          : bool
, subdivisions          : u32
, air_temp              : f64
, air_pressure          : f64
) -> JsValue{
  const FN_NAME : &str = &"microperforated_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // What values did we receive from JavaScript?
  trace(&format!("panel_thickness_mm    = {}", panel_thickness_mm));
  trace(&format!("repeat_distance_mm    = {}", repeat_distance_mm));
  trace(&format!("hole_radius_mm        = {}", hole_radius_mm));
  trace(&format!("porosity              = {}", porosity));
  trace(&format!("air_gap_mm            = {}", air_gap_mm));
  trace(&format!("angle                 = {}", angle));
  trace(&format!("graph_start_freq      = {}", graph_start_freq));
  trace(&format!("smooth_curve          = {}", smooth_curve));
  trace(&format!("subdivisions          = {}", subdivisions));
  trace(&format!("air_temp              = {}", air_temp));
  trace(&format!("air_pressure          = {}", air_pressure));

  // Empty return data structure
  let mut error_msgs: Vec<String> = vec!();

  // Construct configuration structs
  let air_cfg = AirConfig::new(air_temp, air_pressure).unwrap_or_else(|err: AirError| {
    error_msgs.push(String::from(err.description()));
    AirConfig::default()
  });

  let cavity_cfg = CavityConfig::new(air_gap_mm).unwrap_or_else(|err: CavityError| {
    error_msgs.push(String::from(err.description()));
    CavityConfig::default()
  });

  let display_cfg = DisplayConfig::new(graph_start_freq, smooth_curve, subdivisions).unwrap_or_else(|err: DisplayError| {
    error_msgs.push(String::from(err.description()));
    DisplayConfig::default()
  });

  let panel_cfg = MicroperforatedPanelConfig::new(panel_thickness_mm, repeat_distance_mm, hole_radius_mm, porosity).unwrap_or_else(|err: MicroperforatedError| {
    error_msgs.push(String::from(err.description()));
    MicroperforatedPanelConfig::default()
  });

  let sound_cfg = SoundConfig::new(angle).unwrap_or_else(|err: SoundError| {
    error_msgs.push(String::from(err.description()));
    SoundConfig::default()
  });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.len() == 0 {
    let absorber_info = calculate_microperforated_panel(&air_cfg, &cavity_cfg, &display_cfg, &panel_cfg, &sound_cfg);
    
    // Plot the graph
    let chart_info = render::plot_microperforated_panel(&absorber_info, &display_cfg, &sound_cfg);

    JsValue::from_serde(&chart_info).unwrap()
  }
  else {
    // Serialize the error message(s)
    JsValue::from_serde(&error_msgs).unwrap()
  };

  trace_boundary(&Some(false));

  // Return either the {X,Y} values of plot points or the error messages back to JavaScript
  return_value
}

