/***********************************************************************************************************************
 * Generic validation failure message functions
 *
 * (c) Chris Whealy 2020
 */
use crate::config::ranges::NamedRange;

pub fn failure_msg<T: std::fmt::Debug>(range: NamedRange<T>, err_val: T) -> String {
    format!(
        "{} must be a value in {} between {:?} and {:?}, not '{:?}'",
        range.name, range.units, range.min, range.max, err_val
    )
}
