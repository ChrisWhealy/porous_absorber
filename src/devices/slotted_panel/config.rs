/***********************************************************************************************************************
 * Porous Absorber Calculator - Slotted panel properties
 *
 * (c) Chris Whealy 2020, 2024
 */
use serde_derive::{Deserialize, Serialize};
use super::GenericError;
use crate::config::{constants, ranges::NamedRange};

/***********************************************************************************************************************
 * Range check values
 */
const THICKNESS_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_THICKNESS,
    units: constants::UNITS_THICKNESS,
    min: 1.0,
    default: 10.0,
    max: 50.0,
};

const DISTANCE_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_DISTANCE,
    units: constants::UNITS_DISTANCE,
    min: 2.0,
    default: 25.4,
    max: 300.0,
};

const WIDTH_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_WIDTH,
    units: constants::UNITS_WIDTH,
    min: 1.0,
    default: 5.0,
    max: 50.0,
};

const DEFAULT_POROSITY: f64 = WIDTH_RANGE.default / (DISTANCE_RANGE.default + WIDTH_RANGE.default);

/***********************************************************************************************************************
 * Slotted panel configuration
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct SlottedPanelConfig {
    pub thickness_mm: f64,
    pub thickness: f64,
    pub slot_distance: f64,
    pub slot_distance_mm: f64,
    pub slot_width: f64,
    pub slot_width_mm: f64,
    pub porosity: f64,
}

impl SlottedPanelConfig {
    pub fn default() -> SlottedPanelConfig {
        SlottedPanelConfig::new(
            THICKNESS_RANGE.default,
            DISTANCE_RANGE.default,
            WIDTH_RANGE.default,
            DEFAULT_POROSITY,
        )
            .unwrap()
    }

    pub fn new(
        thickness_arg: f64,
        distance_arg: f64,
        width_arg: f64,
        porosity_arg: f64,
    ) -> Result<SlottedPanelConfig, GenericError> {
        if thickness_arg < THICKNESS_RANGE.min || thickness_arg > THICKNESS_RANGE.max {
            return Err(GenericError::new_from_f64(THICKNESS_RANGE, thickness_arg));
        }

        if distance_arg < DISTANCE_RANGE.min || distance_arg > DISTANCE_RANGE.max {
            return Err(GenericError::new_from_f64(DISTANCE_RANGE, distance_arg));
        }

        if width_arg < WIDTH_RANGE.min || width_arg > WIDTH_RANGE.max {
            return Err(GenericError::new_from_f64(WIDTH_RANGE, width_arg));
        }

        Ok(SlottedPanelConfig {
            thickness_mm: thickness_arg,
            thickness: thickness_arg / 1000.0,
            slot_distance_mm: distance_arg,
            slot_distance: distance_arg / 1000.0,
            slot_width_mm: width_arg,
            slot_width: width_arg / 1000.0,
            porosity: porosity_arg,
        })
    }
}
