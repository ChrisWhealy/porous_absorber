/***********************************************************************************************************************
 * Porous Absorber Calculator - Sound properties
 *
 * (c) Chris Whealy 2020
 */
use std::fmt;

use crate::structs::ranges::RangeU16;

/***********************************************************************************************************************
 * Range check values
 */
const ANGLE_RANGE: RangeU16 = RangeU16 {
  min: 0,
  default: 0,
  max: 89,
};

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
    SoundConfig::new(ANGLE_RANGE.default).unwrap()
  }

  pub fn new(angle_arg: u16) -> Result<SoundConfig, SoundError> {
    if !ANGLE_RANGE.contains(angle_arg) {
      Err(SoundError::new(
        "Incident angle",
        UNITS_ANGLE,
        ANGLE_RANGE.min,
        ANGLE_RANGE.max,
        angle_arg,
      ))
    } else {
      Ok(SoundConfig { angle: angle_arg })
    }
  }
}
