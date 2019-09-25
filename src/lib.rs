// *********************************************************************************************************************
// Porous Absorber Calculator
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Submodules
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
mod trace;

mod porous_absorber;
mod perforated_panel;
mod slotted_panel;
mod air;
mod cavity;
mod sound;
mod display;
mod render;

mod struct_lib;
mod calc_engine;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Usage
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use std::error::Error;

use trace::Trace;

use air::{AirConfig, AirError};
use cavity::{CavityConfig, CavityError};
use display::{DisplayConfig, DisplayError};
use sound::{SoundConfig, SoundError};

use porous_absorber::{PorousAbsorberConfig, PorousError};
use perforated_panel::{PerforatedPanelConfig, PerforatedError};
use slotted_panel::{SlottedPanelConfig, SlottedError};

use calc_engine::{
  calculate_porous_absorber
, calculate_perforated_panel
, calculate_slotted_panel
};


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const LIB_NAME     : &str  = &"lib";
const TRACE_ACTIVE : &bool = &false;


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Interface to browser functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}



// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Main entry points
// The names of the public functions must be identical to the tab names listed in the tabConfig object
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
  return if error_msgs.len() == 0 {
    let absorber_info = calculate_porous_absorber(&air_cfg, &cavity_cfg, &display_cfg, &sound_cfg, &porous_cfg);
    
    // Plot the graph
    render::plot_porous_absorber(&absorber_info, &display_cfg, &sound_cfg);

    JsValue::from("Ok")
  }
  else {
    log(&format!("{} error{} detected in input values", error_msgs.len(), if error_msgs.len() == 1 { "" } else { "s" }));

    // Serialize the error message(s) and pass back to JavaScript
    JsValue::from_serde(&error_msgs).unwrap()
  }
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
  trace(&format!("air_pressure          = {}\n", air_pressure));

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
  return if error_msgs.len() == 0 {
    let absorber_info = calculate_perforated_panel(&air_cfg, &cavity_cfg, &display_cfg, &panel_cfg, &porous_cfg);
    
    // Plot the graph
    trace(&format!("Absorber against panel {:?}", absorber_info.abs_against_panel));
    trace(&format!("Absorber against backing {:?}", absorber_info.abs_against_backing));
    trace(&format!("No air gap {:?}", absorber_info.no_air_gap));

    render::plot_perforated_panel(&absorber_info, &display_cfg);

    JsValue::from("Ok")
  }
  else {
    log(&format!("{} error{} detected in input values", error_msgs.len(), if error_msgs.len() == 1 { "" } else { "s" }));

    // Serialize the error message(s) and pass back to JavaScript
    JsValue::from_serde(&error_msgs).unwrap()
  }
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
  trace(&format!("air_pressure          = {}\n", air_pressure));

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
  return if error_msgs.len() == 0 {
    let absorber_info = calculate_slotted_panel(&air_cfg, &cavity_cfg, &display_cfg, &panel_cfg, &porous_cfg);
    
    // Plot the graph
    trace(&format!("Absorber against panel {:?}", absorber_info.abs_against_panel));
    trace(&format!("Absorber against backing {:?}", absorber_info.abs_against_backing));
    trace(&format!("No air gap {:?}", absorber_info.no_air_gap));

    render::plot_slotted_panel(&absorber_info, &display_cfg);

    JsValue::from("Ok")
  }
  else {
    log(&format!("{} error{} detected in input values", error_msgs.len(), if error_msgs.len() == 1 { "" } else { "s" }));

    // Serialize the error message(s) and pass back to JavaScript
    JsValue::from_serde(&error_msgs).unwrap()
  }
}


