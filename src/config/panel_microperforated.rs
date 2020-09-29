/***********************************************************************************************************************
* Porous Absorber Calculator - Microperforated panel properties
*
* (c) Chris Whealy 2020
*/
extern crate num_format;

use std::f64::consts::PI;
use std::fmt;

use crate::config::{constants, ranges::Range};
use crate::utils::validation;

/***********************************************************************************************************************
 * Range check values
 */
const THICKNESS_RANGE: Range<f64> = Range {
  name: constants::TXT_THICKNESS,
  units: constants::UNITS_THICKNESS,
  min: 0.5,
  default: 1.0,
  max: 10.0,
};

const CENTRES_RANGE: Range<f64> = Range {
  name: constants::TXT_CENTRES,
  units: constants::UNITS_CENTRES,
  min: 0.5,
  default: 5.0,
  max: 10.0,
};

const RADIUS_RANGE: Range<f64> = Range {
  name: constants::TXT_RADIUS,
  units: constants::UNITS_RADIUS,
  min: 0.05,
  default: 0.25,
  max: 0.5,
};

const DEFAULT_POROSITY: f64 =
  (PI * RADIUS_RANGE.default * RADIUS_RANGE.default) / (CENTRES_RANGE.default * CENTRES_RANGE.default);

/***********************************************************************************************************************
 * Possible errors when creating porous absorber struct
 */
#[derive(Debug)]
pub struct MicroperforatedPanelError {
  pub msg: String,
}

impl MicroperforatedPanelError {
  pub fn new(range: Range<f64>, err_val: f64) -> MicroperforatedPanelError {
    MicroperforatedPanelError {
      msg: validation::failure_msg(range, err_val),
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
  pub thickness_mm: f64,
  pub thickness: f64,
  pub hole_centres: f64,
  pub hole_centres_mm: f64,
  pub hole_radius: f64,
  pub hole_radius_mm: f64,
  pub porosity: f64,
}

impl MicroperforatedPanelConfig {
  pub fn default() -> MicroperforatedPanelConfig {
    MicroperforatedPanelConfig::new(
      THICKNESS_RANGE.default,
      CENTRES_RANGE.default,
      RADIUS_RANGE.default,
      DEFAULT_POROSITY,
    )
    .unwrap()
  }

  pub fn new(
    thickness_arg: f64,
    centres_arg: f64,
    radius_arg: f64,
    porosity_arg: f64,
  ) -> Result<MicroperforatedPanelConfig, MicroperforatedPanelError> {
    if !THICKNESS_RANGE.contains(thickness_arg) {
      return Err(MicroperforatedPanelError::new(THICKNESS_RANGE, thickness_arg));
    }

    if !CENTRES_RANGE.contains(centres_arg) {
      return Err(MicroperforatedPanelError::new(CENTRES_RANGE, centres_arg));
    }

    if !RADIUS_RANGE.contains(radius_arg) {
      return Err(MicroperforatedPanelError::new(RADIUS_RANGE, radius_arg));
    }

    Ok(MicroperforatedPanelConfig {
      thickness_mm: thickness_arg,
      thickness: thickness_arg / 1000.0,
      hole_centres_mm: centres_arg,
      hole_centres: centres_arg / 1000.0,
      hole_radius_mm: radius_arg,
      hole_radius: radius_arg / 1000.0,
      porosity: porosity_arg,
    })
  }
}
