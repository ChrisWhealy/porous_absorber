/***********************************************************************************************************************
 * Porous Absorber Calculator - Sound properties
 *
 * (c) Chris Whealy 2020
 */
use std::fmt;

use crate::config::{constants, ranges::Range};
use crate::utils::validation;

/***********************************************************************************************************************
 * Range check values
 */
const ANGLE_RANGE: Range<u16> = Range {
  name: constants::TXT_INCIDENT_ANGLE,
  units: constants::UNITS_ANGLE,
  min: 0,
  default: 0,
  max: 89,
};

/***********************************************************************************************************************
 * Possible errors when creating sound struct
 */
#[derive(Debug)]
pub struct SoundError {
  msg: String,
}

impl SoundError {
  fn new(range: Range<u16>, err_val: u16) -> SoundError {
    SoundError {
      msg: validation::failure_msg(range, err_val),
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
      Err(SoundError::new(ANGLE_RANGE, angle_arg))
    } else {
      Ok(SoundConfig { angle: angle_arg })
    }
  }
}
