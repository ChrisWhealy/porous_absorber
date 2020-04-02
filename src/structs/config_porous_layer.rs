// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Rigid Backed Porous Absorber
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate num_format;

use std::fmt;


/***********************************************************************************************************************
 * Range check values
 */
const START_THICKNESS   : u16 = 5;
const DEFAULT_THICKNESS : u16 = 30;
const END_THICKNESS     : u16 = 500;

const START_FLOW_RESISTIVITY   : u32 = 1000;
const DEFAULT_FLOW_RESISTIVITY : u32 = 16500;
const END_FLOW_RESISTIVITY     : u32 = 100000;

const UNITS_THICKNESS        : &str = "mm";
const UNITS_FLOW_RESISTIVITY : &str = "rayls/m";


/***********************************************************************************************************************
 * Possible errors when creating porous absorber struct
 */
#[derive(Debug)]
pub struct PorousLayerError {
  pub msg : String
}

impl PorousLayerError {
  pub fn new(property: &str, units: &str, min: u32, max:u32, err_val: u32) -> PorousLayerError {
    PorousLayerError {
      msg : format!("{} must be a value in {} between {:?} and {:?}, not '{:?}'", property, units, min, max, err_val)
    }
  }
}

impl fmt::Display for PorousLayerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

/***********************************************************************************************************************
 * Porous absorber configuration
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct PorousLayerConfig {
  pub thickness_mm : u16
, pub thickness    : f64
, pub sigma        : u32
}

impl PorousLayerConfig {
  pub fn default() -> PorousLayerConfig {
    PorousLayerConfig::new(DEFAULT_THICKNESS, DEFAULT_FLOW_RESISTIVITY).unwrap()
  }

  pub fn new(thickness_arg: u16, sigma_arg: u32) -> Result<PorousLayerConfig, PorousLayerError> {
    if thickness_arg < START_THICKNESS ||
       thickness_arg > END_THICKNESS {
      return Err(
        PorousLayerError::new("Thickness", UNITS_THICKNESS, START_THICKNESS as u32, END_THICKNESS as u32, thickness_arg as u32)
      );
    }

    if sigma_arg < START_FLOW_RESISTIVITY ||
       sigma_arg > END_FLOW_RESISTIVITY {
      return Err(
        PorousLayerError::new("Flow resistivity", UNITS_FLOW_RESISTIVITY, START_FLOW_RESISTIVITY, END_FLOW_RESISTIVITY, sigma_arg)
      );
    }

    return
      Ok(PorousLayerConfig {
        thickness_mm : thickness_arg
      , thickness    : thickness_arg as f64 / 1000.0
      , sigma        : sigma_arg
      })
  }
}

