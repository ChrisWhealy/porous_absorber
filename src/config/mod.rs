use crate::{config::ranges::NamedRange, utils::validation};
use std::fmt;

pub mod air;
pub mod cavity;
pub mod chart;
pub mod sound;

pub mod constants;
pub mod ranges;

/**********************************************************************************************************************/
#[derive(Debug)]
pub struct GenericError {
    pub msg: String,
}

impl GenericError {
    pub fn new_from_f64(range: NamedRange<f64>, err_val: f64) -> GenericError {
        GenericError {
            msg: validation::failure_msg(range, err_val),
        }
    }

    pub fn new_from_u16(range: NamedRange<u16>, err_val: u16) -> GenericError {
        GenericError {
            msg: validation::failure_msg(range, err_val),
        }
    }
    pub fn new_from_i16(range: NamedRange<i16>, err_val: i16) -> GenericError {
        GenericError {
            msg: validation::failure_msg(range, err_val),
        }
    }

    pub fn new_from_u32(range: NamedRange<u32>, err_val: u32) -> GenericError {
        GenericError {
            msg: validation::failure_msg(range, err_val),
        }
    }

    pub fn new_chart_err(err_type: chart::ErrType, err_val: f64) -> GenericError {
        match err_type {
            chart::ErrType::Graph => GenericError {
                msg: format!(
                    "Graph start frequency must be a value in {} between {:?} and {:?}, not '{:?}'",
                    constants::UNITS_FREQ,
                    chart::FREQ_RANGE.min,
                    chart::FREQ_RANGE.max,
                    err_val
                ),
            },
            chart::ErrType::Subdivision => GenericError {
                msg: format!(
                    "Octave subdivisions argument must be either 1, 2, 3 or 6, not '{}'",
                    err_val.round()
                ),
            },
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
