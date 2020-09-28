// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Microperforated Panel Absorption Device
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************
use wasm_bindgen::JsValue;

use crate::structs::config_air::{AirConfig, AirError};
use crate::structs::config_cavity::{CavityConfig, CavityError};
use crate::structs::config_display::{DisplayConfig, DisplayError};
use crate::structs::config_sound::{SoundConfig, SoundError};

use crate::structs::panel_microperforated::{MicroperforatedPanelConfig, MicroperforatedPanelError};

use crate::calc_engine;
use crate::render;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use crate::Trace;

const LIB_NAME: &str = "device_microperforated_panel";
const TRACE_ACTIVE: bool = false;

// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************

/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a microperforated panel absorption device
 */
pub fn do_microperforated_panel_device(wasm_arg_obj: JsValue) -> JsValue {
  const FN_NAME: &str = "do_microperforated_panel_device";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(Some(true));

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

  // Parse arguments to the required data types
  let panel_thickness_mm: f64 = arg_obj.panel_thickness_mm.parse().unwrap();
  let repeat_distance_mm: f64 = arg_obj.repeat_distance_mm.parse().unwrap();
  let hole_radius_mm: f64 = arg_obj.hole_radius_mm.parse().unwrap();
  let porosity: f64 = arg_obj.porosity.parse().unwrap();
  let air_gap_mm: u16 = arg_obj.air_gap_mm.parse().unwrap();
  let angle: u16 = arg_obj.angle.parse().unwrap();
  let graph_start_freq: f64 = arg_obj.graph_start_freq.parse().unwrap();
  let smooth_curve: bool = arg_obj.smooth_curve.parse().unwrap();
  let subdivision: u16 = arg_obj.subdivision.parse().unwrap();
  let show_diagram: bool = arg_obj.show_diagram.parse().unwrap();
  let air_temp: i16 = arg_obj.air_temp.parse().unwrap();
  let air_pressure: f64 = arg_obj.air_pressure.parse().unwrap();

  // Empty return data structure
  let mut error_msgs: Vec<String> = vec![];

  // Construct configuration structs
  let air_cfg = AirConfig::new(air_temp, air_pressure).unwrap_or_else(|err: AirError| {
    error_msgs.push(err.to_string());
    AirConfig::default()
  });

  let cavity_cfg = CavityConfig::new(air_gap_mm).unwrap_or_else(|err: CavityError| {
    error_msgs.push(err.to_string());
    CavityConfig::default()
  });

  let display_cfg = DisplayConfig::new(graph_start_freq, smooth_curve, subdivision, show_diagram).unwrap_or_else(
    |err: DisplayError| {
      error_msgs.push(err.to_string());
      DisplayConfig::default()
    },
  );

  let panel_cfg = MicroperforatedPanelConfig::new(panel_thickness_mm, repeat_distance_mm, hole_radius_mm, porosity)
    .unwrap_or_else(|err: MicroperforatedPanelError| {
      error_msgs.push(err.to_string());
      MicroperforatedPanelConfig::default()
    });

  let sound_cfg = SoundConfig::new(angle).unwrap_or_else(|err: SoundError| {
    error_msgs.push(err.to_string());
    SoundConfig::default()
  });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.is_empty() {
    let absorber_info =
      calc_engine::calculate_microperforated_panel(&air_cfg, &cavity_cfg, &display_cfg, &panel_cfg, &sound_cfg);

    // Plot the graph
    let chart_info = render::plot_generic_device(
      absorber_info,
      &display_cfg,
      &format!("Overall absorption at {}°", sound_cfg.angle),
    );

    JsValue::from_serde(&chart_info).unwrap()
  } else {
    // Serialize the error message(s)
    JsValue::from_serde(&error_msgs).unwrap()
  };

  trace_boundary(Some(false));

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
 * Arguments required by function do_microperforated_panel_device
 */
#[derive(Deserialize)]
struct MicroperforatedPanelArgs {
  panel_thickness_mm: String, // Internally treated as f64
  repeat_distance_mm: String, // Internally treated as f64
  hole_radius_mm: String,     // Internally treated as f64
  porosity: String,           // Internally treated as f64
  air_gap_mm: String,         // Internally treated as u16
  angle: String,              // Internally treated as u16
  graph_start_freq: String,   // Internally treated as f64
  smooth_curve: String,       // Internally treated as bool
  subdivision: String,        // Internally treated as u16
  show_diagram: String,       // Internally treated as bool
  air_temp: String,           // Internally treated as i16
  air_pressure: String,       // Internally treated as f64
}
