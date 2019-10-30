// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Slotted Panel Absorption Device
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
use std::error::Error;

use wasm_bindgen::JsValue;

use crate::structs::config_air::{AirConfig, AirError};
use crate::structs::config_cavity::{CavityConfig, CavityError};
use crate::structs::config_display::{DisplayConfig, DisplayError};
use crate::structs::config_porous_layer::{PorousLayerConfig, PorousLayerError};

use crate::structs::panel_slotted::{SlottedPanelConfig, SlottedPanelError};

use crate::calc_engine;
use crate::render;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use crate::Trace;

const LIB_NAME     : &str  = &"device_slotted_panel";
const TRACE_ACTIVE : &bool = &false;



// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a slotted panel absorption device
 */
pub fn do_slotted_panel_device(wasm_arg_obj : &JsValue) -> JsValue {
  const FN_NAME : &str = &"do_slotted_panel_device";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Parse object received from JavaScript
  let arg_obj : SlottedPanelArgs = wasm_arg_obj.into_serde().unwrap();

  // What values did we receive from JavaScript?
  trace(&format!("panel_thickness_mm    = {}", arg_obj.panel_thickness_mm));
  trace(&format!("slot_distance_mm      = {}", arg_obj.slot_distance_mm));
  trace(&format!("slot_width_mm         = {}", arg_obj.slot_width_mm));
  trace(&format!("slotted_porosity      = {}", arg_obj.slotted_porosity));
  trace(&format!("absorber_thickness_mm = {}", arg_obj.absorber_thickness_mm));
  trace(&format!("flow_resistivity      = {}", arg_obj.flow_resistivity));
  trace(&format!("air_gap_mm            = {}", arg_obj.air_gap_mm));
  trace(&format!("graph_start_freq      = {}", arg_obj.graph_start_freq));
  trace(&format!("smooth_curve          = {}", arg_obj.smooth_curve));
  trace(&format!("subdivision           = {}", arg_obj.subdivision));
  trace(&format!("air_temp              = {}", arg_obj.air_temp));
  trace(&format!("air_pressure          = {}", arg_obj.air_pressure));

  // Parse arguments to the required data types
  let panel_thickness_mm    : f64  = arg_obj.panel_thickness_mm.parse().unwrap();
  let slot_distance_mm      : f64  = arg_obj.slot_distance_mm.parse().unwrap();
  let slot_width_mm         : f64  = arg_obj.slot_width_mm.parse().unwrap();
  let slotted_porosity      : f64  = arg_obj.slotted_porosity.parse().unwrap();
  let absorber_thickness_mm : u16  = arg_obj.absorber_thickness_mm.parse().unwrap();
  let flow_resistivity      : u32  = arg_obj.flow_resistivity.parse().unwrap();
  let air_gap_mm            : u16  = arg_obj.air_gap_mm.parse().unwrap();
  let graph_start_freq      : f64  = arg_obj.graph_start_freq.parse().unwrap();
  let smooth_curve          : bool = arg_obj.smooth_curve.parse().unwrap();
  let subdivision           : u16  = arg_obj.subdivision.parse().unwrap();
  let air_temp              : i16  = arg_obj.air_temp.parse().unwrap();
  let air_pressure          : f64  = arg_obj.air_pressure.parse().unwrap();

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

  let display_cfg = DisplayConfig::new(graph_start_freq, smooth_curve, subdivision).unwrap_or_else(|err: DisplayError| {
    error_msgs.push(String::from(err.description()));
    DisplayConfig::default()
  });

  let panel_cfg = SlottedPanelConfig::new(panel_thickness_mm, slot_distance_mm, slot_width_mm, slotted_porosity).unwrap_or_else(|err: SlottedPanelError| {
    error_msgs.push(String::from(err.description()));
    SlottedPanelConfig::default()
  });

  let porous_cfg = PorousLayerConfig::new(absorber_thickness_mm, flow_resistivity).unwrap_or_else(|err: PorousLayerError| {
    error_msgs.push(String::from(err.description()));
    PorousLayerConfig::default()
  });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.len() == 0 {
    let absorber_info = calc_engine::calculate_slotted_panel(&air_cfg, &cavity_cfg, &display_cfg, &panel_cfg, &porous_cfg);
    
    // Plot the graph
    let chart_info = render::plot_generic_device(absorber_info, &display_cfg, &"Normal Incidence Absorption");

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



// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                 P R I V A T E   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************

/***********************************************************************************************************************
 * Arguments required by function do_slotted_panel_device
 */
#[derive(Deserialize)]
struct SlottedPanelArgs {
  panel_thickness_mm    : String    // Internally treated as f64
, slot_distance_mm      : String    // Internally treated as f64
, slot_width_mm         : String    // Internally treated as f64
, slotted_porosity      : String    // Internally treated as f64
, absorber_thickness_mm : String    // Internally treated as u16
, flow_resistivity      : String    // Internally treated as u32
, air_gap_mm            : String    // Internally treated as u16
, graph_start_freq      : String    // Internally treated as f64
, smooth_curve          : String    // Internally treated as bool
, subdivision           : String    // Internally treated as u16
, air_temp              : String    // Internally treated as i16
, air_pressure          : String    // Internally treated as f64
}
