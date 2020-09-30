/***********************************************************************************************************************
 * Porous Absorber Calculation Engine - Microperforated Panel
 *
 * (c) Chris Whealy 2020
 */
extern crate wasm_bindgen;

use libm::{cos, sin, sqrt};
use num::complex::Complex;
use std::f64::consts::PI;

use crate::config::{
  air::{AirConfig, AIR_VISCOSITY},
  cavity::CavityConfig,
  config_set::ConfigSet,
  display::{PlotAbsPoint, SeriesData},
  generic_device::{DeviceType, GenericDeviceInfo},
  panel_microperforated::MicroperforatedPanelConfig,
};

use crate::chart::{constants, render};
use crate::utils::maths_functions::*;

/***********************************************************************************************************************
 * Trace functionality
 */
use crate::trace::Trace;

const LIB_NAME: &str = "calc_engine::microperforated_panel";
const TRACE_ACTIVE: bool = false;

/***********************************************************************************************************************
 * Microperforated Panel Calculation
 */
pub fn calculate<'a>(config_set: &'a ConfigSet) -> GenericDeviceInfo<'a> {
  const FN_NAME: &str = "calculate";
  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());
  trace_boundary(Some(true));

  let air = config_set.air_config.as_ref().unwrap();
  let cavity = config_set.cavity_config.as_ref().unwrap();
  let display = config_set.display_config.as_ref().unwrap();
  let sound = config_set.sound_config.as_ref().unwrap();
  let panel = config_set
    .panel_config
    .as_ref()
    .unwrap()
    .panel_microperforated
    .as_ref()
    .unwrap();

  let cos_angle = cos(sound.angle as f64 * PI / 180.0);

  let abs_info = display.frequencies.iter().fold(
    GenericDeviceInfo {
      device_type: DeviceType::MicroperforatedPanelAbsorber,
      abs_series: vec![SeriesData {
        name: constants::TXT_MP_PANEL,
        plot_points: vec![],
      }],
      sl_panel: None,
      pf_panel: None,
      mp_panel: Some(&panel),
      porous_layer: None,
      cavity: Some(&cavity),
    },
    |mut acc, frequency| {
      let abs_data = do_microperforated_panel_calc(*frequency, &air, &cavity, &panel, cos_angle);
      acc.abs_series[0].plot_points.push(PlotAbsPoint {
        at: render::constants::ORIGIN,
        freq: *frequency,
        abs: abs_data,
      });

      acc
    },
  );

  trace_boundary(Some(false));
  abs_info
}

/***********************************************************************************************************************
 * Reducer function to calculate the absorption of a microperforated panel absorber at a specific frequency
 */
fn do_microperforated_panel_calc(
  frequency: f64,
  air_cfg: &AirConfig,
  cavity_cfg: &CavityConfig,
  panel_cfg: &MicroperforatedPanelConfig,
  cos_angle: f64,
) -> f64 {
  const FN_NAME: &str = "do_microperforated_panel_calc";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());
  let trace = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(Some(true));

  // Frequently used intermediate values
  let i: Complex<f64> = Complex::new(0.0, 1.0);
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);
  let sqrt_minus_i: Complex<f64> = minus_i.sqrt();

  // Wave number in air and angular frequency
  let k_air = wave_no_in_air(air_cfg, &frequency);
  let omega = f_ang(frequency);
  trace(format!("Wave number       = {}", k_air));
  trace(format!("Angular frequency = {}", omega));

  // Intermediate values for equation 6.36
  // k' from eq 6.37
  let k_prime = panel_cfg.hole_radius * sqrt(air_cfg.density_over_viscosity * omega);
  trace(format!("k_prime = {}", k_prime));

  // i * omega * rho * t
  let inter1 = i * omega * air_cfg.density * panel_cfg.thickness;
  trace(format!("i * omega * rho * t = {}", inter1));

  // k' * root of -i
  let inter2 = k_prime * sqrt_minus_i;
  trace(format!("k_prime * sqrt(-i) = {}", inter2));

  // Bessel function values of the first kind, zero and first orders
  let bessel_k1_0 = zbessel(0, inter2);
  let bessel_k1_1 = zbessel(1, inter2);

  trace(format!("bessel_k1_0 = {}", bessel_k1_0));
  trace(format!("bessel_k1_1 = {}", bessel_k1_1));

  // Eq 6.36
  let microperf_z1 = inter1 / (1.0 - ((2.0 * bessel_k1_1) / (inter2 * bessel_k1_0)));
  trace(format!("Impedance at microperforated layer = {}", microperf_z1));

  // Intermediate values for equation 6.39
  let kd = k_air * cavity_cfg.air_gap;
  trace(format!("kd = {}", kd));

  let air_z2 = minus_i * air_cfg.impedance * cos(kd) / sin(kd);
  trace(format!("Impedance at top of air layer = {}", air_z2));

  let inter3 = sqrt(2.0 * omega * air_cfg.density * AIR_VISCOSITY) / (2.0 * panel_cfg.porosity);
  trace(format!("sqrt(2 * omega * rho * eta) / 2 * porosity = {}", inter3));

  let inter4 = (1.7 * i * omega * air_cfg.density * panel_cfg.hole_radius) / panel_cfg.porosity;
  trace(format!("(1.7i * omega * rho * radius) / porosity = {}", inter4));

  let overall_z = ((microperf_z1 / panel_cfg.porosity) + air_z2 + inter3 + inter4) * cos_angle;
  trace(format!("Overall impedance = {}", overall_z));

  let refl = difference_over_sum(overall_z, air_cfg.impedance);
  let abs = reflectivity_as_alpha(refl);
  trace(format!("Reflectivity = {}", refl));
  trace(format!("Absorption coefficient = {}", abs));

  trace_boundary(Some(false));
  abs
}
