/***********************************************************************************************************************
 * Generic validation failure message functions
 *
 * (c) Chris Whealy 2020
 */
use crate::structs::constants;
use crate::structs::ranges::Range;

pub fn failure_msg<T: std::fmt::Debug>(range: Range<T>, err_val: T) -> String {
  format!(
    "{} must be a value in {} between {:?} and {:?}, not '{:?}'",
    range.name, range.units, range.min, range.max, err_val
  )
}

pub fn start_freq_err(range: Range<f64>, err_val: f64) -> String {
  format!(
    "Graph start frequency must be a value in {} between {:?} and {:?}, not '{:?}'",
    constants::UNITS_FREQ,
    range.min,
    range.max,
    err_val
  )
}

pub fn oct_subdiv_err(err_val: f64) -> String {
  format!(
    "Octave subdivisions argument must be either 1, 2, 3 or 6, not '{}'",
    err_val.round()
  )
}
