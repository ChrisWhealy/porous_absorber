/***********************************************************************************************************************
 * Porous Absorber Calculator - Rigid Backed Porous Absorption Device
 *
 * (c) Chris Whealy 2020
 */
use wasm_bindgen::JsValue;

use crate::{
    calc_engine::porous_absorber,
    config::{
        air::AirConfig, cavity::CavityConfig, chart::ChartConfig, config_set::ConfigSet,
        porous_layer::PorousLayerConfig, sound::SoundConfig, trace_flags::trace_flag_for, GenericError,
    },
    trace::*,
    PorousAbsorberArgs,
};

pub const MOD_NAME: &str = "devices::porous_absorber";

/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a rigid backed porous absorption device
 */
pub fn do_porous_absorber_device(arg_obj: PorousAbsorberArgs) -> JsValue {
    let trace_boundary = make_boundary_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME, "do_porous_absorber_device");
    trace_boundary(TraceAction::Enter);

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
        air_config: AirConfig::new(air_temp, air_pressure).unwrap_or_else(|err: GenericError| {
            error_msgs.push(err.to_string());
            AirConfig::default()
        }),

        cavity_config: CavityConfig::new(air_gap_mm).unwrap_or_else(|err: GenericError| {
            error_msgs.push(err.to_string());
            CavityConfig::default()
        }),

        chart_config: ChartConfig::new(graph_start_freq, smooth_curve, subdivision, show_diagram).unwrap_or_else(
            |err: GenericError| {
                error_msgs.push(err.to_string());
                ChartConfig::default()
            },
        ),

        // Variable configuration
        sound_config: Some(SoundConfig::new(angle).unwrap_or_else(|err: GenericError| {
            error_msgs.push(err.to_string());
            SoundConfig::default()
        })),

        panel_config: None,

        porous_config: Some(PorousLayerConfig::new(absorber_thickness_mm, flow_resistivity).unwrap_or_else(
            |err: GenericError| {
                error_msgs.push(err.to_string());
                PorousLayerConfig::default()
            },
        )),
    };

    // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
    // value "Ok", else return the array of error messages
    let series_data = if error_msgs.is_empty() {
        let absorber_info = porous_absorber::calculate(&config_set);

        // Plot the graph
        let chart_info = crate::chart::render::generic_device(
            absorber_info,
            &config_set.chart_config,
            &crate::chart::constants::chart_title_at_incident_angle(
                crate::chart::constants::CHART_TITLE_OVERALL_ABS,
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
