/***********************************************************************************************************************
 * Porous Absorber Calculator - Set of optional configuration structs
 *
 * (c) Chris Whealy 2020
 */
use crate::config::{
    air::AirConfig, cavity::CavityConfig, chart::ChartConfig, panel_microperforated::MicroperforatedPanelConfig,
    panel_perforated::PerforatedPanelConfig, panel_slotted::SlottedPanelConfig, porous_layer::PorousLayerConfig,
    sound::SoundConfig,
};

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
