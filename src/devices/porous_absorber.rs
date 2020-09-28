// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Rigid Backed Porous Absorption Device
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************
use wasm_bindgen::JsValue;

use crate::structs::config_air::{AirConfig, AirError};
use crate::structs::config_cavity::{CavityConfig, CavityError};
use crate::structs::config_display::{DisplayConfig, DisplayError};
use crate::structs::config_porous_layer::{PorousLayerConfig, PorousLayerError};
use crate::structs::config_sound::{SoundConfig, SoundError};

use crate::calc_engine::porous_absorber;
use crate::render;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use crate::Trace;

const LIB_NAME: &str = "devices::porous_absorber";
const TRACE_ACTIVE: bool = false;

/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a rigid backed porous absorption device
 */
pub fn do_porous_absorber_device(wasm_arg_obj: JsValue) -> JsValue {
  const FN_NAME: &str = "do_porous_absorber_device";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(Some(true));

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

  // Parse arguments to the required data types
  let absorber_thickness_mm: u16 = arg_obj.absorber_thickness_mm.parse().unwrap();
  let flow_resistivity: u32 = arg_obj.flow_resistivity.parse().unwrap();
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

  let sound_cfg = SoundConfig::new(angle).unwrap_or_else(|err: SoundError| {
    error_msgs.push(err.to_string());
    SoundConfig::default()
  });

  let porous_cfg =
    PorousLayerConfig::new(absorber_thickness_mm, flow_resistivity).unwrap_or_else(|err: PorousLayerError| {
      error_msgs.push(err.to_string());
      PorousLayerConfig::default()
    });

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let return_value = if error_msgs.is_empty() {
    let absorber_info = porous_absorber::calculate(&air_cfg, &cavity_cfg, &display_cfg, &sound_cfg, &porous_cfg);

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

/***********************************************************************************************************************
 * Arguments required by function do_porous_absorber_device
 */
#[derive(Deserialize)]
struct PorousAbsorberArgs {
  absorber_thickness_mm: String, // Internally treated as u16
  flow_resistivity: String,      // Internally treated as u32
  air_gap_mm: String,            // Internally treated as u16
  angle: String,                 // Internally treated as u16
  graph_start_freq: String,      // Internally treated as f64
  smooth_curve: String,          // Internally treated as bool
  subdivision: String,           // Internally treated as u16
  show_diagram: String,          // Internally treated as bool
  air_temp: String,              // Internally treated as i16
  air_pressure: String,          // Internally treated as f64
}
