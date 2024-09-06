/***********************************************************************************************************************
 * Porous Absorber Calculator - Microperforated Panel Absorption Device
 *
 * (c) Chris Whealy 2020
 */
pub mod calc_engine;

use wasm_bindgen::JsValue;
use calc_engine::calculate_plot_points;

use crate::{
    chart::constants::chart_title_at_incident_angle,
    config::{
        air::AirConfig,
        cavity::CavityConfig,
        chart::ChartConfig,
        config_set::{ConfigSet, PanelConfigSet},
        panel_microperforated::MicroperforatedPanelConfig,
        sound::SoundConfig,
        trace_flags::trace_flag_for,
        GenericError,
    },
    trace::*,
    MicroperforatedPanelArgs,
};

pub const MOD_NAME: &str = "devices::microperforated_panel";

/***********************************************************************************************************************
 * Handle incoming arguments for calculating the absorption of a micro-perforated panel absorption device
 */
pub fn prepare(arg_obj: MicroperforatedPanelArgs) -> JsValue {
    let trace_boundary = make_boundary_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME, "prepare");
    trace_boundary(TraceAction::Enter);

    // Empty return data structure
    let mut error_msgs: Vec<String> = vec![];

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Construct set of configuration structs
    let panel_config_set = PanelConfigSet {
        panel_microperforated: Some(
            MicroperforatedPanelConfig::new(arg_obj.panel_thickness_mm, arg_obj.repeat_distance_mm, arg_obj.hole_radius_mm, arg_obj.porosity)
                .unwrap_or_else(|err: GenericError| {
                    error_msgs.push(err.to_string());
                    MicroperforatedPanelConfig::default()
                }),
        ),

        panel_perforated: None,
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

        sound_config: Some(SoundConfig::new(arg_obj.angle).unwrap_or_else(|err: GenericError| {
            error_msgs.push(err.to_string());
            SoundConfig::default()
        })),

        panel_config: Some(panel_config_set),
        porous_config: None,
    };

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // If there are no error messages, then calculate the absorption values, plot the graph and return the placeholder
    // value "Ok", else return the array of error messages
    let series_data = if error_msgs.is_empty() {
        let absorber_info = calculate_plot_points(&config_set);

        // Plot the graph
        let chart_info = crate::chart::render::generic_device(
            absorber_info,
            &config_set.chart_config,
            &chart_title_at_incident_angle(
                crate::chart::constants::CHART_TITLE_OVERALL_ABS,
                config_set.sound_config.as_ref().unwrap().angle,
            ),
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
