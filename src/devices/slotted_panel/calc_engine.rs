/***********************************************************************************************************************
 * Porous Absorber Calculation Engine - Slotted Panel
 *
 * (c) Chris Whealy 2020
 */
use libm::{log, sin};
use num::complex::Complex;

use crate::devices::generic_device::{DeviceType, GenericDeviceInfo};
use crate::{
    chart::render,
    config::{chart::PlotAbsPoint, config_set::ConfigSet, trace_flags::trace_flag_for},
    trace::*,
    utils::maths_functions::*,
};

pub const MOD_NAME: &str = "devices::slotted_panel::calc_engine";

/***********************************************************************************************************************
 * Slotted Panel Calculation
 */
pub fn calculate_plot_points(config_set: &'_ ConfigSet) -> GenericDeviceInfo<'_> {
    const FN_NAME: &str = "calculate_plot_points";

    let trace_boundary = make_boundary_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME, FN_NAME);
    let trace = make_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME, FN_NAME);

    trace_boundary(TraceAction::Enter);

    let air = &config_set.air_config;
    let cavity = &config_set.cavity_config;
    let panel = config_set.panel_config.as_ref().unwrap().panel_slotted.as_ref().unwrap();
    let porous = config_set.porous_config.as_ref().unwrap();

    // Calculate apparent panel thickness
    let end_correction_delta = -log(sin(TAU * panel.porosity / 4.0)) * 2.0 / TAU;
    let end_corrected_panel_thickness = panel.thickness + (2.0 * panel.slot_width * end_correction_delta);

    trace(format!("End correction delta          = {}", end_correction_delta));
    trace(format!("End corrected panel thickness = {}", end_corrected_panel_thickness));

    // Calculate resistance terms
    let resistance_at_backing = porous.sigma as f64 * porous.thickness;
    let resistance_at_panel = resistance_at_backing * panel.porosity;
    let mass_term_for_air = end_corrected_panel_thickness * air.density / panel.porosity;

    trace(format!("Resistance at backing = {}", resistance_at_backing));
    trace(format!("Resistance at panel   = {}", resistance_at_panel));
    trace(format!("Mass term for air     = {}", mass_term_for_air));

    let abs_info = config_set.chart_config.frequencies.iter().fold(
        GenericDeviceInfo::new(DeviceType::SlottedPanelAbsorber, Some(panel), None, None, Some(porous), &cavity),
        |mut acc, frequency| {
            let (abs_no_air_gap, abs_against_panel, abs_against_backing) = calculate_plot_point(
                *frequency,
                &config_set,
                end_corrected_panel_thickness,
                resistance_at_panel,
                resistance_at_backing,
                mass_term_for_air,
            );

            // Build the vectors of plot points for each absorber type
            // The order of plot_points entries in the abs_series vector must match the order used in the render module by
            // function plot_generic_device when calculating the series_data vector.  The correct vector of plot_points must
            // be passed to function render::draw_splines
            acc.abs_series[0].plot_points.push(PlotAbsPoint {
                at: render::constants::ORIGIN,
                freq: *frequency,
                abs: abs_no_air_gap,
            });
            acc.abs_series[1].plot_points.push(PlotAbsPoint {
                at: render::constants::ORIGIN,
                freq: *frequency,
                abs: abs_against_panel,
            });
            acc.abs_series[2].plot_points.push(PlotAbsPoint {
                at: render::constants::ORIGIN,
                freq: *frequency,
                abs: abs_against_backing,
            });

            acc
        },
    );

    trace_boundary(TraceAction::Exit);
    abs_info
}

/***********************************************************************************************************************
 * Reducer function to calculate the absorption of a slotted panel absorber at a specific frequency
 */
