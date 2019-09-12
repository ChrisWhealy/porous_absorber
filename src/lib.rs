// *********************************************************************************************************************
// Porous Absorber Calculator
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Submodules
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
mod air;
mod porous_absorber;
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

use struct_lib::PorousAbsInfo;
use air::{AirConfig, AirError};
use porous_absorber::{PorousAbsorberConfig, PorousError};
use cavity::{CavityConfig, CavityError};
use display::{DisplayConfig, DisplayError};
use sound::{SoundConfig, SoundError};

use calc_engine::calculate_porous_absorption;



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
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
pub fn porous_absorber_calculator(
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
    let absorber_info = calculate_porous_absorption(&air_cfg, &cavity_cfg, &display_cfg, &sound_cfg, &porous_cfg);
    
    // Plot the graph
    render::plot(&absorber_info, &display_cfg, &sound_cfg);

    JsValue::from("Ok")
  }
  else {
    log(&format!("{} error{} detected in input values", error_msgs.len(), if error_msgs.len() == 1 { "" } else { "s" }));

    // Serialize the error message(s) and pass back to JavaScript
    JsValue::from_serde(&error_msgs).unwrap()
  }
}


