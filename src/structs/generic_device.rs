// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Properties of a generic absorber device
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************

use serde::Serialize;

use crate::structs::config_display::SeriesData;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Absorption data for a generic absorber device
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub enum DeviceType {
  RigidBackedPorousAbsorber
, PerforatedPanelAbsorber
, SlottedPanelAbsorber
, MicroperforatedPanelAbsorber
}

#[derive(Debug, Serialize)]
pub struct GenericDeviceInfo<'a> {
  pub device_type : DeviceType
, pub abs_series  : Vec<SeriesData<'a>>
}
