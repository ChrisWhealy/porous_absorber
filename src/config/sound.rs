/***********************************************************************************************************************
 * Porous Absorber Calculator - Sound properties
 *
 * (c) Chris Whealy 2020
 */
use super::GenericError;
use crate::config::{constants, ranges::NamedRange};

/***********************************************************************************************************************
 * Range check values
 */
const ANGLE_RANGE: NamedRange<u16> = NamedRange {
    name: constants::TXT_INCIDENT_ANGLE,
    units: constants::UNITS_ANGLE,
    min: 0,
    default: 0,
    max: 89,
};

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

    pub fn new(angle_arg: u16) -> Result<SoundConfig, GenericError> {
        if !ANGLE_RANGE.contains(angle_arg) {
            Err(GenericError::new_from_u16(ANGLE_RANGE, angle_arg))
        } else {
            Ok(SoundConfig { angle: angle_arg })
        }
    }
}
