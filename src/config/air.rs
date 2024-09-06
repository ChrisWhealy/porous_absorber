/***********************************************************************************************************************
 * Porous Absorber Calculator - Air properties
 *
 * (c) Chris Whealy 2020
 */
use super::GenericError;
use crate::{
    config::{constants, ranges::NamedRange},
    utils::maths_functions::TAU,
};
use serde_derive::{Deserialize, Serialize};

/***********************************************************************************************************************
 * Air constants
 */
// Gas constant (J/Kg.K)
const GAS_CONSTANT: f64 = 287.05;
// Specific heat ratio
// const GAMMA: f64 = 1.402;
// Air density at 0C (Kg.m^-3)
// const AIR_DENSITY_0: f64 = 1.293;
const ONE_ATM: f64 = 101325.0; // One atmosphere (Pa)
const KELVIN_OFFSET: f64 = 273.15; // Zero celsius in degrees Kelvin

pub const AIR_VISCOSITY: f64 = 0.0000185; // Kinemetric viscosity of air (m^2/s)

pub fn air_density(pressure: f64, temp: i16) -> f64 {
    (pressure * ONE_ATM) / (GAS_CONSTANT * (temp as f64 + KELVIN_OFFSET))
}

pub fn sound_velocity(temp: f64) -> f64 {
    // ((GAMMA * ONE_ATM) / AIR_DENSITY_0).sqrt() = 331.4614688
    331.4614688 * (1.0 + (temp / KELVIN_OFFSET)).sqrt()
}

/***********************************************************************************************************************
 * Air pressure and temperature range check values
 */
const TEMP_RANGE: NamedRange<i16> = NamedRange {
    name: constants::TXT_AIR_TEMP,
    units: constants::UNITS_TEMP,
    min: -20,
    default: 20,
    max: 100,
};

const PRESSURE_RANGE: NamedRange<f64> = NamedRange {
    name: constants::TXT_AIR_PRESSURE,
    units: constants::UNITS_PRESSURE,
    min: 0.8,
    default: 1.0,
    max: 1.1,
};

/***********************************************************************************************************************
 * Air properties
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct AirConfig {
    pub temperature: i16,
    pub pressure: f64,
    pub density: f64,
    pub velocity: f64,
    pub impedance: f64,
    pub tau_over_c: f64,
    pub c_over_tau: f64,
    pub density_over_viscosity: f64,
}

impl AirConfig {
    pub fn default() -> AirConfig {
        AirConfig::new(TEMP_RANGE.default, PRESSURE_RANGE.default).unwrap()
    }

    pub fn new(temp_arg: i16, pressure_arg: f64) -> Result<AirConfig, GenericError> {
        if !TEMP_RANGE.contains(temp_arg) {
            return Err(GenericError::new_from_i16(TEMP_RANGE, temp_arg));
        }

        if !PRESSURE_RANGE.contains(pressure_arg) {
            return Err(GenericError::new_from_f64(PRESSURE_RANGE, pressure_arg));
        }

        let den = air_density(pressure_arg, temp_arg);
        let vel = sound_velocity(temp_arg as f64);

        Ok(AirConfig {
            temperature: temp_arg,
            pressure: pressure_arg,
            density: den,
            velocity: vel,
            impedance: vel * den,
            tau_over_c: TAU / vel,
            c_over_tau: vel / TAU,
            density_over_viscosity: den / AIR_VISCOSITY,
        })
    }
}
