extern crate num_format;

use std::error::Error;
use std::fmt;

/***********************************************************************************************************************
 * Air pressure and temperature range check values
 */
const START_THICKNESS   : u32 = 5;
const DEFAULT_THICKNESS : u32 = 30;
const END_THICKNESS     : u32 = 500;

const START_FLOW_RESISTIVITY   : u32 = 1000;
const DEFAULT_FLOW_RESISTIVITY : u32 = 16500;
const END_FLOW_RESISTIVITY     : u32 = 100000;

const UNITS_THICKNESS        : &str = "mm";
const UNITS_FLOW_RESISTIVITY : &str = "rayls/m";

/***********************************************************************************************************************
 * Possible errors when creating porous absorber struct
 */
#[derive(Debug)]
pub struct PorousError {
  msg : String
}

impl PorousError {
  fn new(property: &str, units: &str, min: u32, max:u32, err_val: u32) -> PorousError {
    PorousError {
      msg : format!("{} must be a value in {} between {:?} and {:?}, not '{:?}'", property, units, min, max, err_val)
    }
  }
}

impl fmt::Display for PorousError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for PorousError {
  fn description(&self) -> &str {
    &self.msg
  }
}

/***********************************************************************************************************************
 * Porous absorber configuration
 */
pub struct PorousAbsorberConfig {
  pub thickness_mm : u32
, pub thickness    : f64
, pub sigma        : u32
}

impl PorousAbsorberConfig {
  pub fn default() -> PorousAbsorberConfig {
    PorousAbsorberConfig::new(DEFAULT_THICKNESS, DEFAULT_FLOW_RESISTIVITY).unwrap()
  }

  pub fn new(thickness_arg: u32, sigma_arg: u32) -> Result<PorousAbsorberConfig, PorousError> {
    if thickness_arg < START_THICKNESS ||
       thickness_arg > END_THICKNESS {
      return Err(
        PorousError::new("Thickness", UNITS_THICKNESS, START_THICKNESS, END_THICKNESS, thickness_arg)
      );
    }

    if sigma_arg < START_FLOW_RESISTIVITY ||
       sigma_arg > END_FLOW_RESISTIVITY {
      return Err(
        PorousError::new("Flow resistivity", UNITS_FLOW_RESISTIVITY, START_FLOW_RESISTIVITY, END_FLOW_RESISTIVITY, thickness_arg)
      );
    }

    return
      Ok(PorousAbsorberConfig {
        thickness_mm : thickness_arg
      , thickness    : thickness_arg as f64 / 1000.0
      , sigma        : sigma_arg
      })
  }
}

