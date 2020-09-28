// *********************************************************************************************************************
// Mathematical function used by the Porous Absorber Calculation Engine
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

use libm::{fabs, pow, sqrt};
use num::complex::Complex;
use std::f64::consts::PI;

use crate::structs::{config_air::AirConfig, config_porous_layer::PorousLayerConfig};

// *********************************************************************************************************************
// The num::complex::Complex module does not contain a function for returning the absolute value of a complex number
// However, this can be calculated by taking the square root of the normal square
// *********************************************************************************************************************
pub fn cmplx_abs(cplx: Complex<f64>) -> f64 {
  sqrt(cplx.norm_sqr())
}

// *********************************************************************************************************************
// General purpose difference over sum calculation
// *********************************************************************************************************************
pub fn difference_over_sum(a: Complex<f64>, b: f64) -> Complex<f64> {
  (a - b) / (a + b)
}

// *********************************************************************************************************************
// Calculate characteristic absorber impedance and wave number
// *********************************************************************************************************************
pub fn absorber_props(
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
pub fn wave_no_in_air(air_cfg: &AirConfig, frequency: &f64) -> f64 {
  air_cfg.two_pi_over_c * frequency
}

// *********************************************************************************************************************
// Calculate angular frequency
// *********************************************************************************************************************
pub fn f_ang(frequency: f64) -> f64 {
  2.0 * PI * frequency
}

// *********************************************************************************************************************
// Calculate Delaney & Bazley's term X
// *********************************************************************************************************************
pub fn db_x(density: &f64, frequency: &f64, sigma: &u32) -> f64 {
  (density * frequency) / *sigma as f64
}

// *********************************************************************************************************************
// Convert reflectivity to absoprtion and round to two decimal places
// If the value is less than zero, then return 0.0
// *********************************************************************************************************************
pub fn reflectivity_as_alpha(refl: Complex<f64>) -> f64 {
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
pub fn zbessel(order: u32, z: Complex<f64>) -> Complex<f64> {
  const BESSEL_TOLERANCE: f64 = 0.000000001;
  const BESSEL_PRECISION: f64 = 0.000000000001;

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
