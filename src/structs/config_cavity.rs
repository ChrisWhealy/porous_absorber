/***********************************************************************************************************************
 * Porous Absorber Calculator - Cavity properties
 *
 * (c) Chris Whealy 2020
 */
use std::fmt;

use crate::structs::{constants, ranges::Range};
use crate::utils::validation;

/***********************************************************************************************************************
 * Range check values
 */
const THICKNESS_RANGE: Range<u16> = Range {
  name: constants::TXT_AIR_GAP,
  units: constants::UNITS_THICKNESS,
  min: 0,
  default: 100,
  max: 500,
};

/***********************************************************************************************************************
 * Possible errors when creating cavity struct
 */
#[derive(Debug)]
pub struct CavityError {
  msg: String,
}

impl CavityError {
  fn new(range: Range<u16>, err_val: u16) -> CavityError {
    CavityError {
      msg: validation::failure_msg(range, err_val),
    }
  }
}

impl fmt::Display for CavityError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

/***********************************************************************************************************************
 * Cavity configuration
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CavityConfig {
  pub air_gap_mm: u16,
  pub air_gap: f64,
}

impl CavityConfig {
  pub fn default() -> CavityConfig {
    CavityConfig::new(THICKNESS_RANGE.default).unwrap()
  }

  pub fn new(air_gap_arg: u16) -> Result<CavityConfig, CavityError> {
    if !THICKNESS_RANGE.contains(air_gap_arg) {
      Err(CavityError::new(THICKNESS_RANGE, air_gap_arg))
    } else {
      Ok(CavityConfig {
        air_gap_mm: air_gap_arg,
        air_gap: air_gap_arg as f64 / 1000.0,
      })
    }
  }
}
