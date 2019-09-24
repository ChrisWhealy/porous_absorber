use std::error::Error;
use std::fmt;

/***********************************************************************************************************************
 * Air pressure and temperature range check values
 */
const START_THICKNESS   : u32 = 0;
const DEFAULT_THICKNESS : u32 = 100;
const END_THICKNESS     : u32 = 500;

const UNITS_THICKNESS : &str = "mm";

/***********************************************************************************************************************
 * Possible errors when creating cavity struct
 */
#[derive(Debug)]
pub struct CavityError {
  msg : String
}

impl CavityError {
  fn new(property: &str, units: &str, min: u32, max:u32, err_val: u32) -> CavityError {
    CavityError {
      msg : format!("{} must be a value in {} between {:?} and {:?}, not '{:?}'", property, units, min, max, err_val)
    }
  }
}

impl fmt::Display for CavityError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for CavityError {
  fn description(&self) -> &str {
    &self.msg
  }
}

/***********************************************************************************************************************
 * Cavity configuration
 */
pub struct CavityConfig {
  pub air_gap_mm : u32
, pub air_gap    : f64
}

impl CavityConfig {
  pub fn default() -> CavityConfig {
    CavityConfig::new(DEFAULT_THICKNESS).unwrap()
  }

  pub fn new(air_gap_arg: u32) -> Result<CavityConfig, CavityError> {
    if air_gap_arg > END_THICKNESS {
      return Err(CavityError::new("Air gap", UNITS_THICKNESS, START_THICKNESS, END_THICKNESS, air_gap_arg));
    }

    return Ok(CavityConfig {
      air_gap_mm : air_gap_arg
    , air_gap    : air_gap_arg as f64 / 1000.0
    })
  }
}

