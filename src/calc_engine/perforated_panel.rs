// *********************************************************************************************************************
// Porous Absorber Calculation Engine
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

use libm::{pow, sqrt};
use num::complex::Complex;

use crate::structs::{
  config_air::{AirConfig, AIR_VISCOSITY},
  config_cavity::CavityConfig,
  config_display::{DisplayConfig, PlotAbsPoint, SeriesData},
  config_porous_layer::PorousLayerConfig,
  generic_device::{DeviceType, GenericDeviceInfo},
  panel_perforated::PerforatedPanelConfig,
};

use crate::utils::maths_functions::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use crate::trace::Trace;

const LIB_NAME: &str = "calc_engine::perforated_panel";
const TRACE_ACTIVE: bool = false;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Constants
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const STR_NO_AIR_GAP: &str = "No Air Gap";
const STR_ABS_AGAINST_PANEL: &str = "Absorber Against Panel";
const STR_ABS_AGAINST_BACKING: &str = "Absorber Against Backing";

// *********************************************************************************************************************
// Perforated Panel
// *********************************************************************************************************************
pub fn calculate<'a>(
  air: &'a AirConfig,
  cavity: &'a CavityConfig,
  display: &'a DisplayConfig,
  panel: &'a PerforatedPanelConfig,
  porous: &'a PorousLayerConfig,
) -> GenericDeviceInfo<'a> {
  const FN_NAME: &str = "calculate";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(Some(true));

  // Calculate apparent panel thickness
  let end_correction_delta = 0.8 * (1.0 - 1.47 * sqrt(panel.porosity) + 0.47 * sqrt(pow(panel.porosity, 3.0)));
  let end_corrected_panel_thickness = panel.thickness + (2.0 * panel.hole_radius * end_correction_delta);

  trace(format!("End correction delta          = {}", &end_correction_delta));
  trace(format!(
    "End corrected panel thickness = {}",
    &end_corrected_panel_thickness
  ));

  let abs_info = display.frequencies.iter().fold(
    GenericDeviceInfo {
      device_type: DeviceType::PerforatedPanelAbsorber,
      abs_series: vec![
        SeriesData {
          name: STR_NO_AIR_GAP,
          plot_points: vec![],
        },
        SeriesData {
          name: STR_ABS_AGAINST_PANEL,
          plot_points: vec![],
        },
        SeriesData {
          name: STR_ABS_AGAINST_BACKING,
          plot_points: vec![],
        },
      ],
      sl_panel: None,
      pf_panel: Some(panel),
      mp_panel: None,
      porous_layer: Some(porous),
      cavity: Some(cavity),
    },
    |mut acc, frequency| {
      let (abs_no_air_gap, abs_against_panel, abs_against_backing) = do_perforated_panel_calc(
        *frequency,
        &air,
        &cavity,
        &panel,
        &porous,
        end_corrected_panel_thickness,
      );

      // Build the vectors of plot points for each absorber type
      // The order of plot_points entries in the abs_series vector must match the order used in the render module by
      // function plot_generic_device when calculating the series_data vector.  The correct vector of plot_points must
      // be passed to function render::draw_splines
      acc.abs_series[0].plot_points.push(PlotAbsPoint {
        x: 0.0,
        y: 0.0,
        freq: *frequency,
        abs: abs_no_air_gap,
      });
      acc.abs_series[1].plot_points.push(PlotAbsPoint {
        x: 0.0,
        y: 0.0,
        freq: *frequency,
        abs: abs_against_panel,
      });
      acc.abs_series[2].plot_points.push(PlotAbsPoint {
        x: 0.0,
        y: 0.0,
        freq: *frequency,
        abs: abs_against_backing,
      });

      acc
    },
  );

  trace_boundary(Some(false));
  abs_info
}

