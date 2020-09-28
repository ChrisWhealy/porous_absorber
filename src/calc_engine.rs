// *********************************************************************************************************************
// Porous Absorber Calculation Engine
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

use libm::{cos, fabs, log, pow, sin, sqrt};
use num::complex::Complex;
use std::f64::consts::PI;

use crate::structs::{
  config_air::{AirConfig, AIR_VISCOSITY},
  config_cavity::CavityConfig,
  config_display::{DisplayConfig, PlotAbsPoint, SeriesData},
  config_porous_layer::PorousLayerConfig,
  config_sound::SoundConfig,
  generic_device::{DeviceType, GenericDeviceInfo},
  panel_microperforated::MicroperforatedPanelConfig,
  panel_perforated::PerforatedPanelConfig,
  panel_slotted::SlottedPanelConfig,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use crate::trace::Trace;

const LIB_NAME: &str = &"calc_engine";
const TRACE_ACTIVE: &bool = &false;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Constants
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const PI_OVER_180: f64 = PI / 180.0;
const ONE_80_OVER_PI: f64 = 180.0 / PI;

const BESSEL_TOLERANCE: f64 = 0.000000001;
const BESSEL_PRECISION: f64 = 0.000000000001;

const STR_AIR_GAP: &str = &"Air Gap";
const STR_NO_AIR_GAP: &str = &"No Air Gap";
const STR_ABS_AGAINST_PANEL: &str = &"Absorber Against Panel";
const STR_ABS_AGAINST_BACKING: &str = &"Absorber Against Backing";
const STR_MP_PANEL: &str = &"Microperforated Panel";

// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************

// *********************************************************************************************************************
// Rigid Backed Porous Absorber
// *********************************************************************************************************************
pub fn calculate_porous_absorber<'a>(
  air: &'a AirConfig,
  cavity: &'a CavityConfig,
  display: &'a DisplayConfig,
  sound: &'a SoundConfig,
  porous: &'a PorousLayerConfig,
) -> GenericDeviceInfo<'a> {
  const FN_NAME: &str = &"calculate_porous_absorber";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let abs_info = display.frequencies.iter().fold(
    GenericDeviceInfo {
      device_type: DeviceType::RigidBackedPorousAbsorber,
      abs_series: vec![
        SeriesData {
          name: STR_AIR_GAP,
          plot_points: vec![],
        },
        SeriesData {
          name: STR_NO_AIR_GAP,
          plot_points: vec![],
        },
      ],
      sl_panel: None,
      pf_panel: None,
      mp_panel: None,
      porous_layer: Some(porous),
      cavity: Some(cavity),
    },
    |mut acc, frequency| {
      let (abs_no_air_gap, abs_air_gap) =
        do_porous_abs_calc(*frequency, &air, &cavity, &sound, &porous);

      // Build the vectors of plot points for each absorber type
      // The order of plot_points entries in the abs_series vector must match the order used in the render module by
      // function plot_generic_device when calculating the series_data vector.  The correct vector of plot_points must
      // be passed to function render::draw_splines
      acc.abs_series[0].plot_points.push(PlotAbsPoint {
        x: 0.0,
        y: 0.0,
        freq: *frequency,
        abs: abs_air_gap,
      });
      acc.abs_series[1].plot_points.push(PlotAbsPoint {
        x: 0.0,
        y: 0.0,
        freq: *frequency,
        abs: abs_no_air_gap,
      });

      acc
    },
  );

  trace_boundary(&Some(false));
  abs_info
}

