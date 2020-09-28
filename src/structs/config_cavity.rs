// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Cavity properties
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************

use std::fmt;

/***********************************************************************************************************************
 * Range check values
 */
const START_THICKNESS: u16 = 0;
const DEFAULT_THICKNESS: u16 = 100;
const END_THICKNESS: u16 = 500;

const UNITS_THICKNESS: &str = "mm";

/***********************************************************************************************************************
 * Possible errors when creating cavity struct
 */
#[derive(Debug)]
pub struct CavityError {
  msg: String,
}

impl CavityError {
  fn new(property: &str, units: &str, min: u16, max: u16, err_val: u16) -> CavityError {
    CavityError {
      msg: format!(
        "{} must be a value in {} between {:?} and {:?}, not '{:?}'",
        property, units, min, max, err_val
      ),
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
    CavityConfig::new(DEFAULT_THICKNESS).unwrap()
  }

  pub fn new(air_gap_arg: u16) -> Result<CavityConfig, CavityError> {
    if air_gap_arg > END_THICKNESS {
      Err(CavityError::new(
        "Air gap",
        UNITS_THICKNESS,
        START_THICKNESS,
        END_THICKNESS,
        air_gap_arg,
      ))
    } else {
      Ok(CavityConfig {
        air_gap_mm: air_gap_arg,
        air_gap: air_gap_arg as f64 / 1000.0,
      })
    }
  }
}
