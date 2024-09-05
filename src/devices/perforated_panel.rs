/***********************************************************************************************************************
 * Porous Absorber Calculator - Perforated Panel Absorption Device
 *
 * (c) Chris Whealy 2020
 */
use wasm_bindgen::JsValue;

use crate::{
    calc_engine::perforated_panel,
    config::{
        air::AirConfig,
        cavity::CavityConfig,
        chart::ChartConfig,
        config_set::{ConfigSet, PanelConfigSet},
        panel_perforated::PerforatedPanelConfig,
        porous_layer::PorousLayerConfig,
        trace_flags::trace_flag_for,
        GenericError,
    },
    trace::*,
    PerforatedPanelArgs,
};

pub const MOD_NAME: &str = "devices::perforated_panel";

/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a perforated panel absorption device
 */
pub fn calculate(arg_obj: PerforatedPanelArgs) -> JsValue {
    let trace_boundary = make_boundary_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME, "do_perforated_panel_device");
    trace_boundary(TraceAction::Enter);

    // Parse String arguments to the required data types
    // let panel_thickness_mm = arg_obj.panel_thickness_mm.parse::<f64>().unwrap();
    // let repeat_distance_mm = arg_obj.repeat_distance_mm.parse::<f64>().unwrap();
    // let hole_radius_mm = arg_obj.hole_radius_mm.parse::<f64>().unwrap();
    // let porosity = arg_obj.porosity.parse::<f64>().unwrap();
    // let absorber_thickness_mm = arg_obj.absorber_thickness_mm.parse::<u16>().unwrap();
    // let flow_resistivity = arg_obj.flow_resistivity.parse::<u32>().unwrap();
    // let air_gap_mm = arg_obj.air_gap_mm.parse::<u16>().unwrap();
    // let graph_start_freq = arg_obj.graph_start_freq.parse::<f64>().unwrap();
    // let smooth_curve = arg_obj.smooth_curve.parse::<bool>().unwrap();
    // let subdivision = arg_obj.subdivision.parse::<u16>().unwrap();
    // let show_diagram = arg_obj.show_diagram.parse::<bool>().unwrap();
    // let air_temp = arg_obj.air_temp.parse::<i16>().unwrap();
    // let air_pressure = arg_obj.air_pressure.parse::<f64>().unwrap();

    // Empty return data structure
    let mut error_msgs: Vec<String> = vec![];

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Construct set of configuration structs
    let panel_config_set = PanelConfigSet {
        panel_microperforated: None,
        panel_perforated: Some(
            PerforatedPanelConfig::new(arg_obj.panel_thickness_mm, arg_obj.repeat_distance_mm, arg_obj.hole_radius_mm, arg_obj.porosity)
                .unwrap_or_else(|err: GenericError| {
                    error_msgs.push(err.to_string());
                    PerforatedPanelConfig::default()
                }),
        ),
        panel_slotted: None,
    };

    let config_set = ConfigSet {
        // Required configuration
        air_config: AirConfig::new(arg_obj.air_temp, arg_obj.air_pressure).unwrap_or_else(|err: GenericError| {
            error_msgs.push(err.to_string());
            AirConfig::default()
        }),

        cavity_config: CavityConfig::new(arg_obj.air_gap_mm).unwrap_or_else(|err: GenericError| {
            error_msgs.push(err.to_string());
            CavityConfig::default()
        }),

        chart_config: ChartConfig::new(arg_obj.graph_start_freq, arg_obj.smooth_curve, arg_obj.subdivision, arg_obj.show_diagram).unwrap_or_else(
            |err: GenericError| {
                error_msgs.push(err.to_string());
                ChartConfig::default()
            },
        ),

        // Variable configuration
        sound_config: None,

        panel_config: Some(panel_config_set),
        porous_config: Some(PorousLayerConfig::new(arg_obj.absorber_thickness_mm, arg_obj.flow_resistivity).unwrap_or_else(
            |err: GenericError| {
                error_msgs.push(err.to_string());
                PorousLayerConfig::default()
            },
        )),
    };

    // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
    // value "Ok", else return the array of error messages
    let series_data = if error_msgs.is_empty() {
        let absorber_info = perforated_panel::calculate(&config_set);

        // Plot the graph
        let chart_info = crate::chart::render::generic_device(
            absorber_info,
            &config_set.chart_config,
            crate::chart::constants::CHART_TITLE_NORMAL_INCIDENCE,
        );

        serde_wasm_bindgen::to_value(&chart_info).unwrap()
    } else {
        // Serialize the error message(s)
        serde_wasm_bindgen::to_value(&error_msgs).unwrap()
    };

    trace_boundary(TraceAction::Exit);

    // Return either the {X,Y} values of plot points or the error messages back to JavaScript
    series_data
}