// *********************************************************************************************************************
// Perforated Panel
// *********************************************************************************************************************
pub fn calculate_perforated_panel<'a>(
  air: &'a AirConfig,
  cavity: &'a CavityConfig,
  display: &'a DisplayConfig,
  panel: &'a PerforatedPanelConfig,
  porous: &'a PorousLayerConfig,
) -> GenericDeviceInfo<'a> {
  const FN_NAME: &str = &"calculate_perforated_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Calculate apparent panel thickness
  let end_correction_delta =
    0.8 * (1.0 - 1.47 * sqrt(panel.porosity) + 0.47 * sqrt(pow(panel.porosity, 3.0)));
  let end_corrected_panel_thickness =
    panel.thickness + (2.0 * panel.hole_radius * end_correction_delta);

  trace(&format!(
    "End correction delta          = {}",
    &end_correction_delta
  ));
  trace(&format!(
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

  trace_boundary(&Some(false));
  abs_info
}

// *********************************************************************************************************************
// Slotted Panel
// *********************************************************************************************************************
pub fn calculate_slotted_panel<'a>(
  air: &'a AirConfig,
  cavity: &'a CavityConfig,
  display: &'a DisplayConfig,
  panel: &'a SlottedPanelConfig,
  porous: &'a PorousLayerConfig,
) -> GenericDeviceInfo<'a> {
  const FN_NAME: &str = &"calculate_slotted_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Calculate apparent panel thickness
  let end_correction_delta = -log(sin(PI * panel.porosity / 2.0)) / PI;
  let end_corrected_panel_thickness =
    panel.thickness + (2.0 * panel.slot_width * end_correction_delta);

  trace(&format!(
    "End correction delta          = {}",
    &end_correction_delta
  ));
  trace(&format!(
    "End corrected panel thickness = {}",
    &end_corrected_panel_thickness
  ));

  // Calculate resistance terms
  let resistance_at_backing = porous.sigma as f64 * porous.thickness;
  let resistance_at_panel = resistance_at_backing * panel.porosity;
  let mass_term_for_air = end_corrected_panel_thickness * air.density / panel.porosity;

  trace(&format!(
    "Resistance at backing = {}",
    &resistance_at_backing
  ));
  trace(&format!("Resistance at panel   = {}", &resistance_at_panel));
  trace(&format!("Mass term for air     = {}", &mass_term_for_air));

  let abs_info = display.frequencies.iter().fold(
    GenericDeviceInfo {
      device_type: DeviceType::SlottedPanelAbsorber,
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
      sl_panel: Some(panel),
      pf_panel: None,
      mp_panel: None,
      porous_layer: Some(porous),
      cavity: Some(cavity),
    },
    |mut acc, frequency| {
      let (abs_no_air_gap, abs_against_panel, abs_against_backing) = do_slotted_panel_calc(
        *frequency,
        &air,
        &cavity,
        &porous,
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

  trace_boundary(&Some(false));
  abs_info
}

// *********************************************************************************************************************
// Microperforated Panel
// *********************************************************************************************************************
pub fn calculate_microperforated_panel<'a>(
  air: &'a AirConfig,
  cavity: &'a CavityConfig,
  display: &'a DisplayConfig,
  panel: &'a MicroperforatedPanelConfig,
  sound: &'a SoundConfig,
) -> GenericDeviceInfo<'a> {
  const FN_NAME: &str = &"calculate_microperforated_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let cos_angle = cos(sound.angle as f64 * PI / 180.0);

  let abs_info = display.frequencies.iter().fold(
    GenericDeviceInfo {
      device_type: DeviceType::MicroperforatedPanelAbsorber,
      abs_series: vec![SeriesData {
        name: STR_MP_PANEL,
        plot_points: vec![],
      }],
      sl_panel: None,
      pf_panel: None,
      mp_panel: Some(panel),
      porous_layer: None,
      cavity: Some(cavity),
    },
    |mut acc, frequency| {
      let abs_data = do_microperforated_panel_calc(*frequency, &air, &cavity, &panel, cos_angle);
      acc.abs_series[0].plot_points.push(PlotAbsPoint {
        x: 0.0,
        y: 0.0,
        freq: *frequency,
        abs: abs_data,
      });

      acc
    },
  );

  trace_boundary(&Some(false));
  abs_info
}

// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                 P R I V A T E   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************

// *********************************************************************************************************************
// Reducer function to calculate the absorption of a porous absorber at a specific frequency
// *********************************************************************************************************************
fn do_porous_abs_calc(
  frequency: f64,
  air_cfg: &AirConfig,
  cavity_cfg: &CavityConfig,
  sound_cfg: &SoundConfig,
  porous_cfg: &PorousLayerConfig,
) -> (f64, f64) {
  // Frequently used intermediate values
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

  let angle_rad = sound_cfg.angle as f64 * PI_OVER_180;
  let sin_phi: f64 = sin(angle_rad);
  let cos_phi: f64 = cos(angle_rad);

  // Wave number in air
  let k_air = wave_no_in_air(air_cfg, &frequency);

  // Characteristic absorber impedance and wave number
  let (z_abs, wave_no_abs) = absorber_props(air_cfg, porous_cfg, &frequency);
  let wave_no_abs_y = k_air * sin_phi;
  let wave_no_abs_x = ((wave_no_abs * wave_no_abs) - (wave_no_abs_y * wave_no_abs_y)).sqrt();

  // Angle of propagation within porous layer
  let beta_porous = sin(cmplx_abs(wave_no_abs_y / wave_no_abs)) * ONE_80_OVER_PI;

  // Intermediate term for porous impedance calculation
  let porous_wave_no = wave_no_abs * porous_cfg.thickness;
  let cot_porous_wave_no = porous_wave_no.cos() / porous_wave_no.sin();

  // Impedance at absorber surface
  let z_abs_surface = minus_i * z_abs * (wave_no_abs / wave_no_abs_x) * cot_porous_wave_no;

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Calculate absorption coefficient for porous absorber with no air gap
  let abs_refl = difference_over_sum((z_abs_surface / air_cfg.impedance) * cos_phi, 1.0);
  let abs_alpha = reflectivity_as_alpha(abs_refl);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Impedance values (with air gap)
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

  // X and Y components of the wave number in the air gap
  let wave_no_air_y = wave_no_abs * sin(beta_porous * PI_OVER_180);
  let wave_no_air_x = ((k_air * k_air) - (wave_no_air_y * wave_no_air_y)).sqrt();

  // Impedance at top of air gap (after passing through porous absorber)
  let temp_imp = k_air * cavity_cfg.air_gap;
  let air_gap_z =
    minus_i * air_cfg.impedance * (k_air / wave_no_air_x) * (cos(temp_imp) / sin(temp_imp));

  // Impedance at top of porous absorber after passing through air gap
  let intermediate3 = minus_i * z_abs * cot_porous_wave_no;
  let abs_air_z = ((air_gap_z * intermediate3) + (z_abs * z_abs)) / (air_gap_z + intermediate3);

  // Absorption coefficient for porous absorber with air gap
  let abs_air_refl = difference_over_sum((abs_air_z / air_cfg.impedance) * cos_phi, 1.0);
  let abs_air_alpha = reflectivity_as_alpha(abs_air_refl);

  (abs_alpha, abs_air_alpha)
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
  const FN_NAME: &str = &"do_perforated_panel_calc";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Frequently used intermediate values
  let i: Complex<f64> = Complex::new(0.0, 1.0);
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

  // Wave number in air and angular frequency
  let k_air = wave_no_in_air(air_cfg, &frequency);
  let omega = f_ang(frequency);
  trace(&format!("Wave number       = {}", k_air));
  trace(&format!("Angular frequency = {}", omega));

  // Characteristic absorber impedance and wave number
  let (z_abs, wave_no_abs) = absorber_props(air_cfg, porous_cfg, &frequency);
  trace(&format!("Characteristic impedance = {}", z_abs));
  trace(&format!("Complex wave number      = {}", wave_no_abs));

  // Intermediate terms
  let inter1 = k_air * cavity_cfg.air_gap;
  let cot_inter1 = inter1.cos() / inter1.sin();
  let inter2 = wave_no_abs * porous_cfg.thickness;
  let cot_inter2 = inter2.cos() / inter2.sin();

  trace(&format!("k air * t air      = {}", inter1));
  trace(&format!("cot(k air * t air) = {}", cot_inter1));
  trace(&format!("k cmplx_abs * t cmplx_abs      = {}", inter2));
  trace(&format!("cot(k cmplx_abs * t cmplx_abs) = {}", cot_inter2));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against panel
  let abs_against_panel_z1 = minus_i * air_cfg.impedance * cot_inter1;
  let abs_against_panel_z2 = ((minus_i * abs_against_panel_z1 * z_abs * cot_inter2)
    + (z_abs * z_abs))
    / (abs_against_panel_z1 - (i * z_abs * cot_inter2));
  let surface_resistence = (air_cfg.density / panel_cfg.porosity)
    * sqrt(8.0 * AIR_VISCOSITY * omega)
    * (1.0 + ec_panel_thickness / (2.0 * panel_cfg.hole_radius));
  let abs_against_panel_z3 =
    ((i / panel_cfg.porosity) * ec_panel_thickness * omega * air_cfg.density)
      + abs_against_panel_z2
      + surface_resistence;

  let abs_against_panel_refl = difference_over_sum(abs_against_panel_z3, air_cfg.impedance);
  let abs_against_panel_alpha = reflectivity_as_alpha(abs_against_panel_refl);

  trace(&format!(
    "Absorber against panel z1 = {}",
    abs_against_panel_z1
  ));
  trace(&format!(
    "Absorber against panel z2 = {}",
    abs_against_panel_z2
  ));
  trace(&format!(
    "Surface resistance        = {}",
    surface_resistence
  ));
  trace(&format!(
    "Overall impedence         = {}",
    abs_against_panel_z3
  ));

  trace(&format!(
    "Absorber against panel reflection = {}",
    abs_against_panel_refl
  ));
  trace(&format!(
    "Absorber against panel absorption = {}",
    abs_against_panel_alpha
  ));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against backing
  let abs_against_backing_z1 = minus_i * z_abs * cot_inter2;
  let abs_against_backing_z2 =
    ((minus_i * abs_against_backing_z1 * air_cfg.impedance * cot_inter1)
      + (air_cfg.impedance * air_cfg.impedance))
      / (abs_against_backing_z1 - (i * air_cfg.impedance * cot_inter1));

  let abs_against_backing_z3 = ((air_cfg.density / panel_cfg.porosity)
    * sqrt(8.0 * AIR_VISCOSITY * omega)
    * ((panel_cfg.thickness / 2.0 * panel_cfg.hole_radius) + 1.0))
    + ((ec_panel_thickness / panel_cfg.porosity) * i * omega * air_cfg.density)
    + abs_against_backing_z2;

  let abs_against_backing_refl = difference_over_sum(abs_against_backing_z3, air_cfg.impedance);
  let abs_against_backing_alpha = reflectivity_as_alpha(abs_against_backing_refl);

  trace(&format!(
    "Absorber against backing z1 = {}",
    abs_against_backing_z1
  ));
  trace(&format!(
    "Absorber against backing z2 = {}",
    abs_against_backing_z2
  ));
  trace(&format!(
    "Absorber against backing z3 = {}",
    abs_against_backing_z3
  ));

  trace(&format!(
    "Absorber against backing reflection = {}",
    abs_against_backing_refl
  ));
  trace(&format!(
    "Absorber against backing absorption = {}",
    abs_against_backing_alpha
  ));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber with no air gap
  let inter3 = wave_no_abs * (porous_cfg.thickness + cavity_cfg.air_gap);
  let cot_inter3 = inter3.cos() / inter3.sin();

  let no_air_gap_z1 = minus_i * z_abs * cot_inter3;
  let no_air_gap_z2 =
    (i * omega * air_cfg.density * (ec_panel_thickness / panel_cfg.porosity)) + no_air_gap_z1;

  let no_air_gap_refl = difference_over_sum(no_air_gap_z2, air_cfg.impedance);
  let no_air_gap_alpha = reflectivity_as_alpha(no_air_gap_refl);

  trace(&format!(
    "cot(complex wave no * cavity depth) = {}",
    cot_inter3
  ));
  trace(&format!("No air gap z1 = {}", no_air_gap_z1));
  trace(&format!("No air gap z2 = {}", no_air_gap_z2));

  trace(&format!("No air gap reflection = {}", no_air_gap_refl));
  trace(&format!("No air gap absorption = {}", no_air_gap_alpha));

  trace_boundary(&Some(false));
  (
    no_air_gap_alpha,
    abs_against_panel_alpha,
    abs_against_backing_alpha,
  )
}

