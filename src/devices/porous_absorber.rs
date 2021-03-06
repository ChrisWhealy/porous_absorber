/***********************************************************************************************************************
 * Porous Absorber Calculator - Rigid Backed Porous Absorption Device
 *
 * (c) Chris Whealy 2020
 */
use wasm_bindgen::JsValue;

use crate::config::{
  air::{AirConfig, AirError},
  cavity::{CavityConfig, CavityError},
  chart::{ChartConfig, ChartError},
  config_set::ConfigSet,
  porous_layer::{PorousLayerConfig, PorousLayerError},
  sound::{SoundConfig, SoundError},
};

use crate::calc_engine::porous_absorber;
use crate::chart;

/***********************************************************************************************************************
 * Trace functionality
 */
use crate::{
  config::trace_flags::trace_flag_for,
  trace::{
    function_boundaries::{make_boundary_trace_fn, TraceAction},
    function_data::make_trace_fn,
  },
};

pub const MOD_NAME: &str = "devices::porous_absorber";

/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a rigid backed porous absorption device
 */
pub fn do_porous_absorber_device(wasm_arg_obj: JsValue) -> JsValue {
  const FN_NAME: &str = "do_porous_absorber_device";

  let trace_boundary = make_boundary_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME.to_string(), FN_NAME.to_string());
  let trace = make_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(TraceAction::Enter);

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

  // Parse String arguments to the required data types
  let absorber_thickness_mm = arg_obj.absorber_thickness_mm.parse::<u16>().unwrap();
  let flow_resistivity = arg_obj.flow_resistivity.parse::<u32>().unwrap();
  let air_gap_mm = arg_obj.air_gap_mm.parse::<u16>().unwrap();
  let angle = arg_obj.angle.parse::<u16>().unwrap();
  let graph_start_freq = arg_obj.graph_start_freq.parse::<f64>().unwrap();
  let smooth_curve = arg_obj.smooth_curve.parse::<bool>().unwrap();
  let subdivision = arg_obj.subdivision.parse::<u16>().unwrap();
  let show_diagram = arg_obj.show_diagram.parse::<bool>().unwrap();
  let air_temp = arg_obj.air_temp.parse::<i16>().unwrap();
  let air_pressure = arg_obj.air_pressure.parse::<f64>().unwrap();

  // Empty return data structure
  let mut error_msgs: Vec<String> = vec![];

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Construct set of configuration structs
  let config_set = ConfigSet {
    // Required configuration
    air_config: AirConfig::new(air_temp, air_pressure).unwrap_or_else(|err: AirError| {
      error_msgs.push(err.to_string());
      AirConfig::default()
    }),

    cavity_config: CavityConfig::new(air_gap_mm).unwrap_or_else(|err: CavityError| {
      error_msgs.push(err.to_string());
      CavityConfig::default()
    }),

    chart_config: ChartConfig::new(graph_start_freq, smooth_curve, subdivision, show_diagram).unwrap_or_else(
      |err: ChartError| {
        error_msgs.push(err.to_string());
        ChartConfig::default()
      },
    ),

    // Variable configuration
    sound_config: Some(SoundConfig::new(angle).unwrap_or_else(|err: SoundError| {
      error_msgs.push(err.to_string());
      SoundConfig::default()
    })),

    panel_config: None,

    porous_config: Some(
      PorousLayerConfig::new(absorber_thickness_mm, flow_resistivity).unwrap_or_else(|err: PorousLayerError| {
        error_msgs.push(err.to_string());
        PorousLayerConfig::default()
      }),
    ),
  };

  // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
  // value "Ok", else return the array of error messages
  let series_data = if error_msgs.is_empty() {
    let absorber_info = porous_absorber::calculate(&config_set);

    // Plot the graph
    let chart_info = chart::render::generic_device(
      absorber_info,
      &config_set.chart_config,
      &chart::constants::chart_title_at_incident_angle(
        chart::constants::CHART_TITLE_OVERALL_ABS,
        config_set.sound_config.as_ref().unwrap().angle,
      ),
    );

    JsValue::from_serde(&chart_info).unwrap()
  } else {
    // Serialize the error message(s)
    JsValue::from_serde(&error_msgs).unwrap()
  };

  trace_boundary(TraceAction::Exit);

  // Return either the {X,Y} values of plot points or the error messages back to JavaScript
  series_data
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
