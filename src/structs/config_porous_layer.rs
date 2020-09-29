/***********************************************************************************************************************
 * Porous Absorber Calculator - Rigid Backed Porous Absorber
 *
 * (c) Chris Whealy 2020
 */
extern crate num_format;

use std::fmt;

use crate::structs::ranges::{RangeU16, RangeU32};

/***********************************************************************************************************************
 * Range check values
 */
const THICKNESS_RANGE: RangeU16 = RangeU16 {
  min: 5,
  default: 30,
  max: 500,
};

const FLOW_RESISTIVITY_RANGE: RangeU32 = RangeU32 {
  min: 1000,
  default: 16500,
  max: 100000,
};

const UNITS_THICKNESS: &str = "mm";
const UNITS_FLOW_RESISTIVITY: &str = "rayls/m";

/***********************************************************************************************************************
 * Possible errors when creating porous absorber struct
 */
#[derive(Debug)]
pub struct PorousLayerError {
  pub msg: String,
}

impl PorousLayerError {
  pub fn new(property: &str, units: &str, min: u32, max: u32, err_val: u32) -> PorousLayerError {
    PorousLayerError {
      msg: format!(
        "{} must be a value in {} between {:?} and {:?}, not '{:?}'",
        property, units, min, max, err_val
      ),
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
      return Err(PorousLayerError::new(
        "Thickness",
        UNITS_THICKNESS,
        THICKNESS_RANGE.min as u32,
        THICKNESS_RANGE.max as u32,
        thickness_arg as u32,
      ));
    }

    if !FLOW_RESISTIVITY_RANGE.contains(sigma_arg) {
      return Err(PorousLayerError::new(
        "Flow resistivity",
        UNITS_FLOW_RESISTIVITY,
        FLOW_RESISTIVITY_RANGE.min,
        FLOW_RESISTIVITY_RANGE.max,
        sigma_arg,
      ));
    }

    Ok(PorousLayerConfig {
      thickness_mm: thickness_arg,
      thickness: thickness_arg as f64 / 1000.0,
      sigma: sigma_arg,
    })
  }
}