// *********************************************************************************************************************
// Reducer function to calculate the absorption of a slotted panel absorber at a specific frequency
// *********************************************************************************************************************
fn do_slotted_panel_calc(
  frequency: f64,
  air_cfg: &AirConfig,
  cavity_cfg: &CavityConfig,
  porous_cfg: &PorousLayerConfig,
  ec_panel_thickness: f64,
  resistance_at_panel: f64,
  resistance_at_backing: f64,
  mass_term_for_air: f64,
) -> (f64, f64, f64) {
  const FN_NAME: &str = &"do_slotted_panel_calc";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Frequently used intermediate values
  let i: Complex<f64> = Complex::new(0.0, 1.0);
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

  // Wave number in air and angular frequency
  let k_air = wave_no_in_air(air_cfg, &frequency);
  let omega = f_ang(frequency);
  trace(&format!("Wave number       = {}", k_air));
  trace(&format!("Angular frequency = {}", omega));

  // Characteristic absorber impedance and wave number
  let (z_abs, wave_no_abs) = absorber_props(air_cfg, porous_cfg, &frequency);
  trace(&format!("Characteristic impedance = {}", z_abs));
  trace(&format!("Complex wave number      = {}", wave_no_abs));

  // Intermediate terms
  let inter1 = k_air * ec_panel_thickness;
  let cot_inter1 = inter1.cos() / inter1.sin();
  trace(&format!("cot(k air * t panel) = {}", cot_inter1));

  let inter2 = k_air * cavity_cfg.air_gap;
  let cot_inter2 = inter2.cos() / inter2.sin();
  trace(&format!("cot(k cmplx_abs * t air) = {}", cot_inter2));

  let inter3 = wave_no_abs * porous_cfg.thickness;
  let cot_inter3 = inter3.cos() / inter3.sin();
  trace(&format!(
    "cot(complex_wave_no * t cmplx_abs) = {}",
    cot_inter3
  ));

  let inter4 = wave_no_abs * (cavity_cfg.air_gap + porous_cfg.thickness);
  let cot_inter4 = inter4.cos() / inter4.sin();
  trace(&format!(
    "cot(complex_wave_no * total depth) = {}",
    cot_inter4
  ));

  let mass_term_for_slotted_panel =
    i * ((omega * mass_term_for_air) - (air_cfg.impedance * cot_inter1));
  trace(&format!(
    "Mass term for air in slotted panel = {}",
    mass_term_for_slotted_panel
  ));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against panel
  let abs_against_panel_z1 = minus_i * air_cfg.impedance * cot_inter2;
  let abs_against_panel_z2 = ((minus_i * abs_against_panel_z1 * z_abs * cot_inter3)
    + (z_abs * z_abs))
    / (abs_against_panel_z1 - (i * z_abs * cot_inter3));
  let abs_against_panel_z3 =
    resistance_at_panel + mass_term_for_slotted_panel + abs_against_panel_z2;

  let abs_against_panel_refl = difference_over_sum(abs_against_panel_z3, air_cfg.impedance);
  let abs_against_panel_alpha = reflectivity_as_alpha(abs_against_panel_refl);

  trace(&format!(
    "Absorber against panel z1 = {}",
    abs_against_panel_z1
  ));
  trace(&format!(
    "Absorber against panel z2 = {}",
    abs_against_panel_z2
  ));
  trace(&format!(
    "Overall impedence         = {}",
    abs_against_panel_z3
  ));

  trace(&format!(
    "Absorber against panel reflection = {}",
    abs_against_panel_refl
  ));
  trace(&format!(
    "Absorber against panel absorption = {}",
    abs_against_panel_alpha
  ));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against backing
  let abs_against_backing_z1 = minus_i * z_abs * cot_inter3;
  let abs_against_backing_z2 =
    ((minus_i * abs_against_backing_z1 * air_cfg.impedance * cot_inter2)
      + (air_cfg.impedance * air_cfg.impedance))
      / (abs_against_backing_z1 - (i * air_cfg.impedance * cot_inter2));
  let abs_against_backing_z3 =
    resistance_at_backing + mass_term_for_slotted_panel + abs_against_backing_z2;

  let abs_against_backing_refl = difference_over_sum(abs_against_backing_z3, air_cfg.impedance);
  let abs_against_backing_alpha = reflectivity_as_alpha(abs_against_backing_refl);

  trace(&format!(
    "Absorber against backing z1 = {}",
    abs_against_backing_z1
  ));
  trace(&format!(
    "Absorber against backing z2 = {}",
    abs_against_backing_z2
  ));
  trace(&format!(
    "Absorber against backing z3 = {}",
    abs_against_backing_z3
  ));

  trace(&format!(
    "Absorber against backing reflection = {}",
    abs_against_backing_refl
  ));
  trace(&format!(
    "Absorber against backing absorption = {}",
    abs_against_backing_alpha
  ));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber with no air gap
  let no_air_gap_z1 = minus_i * z_abs * cot_inter4;
  let no_air_gap_z2 = resistance_at_panel + mass_term_for_slotted_panel + no_air_gap_z1;

  let no_air_gap_refl = difference_over_sum(no_air_gap_z2, air_cfg.impedance);
  let no_air_gap_alpha = reflectivity_as_alpha(no_air_gap_refl);

  trace(&format!("No air gap z1 = {}", no_air_gap_z1));
  trace(&format!("No air gap z2 = {}", no_air_gap_z2));

  trace(&format!("No air gap reflection = {}", no_air_gap_refl));
  trace(&format!("No air gap absorption = {}", no_air_gap_alpha));

  trace_boundary(&Some(false));
  (
    no_air_gap_alpha,
    abs_against_panel_alpha,
    abs_against_backing_alpha,
  )
}

