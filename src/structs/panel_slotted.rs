// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Slotted panel properties
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate num_format;

use std::fmt;

/***********************************************************************************************************************
 * Range check values
 */
const START_THICKNESS   : f64 = 1.0;
const DEFAULT_THICKNESS : f64 = 10.0;
const END_THICKNESS     : f64 = 50.0;

const START_DISTANCE    : f64 = 2.0;
const DEFAULT_DISTANCE  : f64 = 25.4;
const END_DISTANCE      : f64 = 300.0;

const START_WIDTH       : f64 = 1.0;
const DEFAULT_WIDTH     : f64 = 5.0;
const END_WIDTH         : f64 = 50.0;

const DEFAULT_POROSITY  : f64 = DEFAULT_WIDTH / (DEFAULT_DISTANCE + DEFAULT_WIDTH);

const UNITS_THICKNESS   : &str = "mm";
const UNITS_DISTANCE    : &str = "mm";
const UNITS_WIDTH       : &str = "mm";

/***********************************************************************************************************************
 * Possible errors when creating porous absorber struct
 */
#[derive(Debug)]
pub struct SlottedPanelError {
  pub msg : String
}

impl SlottedPanelError {
  pub fn new(property: &str, units: &str, min: f64, max: f64, err_val: f64) -> SlottedPanelError {
    SlottedPanelError {
      msg : format!("{} must be a value in {} between {:?} and {:?}, not '{:?}'", property, units, min, max, err_val)
    }
  }
}

impl fmt::Display for SlottedPanelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

/***********************************************************************************************************************
 * Slotted panel configuration
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct SlottedPanelConfig {
  pub thickness_mm     : f64
, pub thickness        : f64
, pub slot_distance    : f64
, pub slot_distance_mm : f64
, pub slot_width       : f64
, pub slot_width_mm    : f64
, pub porosity         : f64
}

impl SlottedPanelConfig {
  pub fn default() -> SlottedPanelConfig {
    SlottedPanelConfig::new(DEFAULT_THICKNESS, DEFAULT_DISTANCE, DEFAULT_WIDTH, DEFAULT_POROSITY).unwrap()
  }

  pub fn new(
    thickness_arg : f64
  , distance_arg  : f64
  , width_arg     : f64
  , porosity_arg  : f64
  ) -> Result<SlottedPanelConfig, SlottedPanelError> {
    if thickness_arg < START_THICKNESS ||
       thickness_arg > END_THICKNESS {
      return Err(
        SlottedPanelError::new("Thickness", UNITS_THICKNESS, START_THICKNESS, END_THICKNESS, thickness_arg)
      );
    }

    if distance_arg < START_DISTANCE ||
       distance_arg > END_DISTANCE {
      return Err(
        SlottedPanelError::new("Distance", UNITS_DISTANCE, START_DISTANCE, END_DISTANCE, distance_arg)
      );
    }

    if width_arg < START_WIDTH ||
       width_arg > END_WIDTH {
      return Err(
        SlottedPanelError::new("Width", UNITS_WIDTH, START_WIDTH, END_WIDTH, width_arg)
      );
    }

    return
      Ok(SlottedPanelConfig {
        thickness_mm     : thickness_arg
      , thickness        : thickness_arg / 1000.0
      , slot_distance_mm : distance_arg
      , slot_distance    : distance_arg / 1000.0
      , slot_width_mm    : width_arg
      , slot_width       : width_arg / 1000.0
      , porosity         : porosity_arg
      })
  }
}

