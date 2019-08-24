use std::error::Error;
use std::fmt;

/***********************************************************************************************************************
 * Air pressure and temperature range check values
 */
const START_ANGLE   : u32 = 0;
const DEFAULT_ANGLE : u32 = START_ANGLE;
const END_ANGLE     : u32 = 89;

const UNITS_ANGLE : &str = "degrees";

/***********************************************************************************************************************
 * Possible errors when creating sound struct
 */
#[derive(Debug)]
pub struct SoundError {
  msg : String
}

impl SoundError {
  fn new(property: &str, units: &str, min: u32, max:u32, err_val: u32) -> SoundError {
    SoundError {
      msg : format!("{} must be a value in {} between {:?} and {:?}, not '{:?}'", property, units, min, max, err_val)
    }
  }
}

impl fmt::Display for SoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for SoundError {
  fn description(&self) -> &str {
    &self.msg
  }
}

/***********************************************************************************************************************
 * Sound configuration
 */
pub struct SoundConfig {
  pub angle : u32
}

impl SoundConfig {
  pub fn angle_as_string(&self) -> String { format!("{}˚", self.angle) }

  pub fn default() -> SoundConfig {
    SoundConfig::new(DEFAULT_ANGLE).unwrap()
  }

  pub fn new(angle_arg: u32) -> Result<SoundConfig, SoundError> {
    if angle_arg > 90 {
      return Err(SoundError::new("Incident angle", UNITS_ANGLE, START_ANGLE, END_ANGLE, angle_arg));
    }

    Ok(SoundConfig { angle : angle_arg })
  }
}

