/***********************************************************************************************************************
 * Porous Absorber Calculator - Ranges
 *
 * (c) Chris Whealy 2020
 */
#[derive(Debug)]
pub struct Range<T> {
  pub name: &'static str,
  pub units: &'static str,
  pub min: T,
  pub default: T,
  pub max: T,
}

impl Range<f64> {
  pub fn contains(&self, some_val: f64) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}

impl Range<i16> {
  pub fn contains(&self, some_val: i16) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}

impl Range<u16> {
  pub fn contains(&self, some_val: u16) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}

impl Range<u32> {
  pub fn contains(&self, some_val: u32) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}
