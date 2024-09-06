/***********************************************************************************************************************
 * Porous Absorber Calculator - Perforated panel properties
 *
 * (c) Chris Whealy 2020
 */
use serde_derive::{Deserialize, Serialize};
use super::GenericError;
use crate::{
    config::{constants, ranges::NamedRange},
    utils::{maths_functions::TAU},
};

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

const CENTRES_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_CENTRES,
    units: constants::UNITS_CENTRES,
    min: 2.0,
    default: 25.4,
    max: 300.0,
};

const RADIUS_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_RADIUS,
    units: constants::UNITS_RADIUS,
    min: 1.0,
    default: 12.7,
    max: 50.0,
};

const DEFAULT_POROSITY: f64 =
    ((TAU / 2.0) * RADIUS_RANGE.default * RADIUS_RANGE.default) / (CENTRES_RANGE.default * CENTRES_RANGE.default);

/***********************************************************************************************************************
 * Perforated panel configuration
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct PerforatedPanelConfig {
    pub thickness_mm: f64,
    pub thickness: f64,
    pub hole_centres: f64,
    pub hole_centres_mm: f64,
    pub hole_radius: f64,
    pub hole_radius_mm: f64,
    pub porosity: f64,
}

impl PerforatedPanelConfig {
    pub fn default() -> PerforatedPanelConfig {
        PerforatedPanelConfig::new(
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
    ) -> Result<PerforatedPanelConfig, GenericError> {
        if thickness_arg < THICKNESS_RANGE.min || thickness_arg > THICKNESS_RANGE.max {
            return Err(GenericError::new_from_f64(THICKNESS_RANGE, thickness_arg));
        }

        if centres_arg < CENTRES_RANGE.min || centres_arg > CENTRES_RANGE.max {
            return Err(GenericError::new_from_f64(CENTRES_RANGE, centres_arg));
        }

        if radius_arg < RADIUS_RANGE.min || radius_arg > RADIUS_RANGE.max {
            return Err(GenericError::new_from_f64(RADIUS_RANGE, radius_arg));
        }

        Ok(PerforatedPanelConfig {
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
