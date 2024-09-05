/***********************************************************************************************************************
* Porous Absorber Calculator - Microperforated panel properties
*
* (c) Chris Whealy 2020
*/
extern crate num_format;

use super::GenericError;
use crate::{
    config::{constants, ranges::NamedRange},
    utils::maths_functions::TAU,
};

/***********************************************************************************************************************
 * Range check values
 */
const THICKNESS_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_THICKNESS,
    units: constants::UNITS_THICKNESS,
    min: 0.5,
    default: 1.0,
    max: 10.0,
};

const CENTRES_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_CENTRES,
    units: constants::UNITS_CENTRES,
    min: 0.5,
    default: 5.0,
    max: 10.0,
};

const RADIUS_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_RADIUS,
    units: constants::UNITS_RADIUS,
    min: 0.05,
    default: 0.25,
    max: 0.5,
};

const DEFAULT_POROSITY: f64 =
    ((TAU / 2.0) * RADIUS_RANGE.default * RADIUS_RANGE.default) / (CENTRES_RANGE.default * CENTRES_RANGE.default);

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
    ) -> Result<MicroperforatedPanelConfig, GenericError> {
        if !THICKNESS_RANGE.contains(thickness_arg) {
            return Err(GenericError::new_from_f64(THICKNESS_RANGE, thickness_arg));
        }

        if !CENTRES_RANGE.contains(centres_arg) {
            return Err(GenericError::new_from_f64(CENTRES_RANGE, centres_arg));
        }

        if !RADIUS_RANGE.contains(radius_arg) {
            return Err(GenericError::new_from_f64(RADIUS_RANGE, radius_arg));
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