// *********************************************************************************************************************
// Reducer function to calculate the absorption of a microperforated panel absorber at a specific frequency
// *********************************************************************************************************************
fn do_microperforated_panel_calc(
  frequency: f64,
  air_cfg: &AirConfig,
  cavity_cfg: &CavityConfig,
  panel_cfg: &MicroperforatedPanelConfig,
  cos_angle: f64,
) -> f64 {
  const FN_NAME: &str = &"do_microperforated_panel_calc";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Frequently used intermediate values
  let i: Complex<f64> = Complex::new(0.0, 1.0);
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);
  let sqrt_minus_i: Complex<f64> = minus_i.sqrt();

  // Wave number in air and angular frequency
  let k_air = wave_no_in_air(air_cfg, &frequency);
  let omega = f_ang(frequency);
  trace(&format!("Wave number       = {}", k_air));
  trace(&format!("Angular frequency = {}", omega));

  // Intermediate values for equation 6.36
  // k' from eq 6.37
  let k_prime = panel_cfg.hole_radius * sqrt(air_cfg.density_over_viscosity * omega);
  trace(&format!("k_prime = {}", k_prime));

  // i * omega * rho * t
  let inter1 = i * omega * air_cfg.density * panel_cfg.thickness;
  trace(&format!("i * omega * rho * t = {}", inter1));

  // k' * root of -i
  let inter2 = k_prime * sqrt_minus_i;
  trace(&format!("k_prime * sqrt(-i) = {}", inter2));

  // Bessel function values of the first kind, zero and first orders
  let bessel_k1_0 = zbessel(0, inter2);
  let bessel_k1_1 = zbessel(1, inter2);

  trace(&format!("bessel_k1_0 = {}", bessel_k1_0));
  trace(&format!("bessel_k1_1 = {}", bessel_k1_1));

  // Eq 6.36
  let microperf_z1 = inter1 / (1.0 - ((2.0 * bessel_k1_1) / (inter2 * bessel_k1_0)));
  trace(&format!(
    "Impedance at microperforated layer = {}",
    microperf_z1
  ));

  // Intermediate values for equation 6.39
  let kd = k_air * cavity_cfg.air_gap;
  trace(&format!("kd = {}", kd));

  let air_z2 = minus_i * air_cfg.impedance * cos(kd) / sin(kd);
  trace(&format!("Impedance at top of air layer = {}", air_z2));

  let inter3 = sqrt(2.0 * omega * air_cfg.density * AIR_VISCOSITY) / (2.0 * panel_cfg.porosity);
  trace(&format!(
    "sqrt(2 * omega * rho * eta) / 2 * porosity = {}",
    inter3
  ));

  let inter4 = (1.7 * i * omega * air_cfg.density * panel_cfg.hole_radius) / panel_cfg.porosity;
  trace(&format!(
    "(1.7i * omega * rho * radius) / porosity = {}",
    inter4
  ));

  let overall_z = ((microperf_z1 / panel_cfg.porosity) + air_z2 + inter3 + inter4) * cos_angle;
  trace(&format!("Overall impedance = {}", overall_z));

  let refl = difference_over_sum(overall_z, air_cfg.impedance);
  let abs = reflectivity_as_alpha(refl);
  trace(&format!("Reflectivity = {}", refl));
  trace(&format!("Absorption coefficient = {}", abs));

  trace_boundary(&Some(false));
  abs
}