fn calculate_plot_point(
    frequency: f64,
    config_set: &ConfigSet,
    ec_panel_thickness: f64,
    resistance_at_panel: f64,
    resistance_at_backing: f64,
    mass_term_for_air: f64,
) -> (f64, f64, f64) {
    const FN_NAME: &str = "calculate_plot_point";
    let trace_boundary = make_boundary_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME, FN_NAME);
    let trace = make_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME, FN_NAME);

    trace_boundary(TraceAction::Enter);

    let air_cfg = &config_set.air_config;
    let cavity_cfg = &config_set.cavity_config;
    let porous_cfg = config_set.porous_config.as_ref().unwrap();

    // Frequently used intermediate values
    let i: Complex<f64> = Complex::new(0.0, 1.0);
    let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

    // Wave number in air and angular frequency
    let k_air = wave_no_in_air(&air_cfg, frequency);
    let omega = f_ang(frequency);
    trace(format!("Wave number       = {}", k_air));
    trace(format!("Angular frequency = {}", omega));

    // Characteristic absorber impedance and wave number
    let (z_abs, wave_no_abs) = absorber_props(&air_cfg, porous_cfg, &frequency);
    trace(format!("Characteristic impedance = {}", z_abs));
    trace(format!("Complex wave number      = {}", wave_no_abs));

    // Intermediate terms
    let inter1 = k_air * ec_panel_thickness;
    let cot_inter1 = inter1.cos() / inter1.sin();
    trace(format!("cot(k air * t panel) = {}", cot_inter1));

    let inter2 = k_air * cavity_cfg.air_gap;
    let cot_inter2 = inter2.cos() / inter2.sin();
    trace(format!("cot(k cmplx_abs * t air) = {}", cot_inter2));

    let inter3 = wave_no_abs * porous_cfg.thickness;
    let cot_inter3 = inter3.cos() / inter3.sin();
    trace(format!("cot(complex_wave_no * t cmplx_abs) = {}", cot_inter3));

    let inter4 = wave_no_abs * (cavity_cfg.air_gap + porous_cfg.thickness);
    let cot_inter4 = inter4.cos() / inter4.sin();
    trace(format!("cot(complex_wave_no * total depth) = {}", cot_inter4));

    let mass_term_for_slotted_panel = i * ((omega * mass_term_for_air) - (air_cfg.impedance * cot_inter1));
    trace(format!("Mass term for air in slotted panel = {}", mass_term_for_slotted_panel));

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Absorber against panel
    let abs_against_panel_z1 = minus_i * air_cfg.impedance * cot_inter2;
    let abs_against_panel_z2 = ((minus_i * abs_against_panel_z1 * z_abs * cot_inter3) + (z_abs * z_abs))
        / (abs_against_panel_z1 - (i * z_abs * cot_inter3));
    let abs_against_panel_z3 = resistance_at_panel + mass_term_for_slotted_panel + abs_against_panel_z2;

    let abs_against_panel_refl = difference_over_sum(abs_against_panel_z3, air_cfg.impedance);
    let abs_against_panel_alpha = reflectivity_as_alpha(abs_against_panel_refl);

    trace(format!("Absorber against panel z1 = {}", abs_against_panel_z1));
    trace(format!("Absorber against panel z2 = {}", abs_against_panel_z2));
    trace(format!("Overall impedance         = {}", abs_against_panel_z3));

    trace(format!("Absorber against panel reflection = {}", abs_against_panel_refl));
    trace(format!("Absorber against panel absorption = {}", abs_against_panel_alpha));

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Absorber against backing
    let abs_against_backing_z1 = minus_i * z_abs * cot_inter3;
    let abs_against_backing_z2 = ((minus_i * abs_against_backing_z1 * air_cfg.impedance * cot_inter2)
        + (air_cfg.impedance * air_cfg.impedance))
        / (abs_against_backing_z1 - (i * air_cfg.impedance * cot_inter2));
    let abs_against_backing_z3 = resistance_at_backing + mass_term_for_slotted_panel + abs_against_backing_z2;

    let abs_against_backing_refl = difference_over_sum(abs_against_backing_z3, air_cfg.impedance);
    let abs_against_backing_alpha = reflectivity_as_alpha(abs_against_backing_refl);

    trace(format!("Absorber against backing z1 = {}", abs_against_backing_z1));
    trace(format!("Absorber against backing z2 = {}", abs_against_backing_z2));
    trace(format!("Absorber against backing z3 = {}", abs_against_backing_z3));

    trace(format!("Absorber against backing reflection = {}", abs_against_backing_refl));
    trace(format!("Absorber against backing absorption = {}", abs_against_backing_alpha));

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Absorber with no air gap
    let no_air_gap_z1 = minus_i * z_abs * cot_inter4;
    let no_air_gap_z2 = resistance_at_panel + mass_term_for_slotted_panel + no_air_gap_z1;

    let no_air_gap_refl = difference_over_sum(no_air_gap_z2, air_cfg.impedance);
    let no_air_gap_alpha = reflectivity_as_alpha(no_air_gap_refl);

    trace(format!("No air gap z1 = {}", no_air_gap_z1));
    trace(format!("No air gap z2 = {}", no_air_gap_z2));

    trace(format!("No air gap reflection = {}", no_air_gap_refl));
    trace(format!("No air gap absorption = {}", no_air_gap_alpha));

    trace_boundary(TraceAction::Exit);
    (no_air_gap_alpha, abs_against_panel_alpha, abs_against_backing_alpha)
}
