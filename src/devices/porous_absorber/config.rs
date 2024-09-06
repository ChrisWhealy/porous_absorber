/***********************************************************************************************************************
 * Porous Absorber Calculator - Porous Layer Configuration
 *
 * (c) Chris Whealy 2020
 */
use serde_derive::{Deserialize, Serialize};
use super::GenericError;
use crate::config::{constants, ranges::NamedRange};

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

    pub fn new(thickness_arg: u16, sigma_arg: u32) -> Result<PorousLayerConfig, GenericError> {
        if !THICKNESS_RANGE.contains(thickness_arg) {
            return Err(GenericError::new_from_u16(THICKNESS_RANGE, thickness_arg));
        }

        if !FLOW_RESISTIVITY_RANGE.contains(sigma_arg) {
            return Err(GenericError::new_from_u32(FLOW_RESISTIVITY_RANGE, sigma_arg));
        }

        Ok(PorousLayerConfig {
            thickness_mm: thickness_arg,
            thickness: thickness_arg as f64 / 1000.0,
            sigma: sigma_arg,
        })
    }
}