// *********************************************************************************************************************
// The num::complex::Complex module does not contain a function for returning the absolute value of a complex number
// However, this can be calculated by taking the square root of the normal square
// *********************************************************************************************************************
fn cmplx_abs(cplx: Complex<f64>) -> f64 {
  sqrt(cplx.norm_sqr())
}

// *********************************************************************************************************************
// General purpose difference over sum calculation
// *********************************************************************************************************************
fn difference_over_sum(a: Complex<f64>, b: f64) -> Complex<f64> {
  (a - b) / (a + b)
}

// *********************************************************************************************************************
// Calculate characteristic absorber impedance and wave number
// *********************************************************************************************************************
fn absorber_props(
  air_cfg: &AirConfig,
  porous_cfg: &PorousLayerConfig,
  frequency: &f64,
) -> (Complex<f64>, Complex<f64>) {
  let d_and_b_term_x = db_x(&air_cfg.density, frequency, &porous_cfg.sigma);

  // Complex impedance
  let z_abs = air_cfg.impedance
    * Complex::new(
      1.0 + 0.0571 * pow(d_and_b_term_x, -0.754),
      -0.087 * pow(d_and_b_term_x, -0.732),
    );

  // Complex wave number
  let k_abs = wave_no_in_air(air_cfg, frequency)
    * Complex::new(
      1.0 + 0.0978 * pow(d_and_b_term_x, -0.7),
      -0.189 * pow(d_and_b_term_x, -0.595),
    );

  (z_abs, k_abs)
}

