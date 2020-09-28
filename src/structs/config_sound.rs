// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Sound properties
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************

use std::fmt;

/***********************************************************************************************************************
 * Range check values
 */
const START_ANGLE: u16 = 0;
const DEFAULT_ANGLE: u16 = START_ANGLE;
const END_ANGLE: u16 = 89;

const UNITS_ANGLE: &str = "degrees";

/***********************************************************************************************************************
 * Possible errors when creating sound struct
 */
#[derive(Debug)]
pub struct SoundError {
  msg: String,
}

impl SoundError {
  fn new(property: &str, units: &str, min: u16, max: u16, err_val: u16) -> SoundError {
    SoundError {
      msg: format!(
        "{} must be a value in {} between {:?} and {:?}, not '{:?}'",
        property, units, min, max, err_val
      ),
    }
  }
}

impl fmt::Display for SoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

/***********************************************************************************************************************
 * Sound configuration
 */
pub struct SoundConfig {
  pub angle: u16,
}

impl SoundConfig {
  pub fn default() -> SoundConfig {
    SoundConfig::new(DEFAULT_ANGLE).unwrap()
  }

  pub fn new(angle_arg: u16) -> Result<SoundConfig, SoundError> {
    if angle_arg > 90 {
      Err(SoundError::new(
        "Incident angle",
        UNITS_ANGLE,
        START_ANGLE,
        END_ANGLE,
        angle_arg,
      ))
    } else {
      Ok(SoundConfig { angle: angle_arg })
    }
  }
}
