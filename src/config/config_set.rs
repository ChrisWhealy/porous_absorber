/***********************************************************************************************************************
 * Porous Absorber Calculator - Set of optional configuration structs
 *
 * (c) Chris Whealy 2020
 */
use crate::config::{
  air::AirConfig, cavity::CavityConfig, display::DisplayConfig, panel_microperforated::MicroperforatedPanelConfig,
  panel_perforated::PerforatedPanelConfig, panel_slotted::SlottedPanelConfig, porous_layer::PorousLayerConfig,
  sound::SoundConfig,
};

pub struct PanelConfigSet {
  pub panel_microperforated: Option<MicroperforatedPanelConfig>,
  pub panel_perforated: Option<PerforatedPanelConfig>,
  pub panel_slotted: Option<SlottedPanelConfig>,
}

/***********************************************************************************************************************
 * All absorption devices require configuration information for air, the cavity dimensions, display parameters and
 * sound; however, the presence of a panel is optional (as in the case of a simple porous absorber) as is the presence
 * of a porous layer (in the case of a microperforated absorber)
 */
pub struct ConfigSet {
  pub air_config: Option<AirConfig>,
  pub cavity_config: Option<CavityConfig>,
  pub display_config: Option<DisplayConfig>,
  pub sound_config: Option<SoundConfig>,
  pub panel_config: Option<PanelConfigSet>,
  pub porous_config: Option<PorousLayerConfig>,
}