// *********************************************************************************************************************
// Calculate wave number in air
// *********************************************************************************************************************
fn wave_no_in_air(air_cfg: &AirConfig, frequency: &f64) -> f64 {
  air_cfg.two_pi_over_c * frequency
}

// *********************************************************************************************************************
// Calculate angular frequency
// *********************************************************************************************************************
fn f_ang(frequency: f64) -> f64 {
  2.0 * PI * frequency
}

// *********************************************************************************************************************
// Calculate Delaney & Bazley's term X
// *********************************************************************************************************************
fn db_x(density: &f64, frequency: &f64, sigma: &u32) -> f64 {
  (density * frequency) / *sigma as f64
}

// *********************************************************************************************************************
// Convert reflectivity to absoprtion and round to two decimal places
// If the value is less than zero, then return 0.0
// *********************************************************************************************************************
fn reflectivity_as_alpha(refl: Complex<f64>) -> f64 {
  let alpha = 1.0 - pow(cmplx_abs(refl), 2.0);

  // Ignore alpha values less than zero, else round to 2dp
  if alpha < 0.0 {
    0.0
  } else {
    (alpha * 100.0).round() / 100.0
  }
}

// *********************************************************************************************************************
// Compute Bessel function of the first kind of integer order>=0 and complex argument z.
//
// Ref: "Handbook of Mathematical Functions with Formulas, Graphs, and Mathematical Tables"
//      M. Abramowitz and I.A. Stegun
//      National Bureau of Standards, U.S. Department of Commerce
//      (1964) - chap. 9, p. 360, Eq. 9.1.10.
//
// http://www.efg2.com/Lab/Mathematics/Complex/Bessel.htm
//
// This routine is verified using tables in the above two references.
//
// This implementation was translated into Rust by Chris Whealy from an original Fortran implementation by
// Gordon C. Everstine, Gaithersburg, MD
// *********************************************************************************************************************
fn zbessel(order: u32, z: Complex<f64>) -> Complex<f64> {
  // Only the non-zero parts of the complex number are output
  let mut result;

  // Exit immediately for special case J0(0)=1
  if order == 0 && z.re == 0.0 && z.im == 0.0 {
    result = Complex::new(0.0, 0.0);
  } else {
    // Divide the input value by 2 and then square it
    let z_over_2 = Complex::new(z.re / 2.0, z.im / 2.0);
    let z_over_2_squared = z_over_2.powf(2.0);

    // Compute zero term of sum (1/n!) without common factor (z/2)^n.
    let mut term0: f64 = 1.0;

    if order > 1 {
      for i in 2..order {
        term0 /= i as f64;
      }
    }

    let mut term = Complex::new(term0, 0.0);
    let mut sum = Complex::new(term0, 0.0);
    let mut sign = true;

    let mut j = Complex::new(0.0, 0.0);
    let mut temp = Complex::new(0.0, 0.0);

    // Sum as many terms of the series as needed to achieve the desired accuracy
    // Maximum number of terms to sum arbitrarily limited to 300
    for idx in 1..300 {
      // Compute new term from preceding one
      j.re = idx as f64;
      temp.re = order as f64 + j.re;
      term = (term / j) * (z_over_2_squared / temp);

      // Series has alternating signs
      sign = !sign;

      if sign {
        sum.re += term.re;
        sum.im += term.im;
      } else {
        sum.re -= term.re;
        sum.im -= term.im;
      }

      temp.re = term0;

      // If the current value is within calculation precision value, then quit the loop
      if cmplx_abs(term / temp) < BESSEL_TOLERANCE {
        break;
      }
    }

    // Multiply by common factor (z/2)^n
    result = sum * z_over_2.powf(order as f64);

    // Any values less than the display precision are ignored
    if fabs(result.re) < BESSEL_PRECISION {
      result.re = 0.0;
    }

    if fabs(result.im) < BESSEL_PRECISION {
      result.im = 0.0;
    }
  }

  result
}
