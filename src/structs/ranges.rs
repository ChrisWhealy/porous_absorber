/***********************************************************************************************************************
 * Porous Absorber Calculator - Ranges
 *
 * (c) Chris Whealy 2020
 */
#[derive(Debug)]
pub struct RangeF64 {
  pub min: f64,
  pub default: f64,
  pub max: f64,
}
#[derive(Debug)]
pub struct RangeI16 {
  pub min: i16,
  pub default: i16,
  pub max: i16,
}

#[derive(Debug)]
pub struct RangeU16 {
  pub min: u16,
  pub default: u16,
  pub max: u16,
}

#[derive(Debug)]
pub struct RangeU32 {
  pub min: u32,
  pub default: u32,
  pub max: u32,
}

impl RangeF64 {
  pub fn contains(&self, some_val: f64) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}

impl RangeI16 {
  pub fn contains(&self, some_val: i16) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}

impl RangeU16 {
  pub fn contains(&self, some_val: u16) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}

impl RangeU32 {
  pub fn contains(&self, some_val: u32) -> bool {
    some_val >= self.min && some_val <= self.max
  }
}
