/***********************************************************************************************************************
 * Porous Absorber Calculator - Rigid Backed Porous Absorber
 *
 * (c) Chris Whealy 2020
 */
extern crate num_format;

use std::fmt;

use crate::config::{constants, ranges::NamedRange};
use crate::utils::validation;

/***********************************************************************************************************************
 * NamedRange check values
 */
const THICKNESS_RANGE: NamedRange<u16> = NamedRange {
  name: constants::TXT_THICKNESS,
  units: constants::UNITS_THICKNESS,
  min: 5,
  default: 30,
  max: 500,
};

const FLOW_RESISTIVITY_RANGE: NamedRange<u32> = NamedRange {
  name: constants::TXT_FLOW_RESISTIVITY,
  units: constants::UNITS_THICKNESS,
  min: 1000,
  default: 16500,
  max: 100000,
};

/***********************************************************************************************************************
 * Possible errors when creating porous absorber struct
 */
#[derive(Debug)]
pub struct PorousLayerError {
  pub msg: String,
}

impl PorousLayerError {
  pub fn new_from_u16(range: NamedRange<u16>, err_val: u16) -> PorousLayerError {
    PorousLayerError {
      msg: validation::failure_msg(range, err_val),
    }
  }

  pub fn new_from_u32(range: NamedRange<u32>, err_val: u32) -> PorousLayerError {
    PorousLayerError {
      msg: validation::failure_msg(range, err_val),
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
  pub thickness_mm: u16,
  pub thickness: f64,
  pub sigma: u32,
}

impl PorousLayerConfig {
  pub fn default() -> PorousLayerConfig {
    PorousLayerConfig::new(THICKNESS_RANGE.default, FLOW_RESISTIVITY_RANGE.default).unwrap()
  }

  pub fn new(thickness_arg: u16, sigma_arg: u32) -> Result<PorousLayerConfig, PorousLayerError> {
    if !THICKNESS_RANGE.contains(thickness_arg) {
      return Err(PorousLayerError::new_from_u16(THICKNESS_RANGE, thickness_arg));
    }

    if !FLOW_RESISTIVITY_RANGE.contains(sigma_arg) {
      return Err(PorousLayerError::new_from_u32(FLOW_RESISTIVITY_RANGE, sigma_arg));
    }

    Ok(PorousLayerConfig {
      thickness_mm: thickness_arg,
      thickness: thickness_arg as f64 / 1000.0,
      sigma: sigma_arg,
    })
  }
}
