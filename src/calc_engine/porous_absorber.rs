/***********************************************************************************************************************
 * Porous Absorber Calculation Engine - Porous Absorber
 *
 * (c) Chris Whealy 2020
 */
extern crate wasm_bindgen;

use libm::{cos, sin};
use num::complex::Complex;
use std::f64::consts::PI;

use crate::config::{
  air::AirConfig,
  cavity::CavityConfig,
  display::{DisplayConfig, PlotAbsPoint, SeriesData},
  generic_device::{DeviceType, GenericDeviceInfo},
  porous_layer::PorousLayerConfig,
  sound::SoundConfig,
};

use crate::chart::constants;
use crate::utils::maths_functions::*;

/***********************************************************************************************************************
 * Trace functionality
 */
use crate::trace::Trace;

const LIB_NAME: &str = "calc_engine::porous_absorber";
const TRACE_ACTIVE: bool = false;

/***********************************************************************************************************************
 * Rigid Backed Porous Absorber Calculation
 */
const PI_OVER_180: f64 = PI / 180.0;
const ONE_80_OVER_PI: f64 = 180.0 / PI;

pub fn calculate<'a>(
  air: &'a AirConfig,
  cavity: &'a CavityConfig,
  display: &'a DisplayConfig,
  sound: &'a SoundConfig,
  porous: &'a PorousLayerConfig,
) -> GenericDeviceInfo<'a> {
  const FN_NAME: &str = "calculate";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(Some(true));

  let abs_info = display.frequencies.iter().fold(
    GenericDeviceInfo {
      device_type: DeviceType::RigidBackedPorousAbsorber,
      abs_series: vec![
        SeriesData {
          name: constants::TXT_AIR_GAP,
          plot_points: vec![],
        },
        SeriesData {
          name: constants::TXT_NO_AIR_GAP,
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
      let (abs_no_air_gap, abs_air_gap) = do_porous_abs_calc(*frequency, &air, &cavity, &sound, &porous);

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

  trace_boundary(Some(false));
  abs_info
}

/***********************************************************************************************************************
 * Reducer function to calculate the absorption of a porous absorber at a specific frequency
 */
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
  // X and Y components of the wave number in the air gap
  let wave_no_air_y = wave_no_abs * sin(beta_porous * PI_OVER_180);
  let wave_no_air_x = ((k_air * k_air) - (wave_no_air_y * wave_no_air_y)).sqrt();

  // Impedance at top of air gap (after passing through porous absorber)
  let temp_imp = k_air * cavity_cfg.air_gap;
  let air_gap_z = minus_i * air_cfg.impedance * (k_air / wave_no_air_x) * (cos(temp_imp) / sin(temp_imp));

  // Impedance at top of porous absorber after passing through air gap
  let intermediate3 = minus_i * z_abs * cot_porous_wave_no;
  let abs_air_z = ((air_gap_z * intermediate3) + (z_abs * z_abs)) / (air_gap_z + intermediate3);

  // Absorption coefficient for porous absorber with air gap
  let abs_air_refl = difference_over_sum((abs_air_z / air_cfg.impedance) * cos_phi, 1.0);
  let abs_air_alpha = reflectivity_as_alpha(abs_air_refl);

  (abs_alpha, abs_air_alpha)
}
