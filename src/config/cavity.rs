/***********************************************************************************************************************
 * Porous Absorber Calculator - Cavity properties
 *
 * (c) Chris Whealy 2020
 */
use serde_derive::{Deserialize, Serialize};
use super::GenericError;
use crate::config::{constants, ranges::NamedRange};

/***********************************************************************************************************************
 * Range check values
 */
const THICKNESS_RANGE: NamedRange<u16> = NamedRange {
    name: constants::TXT_AIR_GAP,
    units: constants::UNITS_THICKNESS,
    min: 0,
    default: 100,
    max: 500,
};

/***********************************************************************************************************************
 * Cavity configuration
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CavityConfig {
    pub air_gap_mm: u16,
    pub air_gap: f64,
}

impl CavityConfig {
    pub fn default() -> CavityConfig {
        CavityConfig::new(THICKNESS_RANGE.default).unwrap()
    }

    pub fn new(air_gap_arg: u16) -> Result<CavityConfig, GenericError> {
        if !THICKNESS_RANGE.contains(air_gap_arg) {
            Err(GenericError::new_from_u16(THICKNESS_RANGE, air_gap_arg))
        } else {
            Ok(CavityConfig {
                air_gap_mm: air_gap_arg,
                air_gap: air_gap_arg as f64 / 1000.0,
            })
        }
    }
}