// *********************************************************************************************************************
// Reducer function to calculate the absorption of a perforated panel absorber at a specific frequency
// *********************************************************************************************************************
fn do_perforated_panel_calc(
  frequency: f64,
  air_cfg: &AirConfig,
  cavity_cfg: &CavityConfig,
  panel_cfg: &PerforatedPanelConfig,
  porous_cfg: &PorousLayerConfig,
  ec_panel_thickness: f64,
) -> (f64, f64, f64) {
  const FN_NAME: &str = "do_perforated_panel_calc";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(Some(true));

  // Frequently used intermediate values
  let i: Complex<f64> = Complex::new(0.0, 1.0);
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

  // Wave number in air and angular frequency
  let k_air = wave_no_in_air(air_cfg, &frequency);
  let omega = f_ang(frequency);
  trace(format!("Wave number       = {}", k_air));
  trace(format!("Angular frequency = {}", omega));

  // Characteristic absorber impedance and wave number
  let (z_abs, wave_no_abs) = absorber_props(air_cfg, porous_cfg, &frequency);
  trace(format!("Characteristic impedance = {}", z_abs));
  trace(format!("Complex wave number      = {}", wave_no_abs));

  // Intermediate terms
  let inter1 = k_air * cavity_cfg.air_gap;
  let cot_inter1 = inter1.cos() / inter1.sin();
  let inter2 = wave_no_abs * porous_cfg.thickness;
  let cot_inter2 = inter2.cos() / inter2.sin();

  trace(format!("k air * t air      = {}", inter1));
  trace(format!("cot(k air * t air) = {}", cot_inter1));
  trace(format!("k cmplx_abs * t cmplx_abs      = {}", inter2));
  trace(format!("cot(k cmplx_abs * t cmplx_abs) = {}", cot_inter2));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against panel
  let abs_against_panel_z1 = minus_i * air_cfg.impedance * cot_inter1;
  let abs_against_panel_z2 = ((minus_i * abs_against_panel_z1 * z_abs * cot_inter2) + (z_abs * z_abs))
    / (abs_against_panel_z1 - (i * z_abs * cot_inter2));
  let surface_resistence = (air_cfg.density / panel_cfg.porosity)
    * sqrt(8.0 * AIR_VISCOSITY * omega)
    * (1.0 + ec_panel_thickness / (2.0 * panel_cfg.hole_radius));
  let abs_against_panel_z3 = ((i / panel_cfg.porosity) * ec_panel_thickness * omega * air_cfg.density)
    + abs_against_panel_z2
    + surface_resistence;

  let abs_against_panel_refl = difference_over_sum(abs_against_panel_z3, air_cfg.impedance);
  let abs_against_panel_alpha = reflectivity_as_alpha(abs_against_panel_refl);

  trace(format!("Absorber against panel z1 = {}", abs_against_panel_z1));
  trace(format!("Absorber against panel z2 = {}", abs_against_panel_z2));
  trace(format!("Surface resistance        = {}", surface_resistence));
  trace(format!("Overall impedence         = {}", abs_against_panel_z3));

  trace(format!(
    "Absorber against panel reflection = {}",
    abs_against_panel_refl
  ));
  trace(format!(
    "Absorber against panel absorption = {}",
    abs_against_panel_alpha
  ));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against backing
  let abs_against_backing_z1 = minus_i * z_abs * cot_inter2;
  let abs_against_backing_z2 = ((minus_i * abs_against_backing_z1 * air_cfg.impedance * cot_inter1)
    + (air_cfg.impedance * air_cfg.impedance))
    / (abs_against_backing_z1 - (i * air_cfg.impedance * cot_inter1));

  let abs_against_backing_z3 = ((air_cfg.density / panel_cfg.porosity)
    * sqrt(8.0 * AIR_VISCOSITY * omega)
    * ((panel_cfg.thickness / 2.0 * panel_cfg.hole_radius) + 1.0))
    + ((ec_panel_thickness / panel_cfg.porosity) * i * omega * air_cfg.density)
    + abs_against_backing_z2;

  let abs_against_backing_refl = difference_over_sum(abs_against_backing_z3, air_cfg.impedance);
  let abs_against_backing_alpha = reflectivity_as_alpha(abs_against_backing_refl);

  trace(format!("Absorber against backing z1 = {}", abs_against_backing_z1));
  trace(format!("Absorber against backing z2 = {}", abs_against_backing_z2));
  trace(format!("Absorber against backing z3 = {}", abs_against_backing_z3));

  trace(format!(
    "Absorber against backing reflection = {}",
    abs_against_backing_refl
  ));
  trace(format!(
    "Absorber against backing absorption = {}",
    abs_against_backing_alpha
  ));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber with no air gap
  let inter3 = wave_no_abs * (porous_cfg.thickness + cavity_cfg.air_gap);
  let cot_inter3 = inter3.cos() / inter3.sin();

  let no_air_gap_z1 = minus_i * z_abs * cot_inter3;
  let no_air_gap_z2 = (i * omega * air_cfg.density * (ec_panel_thickness / panel_cfg.porosity)) + no_air_gap_z1;

  let no_air_gap_refl = difference_over_sum(no_air_gap_z2, air_cfg.impedance);
  let no_air_gap_alpha = reflectivity_as_alpha(no_air_gap_refl);

  trace(format!("cot(complex wave no * cavity depth) = {}", cot_inter3));
  trace(format!("No air gap z1 = {}", no_air_gap_z1));
  trace(format!("No air gap z2 = {}", no_air_gap_z2));

  trace(format!("No air gap reflection = {}", no_air_gap_refl));
  trace(format!("No air gap absorption = {}", no_air_gap_alpha));

  trace_boundary(Some(false));
  (no_air_gap_alpha, abs_against_panel_alpha, abs_against_backing_alpha)
}
