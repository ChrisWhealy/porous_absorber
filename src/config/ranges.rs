/***********************************************************************************************************************
 * Porous Absorber Calculator - Range within which a named parameter is valid
 *
 * (c) Chris Whealy 2020, 2024
 */
#[derive(Debug)]
pub struct NamedRange<T> {
    pub name: &'static str,
    pub units: &'static str,
    pub min: T,
    pub default: T,
    pub max: T,
}

impl<T> NamedRange<T> where T: PartialEq + PartialOrd {
    pub fn contains(&self, some_val: T) -> bool {
        some_val >= self.min && some_val <= self.max
    }
}
