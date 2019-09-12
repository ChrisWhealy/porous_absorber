// *********************************************************************************************************************
// Porous Absorber Calculation Engine
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Usage
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use libm::{sin, cos, sqrt, pow};
use num::complex::Complex;
use std::f64::consts::PI;

use crate::struct_lib::{PorousAbsInfo, PlotPoint};
use crate::air::AirConfig;
use crate::porous_absorber::PorousAbsorberConfig;
use crate::cavity::CavityConfig;
use crate::display::DisplayConfig;
use crate::sound::SoundConfig;


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Constants
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const PI_OVER_180    : f64 = PI / 180.0;
const ONE_80_OVER_PI : f64 = 180.0 / PI;



// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
pub fn calculate_porous_absorption(
  air    : &AirConfig
, cavity : &CavityConfig
, display: &DisplayConfig
, sound  : &SoundConfig
, porous : &PorousAbsorberConfig
) -> PorousAbsInfo {
  display
    .frequencies
    .iter()
    .fold(
      PorousAbsInfo { air_gap: vec!(), no_air_gap : vec!() }
    , | mut acc, frequency | {
        let (abs_no_air_gap, abs_air_gap) = do_porous_abs_calc(*frequency, &air, &cavity, &sound, &porous);

        // Build the vectors of plot points for each absorber type
        acc.no_air_gap.push(PlotPoint { x: *frequency, y: abs_no_air_gap});
        acc.air_gap.push(PlotPoint { x: *frequency, y: abs_air_gap});

        return acc;
      }
    )
}



// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reducer function to calculate the absorption of a porous absorber with or without and air gap at a specific frequency
fn do_porous_abs_calc(
  frequency  : f64
, air_cfg    : &AirConfig
, cavity_cfg : &CavityConfig
, sound_cfg  : &SoundConfig
, porous_cfg : &PorousAbsorberConfig
) -> (f64, f64) {
  // Frequently used intermediate values
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);
  let sin_phi: f64 = sin(sound_cfg.angle as f64 * PI_OVER_180);
  let cos_phi: f64 = cos(sound_cfg.angle as f64 * PI_OVER_180);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Calculation sequence for absorption coefficient of a porous absorber both with and without an air gap
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  
  // Wave number in air
  let k_air = air_cfg.two_pi_over_c * frequency;

  // Delaney and Bazley's term X
  let d_and_b_term_x = (air_cfg.density * frequency) / porous_cfg.sigma as f64;

  // Characteristic absorber impedance
  let z_abs = air_cfg.impedance * Complex::new(1.0 + 0.0571 * pow(d_and_b_term_x, -0.754), -0.087 * pow(d_and_b_term_x, -0.732));

  // Complex wave number within the porous absorber layer with its Y and X component values
  let wave_no_abs = air_cfg.two_pi_over_c
     * frequency
     * Complex::new(1.0 + 0.0978 * pow(d_and_b_term_x, -0.7), -0.189 * pow(d_and_b_term_x, -0.595));
        
  let wave_no_abs_y_comp = k_air * sin_phi;
  let wave_no_abs_x_comp = ((wave_no_abs * wave_no_abs) - (wave_no_abs_y_comp * wave_no_abs_y_comp)).sqrt();

  // Angle of propagation within porous layer
  let beta_porous = sin(abs(wave_no_abs_y_comp / wave_no_abs)) * ONE_80_OVER_PI;

  // Intermediate term for porous impedance calculation
  let porous_wave_no     = wave_no_abs * porous_cfg.thickness;
  let cot_porous_wave_no = porous_wave_no.cos() / porous_wave_no.sin();

  // Impedance at absorber surface
  let z_abs_surface = minus_i * z_abs * (wave_no_abs / wave_no_abs_x_comp) * cot_porous_wave_no;

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Calculate absorption coefficient for porous absorber with no air gap
  let abs_alpha = reflectivity_as_alpha((z_abs_surface / air_cfg.impedance) * cos_phi);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Impedance values (with air gap)
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

  // X and Y components of the wave number in the air gap
  let wave_no_air_y_comp = wave_no_abs * sin(beta_porous * PI_OVER_180);
  let wave_no_air_x_comp = ((k_air * k_air) - (wave_no_air_y_comp * wave_no_air_y_comp)).sqrt();

  // Impedance at top of air gap (after passing through porous absorber)
  let temp_imp = k_air * cavity_cfg.air_gap;
  let air_gap_z = minus_i * air_cfg.impedance * (k_air / wave_no_air_x_comp) * (cos(temp_imp) / sin(temp_imp));

  // Impedance at top of porous absorber after passing through air gap
  let intermediate3 = minus_i * z_abs * cot_porous_wave_no;
  let abs_air_z     = ((air_gap_z * intermediate3) + (z_abs * z_abs)) / (air_gap_z + intermediate3);

  // Overall absorption coefficient
  let abs_air_alpha = reflectivity_as_alpha((abs_air_z / air_cfg.impedance) * cos_phi);

  return (abs_alpha, abs_air_alpha);
}



// *********************************************************************************************************************
// The num::complex::Complex module does not contain a function for returning the absolute value of a complex number
// However, this can be calculated by taking the square root of the normal square
fn abs(cplx: Complex<f64>) -> f64 {
  sqrt(cplx.norm_sqr())
}



// *********************************************************************************************************************
// Convert reflectivity to absoprtion and round to two decimal places
// If the value is less than zero, then return 0.0
fn reflectivity_as_alpha(refl: Complex<f64>) -> f64 {
   let alpha = 1.0 - pow(abs((refl - 1.0) / (refl + 1.0)), 2.0);

  // Ignore alpha values less than zero, else round to 2dp
  if alpha < 0.0 { 0.0 }
  else           { (alpha * 100.0).round() / 100.0 }
}


