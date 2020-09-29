/***********************************************************************************************************************
 * Porous Absorber Calculator - Generic absorber device properties
 *
 * (c) Chris Whealy 2020
 */
use serde::Serialize;

use crate::structs::{
  config_cavity::CavityConfig, config_display::SeriesData, config_porous_layer::PorousLayerConfig,
  panel_microperforated::MicroperforatedPanelConfig, panel_perforated::PerforatedPanelConfig,
  panel_slotted::SlottedPanelConfig,
};

/***********************************************************************************************************************
 * Absorption data
 */
#[derive(Debug, Serialize)]
pub enum DeviceType {
  RigidBackedPorousAbsorber,
  PerforatedPanelAbsorber,
  SlottedPanelAbsorber,
  MicroperforatedPanelAbsorber,
}

/***********************************************************************************************************************
 * Dimensions and absorption data
 */
#[derive(Debug, Serialize)]
pub struct GenericDeviceInfo<'a> {
  pub device_type: DeviceType,
  pub abs_series: Vec<SeriesData<'a>>,
  pub sl_panel: Option<&'a SlottedPanelConfig>,
  pub pf_panel: Option<&'a PerforatedPanelConfig>,
  pub mp_panel: Option<&'a MicroperforatedPanelConfig>,
  pub porous_layer: Option<&'a PorousLayerConfig>,
  pub cavity: Option<&'a CavityConfig>,
}
