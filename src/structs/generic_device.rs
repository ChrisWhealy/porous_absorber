// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Properties of a generic absorber device
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************

use serde::Serialize;

use crate::structs::{
  config_display::SeriesData
, config_porous_layer::PorousLayerConfig
, config_cavity::CavityConfig
, panel_slotted::SlottedPanelConfig
, panel_perforated::PerforatedPanelConfig
, panel_microperforated::MicroperforatedPanelConfig
};


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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Dimensions and absorption data for a generic absorber device
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub struct GenericDeviceInfo<'a> {
  pub device_type  : DeviceType
, pub abs_series   : Vec<SeriesData<'a>>
, pub sl_panel     : Option<&'a SlottedPanelConfig>
, pub pf_panel     : Option<&'a PerforatedPanelConfig>
, pub mp_panel     : Option<&'a MicroperforatedPanelConfig>
, pub porous_layer : Option<&'a PorousLayerConfig>
, pub cavity       : Option<&'a CavityConfig>
}
