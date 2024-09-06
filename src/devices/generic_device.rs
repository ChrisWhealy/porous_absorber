/***********************************************************************************************************************
 * Porous Absorber Calculator - Generic absorber device properties
 *
 * (c) Chris Whealy 2020
 */
use serde_derive::Serialize;

use crate::{
    config::{
        cavity::CavityConfig, chart::SeriesData,
    },
    devices::{
        microperforated_panel::config::MicroperforatedPanelConfig,
        perforated_panel::config::PerforatedPanelConfig,
        porous_absorber::config::PorousLayerConfig,
        slotted_panel::config::SlottedPanelConfig,
    },
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
    pub cavity: &'a CavityConfig,
}
