// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Microperforated panel properties
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate num_format;

use std::f64::consts::PI;
use std::fmt;

/***********************************************************************************************************************
 * Range check values
 */
const START_THICKNESS   : f64 = 0.5;
const DEFAULT_THICKNESS : f64 = 1.0;
const END_THICKNESS     : f64 = 10.0;

const START_CENTRES   : f64 = 0.5;
const DEFAULT_CENTRES : f64 = 5.0;
const END_CENTRES     : f64 = 10.0;

const START_RADIUS   : f64 = 0.05;
const DEFAULT_RADIUS : f64 = 0.25;
const END_RADIUS     : f64 = 0.5;

const DEFAULT_POROSITY : f64 = (PI * DEFAULT_RADIUS * DEFAULT_RADIUS) / (DEFAULT_CENTRES * DEFAULT_CENTRES);

const UNITS_THICKNESS : &str = "mm";
const UNITS_CENTRES   : &str = "mm";
const UNITS_RADIUS    : &str = "mm";

/***********************************************************************************************************************
 * Possible errors when creating porous absorber struct
 */
#[derive(Debug)]
pub struct MicroperforatedPanelError {
  pub msg : String
}

impl MicroperforatedPanelError {
  pub fn new(property: &str, units: &str, min: f64, max: f64, err_val: f64) -> MicroperforatedPanelError {
    MicroperforatedPanelError {
      msg : format!("{} must be a value in {} between {:?} and {:?}, not '{:?}'", property, units, min, max, err_val)
    }
  }
}

impl fmt::Display for MicroperforatedPanelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}


/***********************************************************************************************************************
 * Perforated panel configuration
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct MicroperforatedPanelConfig {
  pub thickness_mm    : f64
, pub thickness       : f64
, pub hole_centres    : f64
, pub hole_centres_mm : f64
, pub hole_radius     : f64
, pub hole_radius_mm  : f64
, pub porosity        : f64
}

impl MicroperforatedPanelConfig {
  pub fn default() -> MicroperforatedPanelConfig {
    MicroperforatedPanelConfig::new(DEFAULT_THICKNESS, DEFAULT_CENTRES, DEFAULT_RADIUS, DEFAULT_POROSITY).unwrap()
  }

  pub fn new(
    thickness_arg : f64
  , centres_arg   : f64
  , radius_arg    : f64
  , porosity_arg  : f64
  ) -> Result<MicroperforatedPanelConfig, MicroperforatedPanelError> {
    if thickness_arg < START_THICKNESS ||
       thickness_arg > END_THICKNESS {
      return Err(
        MicroperforatedPanelError::new("Thickness", UNITS_THICKNESS, START_THICKNESS, END_THICKNESS, thickness_arg)
      );
    }

    if centres_arg < START_CENTRES ||
       centres_arg > END_CENTRES {
      return Err(
        MicroperforatedPanelError::new("Centres", UNITS_CENTRES, START_CENTRES, END_CENTRES, centres_arg)
      );
    }

    if radius_arg < START_RADIUS ||
       radius_arg > END_RADIUS {
      return Err(
        MicroperforatedPanelError::new("Radius", UNITS_RADIUS, START_RADIUS, END_RADIUS, radius_arg)
      );
    }

    return
      Ok(MicroperforatedPanelConfig {
        thickness_mm    : thickness_arg
      , thickness       : thickness_arg / 1000.0
      , hole_centres_mm : centres_arg
      , hole_centres    : centres_arg / 1000.0
      , hole_radius_mm  : radius_arg
      , hole_radius     : radius_arg / 1000.0
      , porosity        : porosity_arg
      })
  }
}

