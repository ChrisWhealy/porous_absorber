/***********************************************************************************************************************
 * Porous Absorber Calculator - Generic absorber device properties
 *
 * (c) Chris Whealy 2020
 */
pub mod microperforated_panel;
pub mod perforated_panel;
pub mod porous_absorber;
pub mod slotted_panel;

use microperforated_panel::config::MicroperforatedPanelConfig;
use perforated_panel::config::PerforatedPanelConfig;
use porous_absorber::config::PorousLayerConfig;
use serde_derive::Serialize;
use slotted_panel::config::SlottedPanelConfig;

use crate::config::{air::AirConfig, cavity::CavityConfig, chart::ChartConfig, chart::SeriesData, sound::SoundConfig};

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

/***********************************************************************************************************************
 * Optional configuration structs
 */
pub struct PanelConfigSet {
    pub panel_microperforated: Option<MicroperforatedPanelConfig>,
    pub panel_perforated: Option<PerforatedPanelConfig>,
    pub panel_slotted: Option<SlottedPanelConfig>,
}

/***********************************************************************************************************************
 * All absorption devices require configuration information for air, the cavity dimensions and the display parameters.
 * However, configuration for sound (angle of incidence), a panel and a porous layer is optional
 */
pub struct ConfigSet {
    pub air_config: AirConfig,
    pub cavity_config: CavityConfig,
    pub chart_config: ChartConfig,
    pub sound_config: Option<SoundConfig>,
    pub panel_config: Option<PanelConfigSet>,
    pub porous_config: Option<PorousLayerConfig>,
}
