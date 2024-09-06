use crate::{
    config::{
        air::AirConfig, cavity::CavityConfig, chart::ChartConfig,
        sound::SoundConfig,
    },
    devices::{
        microperforated_panel::config::MicroperforatedPanelConfig,
        perforated_panel::config::PerforatedPanelConfig,
        porous_absorber::config::PorousLayerConfig,
        slotted_panel::config::SlottedPanelConfig,
    },
};

/***********************************************************************************************************************
 * Config for device with a surface panel
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
