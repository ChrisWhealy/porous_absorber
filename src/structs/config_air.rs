// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Air properties
//
// (c) Chris Whealy 2019
// *********************************************************************************************************************

use std::f64::consts::PI;
use std::fmt;

/***********************************************************************************************************************
 * Air constants
 */
const GAS_CONSTANT: f64 = 287.05; // Gas constant (J/Kg.K)
const GAMMA: f64 = 1.402; // Specific heat ratio
const AIR_DENSITY_0: f64 = 1.293; // Air density at 0C (Kg.m^-3)
const ONE_ATM: f64 = 101325.0; // One atmosphere (Pa)
const KELVIN_OFFSET: f64 = 273.15; // Zero celsius in degrees Kelvin

pub const AIR_VISCOSITY: f64 = 0.0000185; // Kinemetric viscosity of air (m^2/s)

pub fn air_density(pressure: f64, temp: i16) -> f64 {
  (pressure * ONE_ATM) / (GAS_CONSTANT * (temp as f64 + KELVIN_OFFSET))
}

pub fn sound_velocity(temp: f64) -> f64 {
  ((GAMMA * ONE_ATM) / AIR_DENSITY_0).sqrt() * (1.0 + (temp / KELVIN_OFFSET)).sqrt()
}

/***********************************************************************************************************************
 * Air pressure and temperature range check values
 */
const START_TEMP: i16 = -20;
const DEFAULT_TEMP: i16 = 20;
const END_TEMP: i16 = 100;

const START_PRESSURE: f64 = 0.8;
const DEFAULT_PRESSURE: f64 = 1.0;
const END_PRESSURE: f64 = 1.1;

const UNITS_TEMP: &str = "°C";
const UNITS_PRESSURE: &str = "bar";

/***********************************************************************************************************************
 * Possible errors when creating air properties
 */
#[derive(Debug)]
pub struct AirError {
  msg: String,
}

impl AirError {
  fn new(property: &str, units: &str, min: f64, max: f64, err_val: f64) -> AirError {
    AirError {
      msg: format!(
        "{} must be a value in {} between {:?} and {:?}, not '{:?}'",
        property, units, min, max, err_val
      ),
    }
  }
}

impl fmt::Display for AirError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

/***********************************************************************************************************************
 * Air properties
 */
#[derive(Debug)]
pub struct AirConfig {
  pub temperature: i16,
  pub pressure: f64,
  pub density: f64,
  pub velocity: f64,
  pub impedance: f64,
  pub two_pi_over_c: f64,
  pub c_over_two_pi: f64,
  pub density_over_viscosity: f64,
}

impl AirConfig {
  pub fn default() -> AirConfig {
    AirConfig::new(DEFAULT_TEMP, DEFAULT_PRESSURE).unwrap()
  }

  pub fn new(temp_arg: i16, pressure_arg: f64) -> Result<AirConfig, AirError> {
    if temp_arg < START_TEMP || temp_arg > END_TEMP {
      return Err(AirError::new(
        "Air temperature",
        UNITS_TEMP,
        START_TEMP as f64,
        END_TEMP as f64,
        temp_arg as f64,
      ));
    }

    if pressure_arg < START_PRESSURE || pressure_arg > END_PRESSURE {
      return Err(AirError::new(
        "Air pressure",
        UNITS_PRESSURE,
        START_PRESSURE,
        END_PRESSURE,
        pressure_arg,
      ));
    }

    let den = air_density(pressure_arg, temp_arg);
    let vel = sound_velocity(temp_arg as f64);

    Ok(AirConfig {
      temperature: temp_arg,
      pressure: pressure_arg,
      density: den,
      velocity: vel,
      impedance: vel * den,
      two_pi_over_c: (2.0 * PI) / vel,
      c_over_two_pi: vel / (2.0 * PI),
      density_over_viscosity: den / AIR_VISCOSITY,
    })
  }
}
