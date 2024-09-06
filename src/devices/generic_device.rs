/***********************************************************************************************************************
 * Porous Absorber Calculator - Generic absorber device properties
 *
 * (c) Chris Whealy 2020
 */
use serde_derive::Serialize;

use crate::chart::constants;
use crate::{
    config::{cavity::CavityConfig, chart::SeriesData},
    devices::{
        microperforated_panel::config::MicroperforatedPanelConfig, perforated_panel::config::PerforatedPanelConfig,
        porous_absorber::config::PorousLayerConfig, slotted_panel::config::SlottedPanelConfig,
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

impl<'a> GenericDeviceInfo<'a> {
    pub fn new(
        device_type: DeviceType,
        sl_panel: Option<&'a SlottedPanelConfig>,
        pf_panel: Option<&'a PerforatedPanelConfig>,
        mp_panel: Option<&'a MicroperforatedPanelConfig>,
        porous_layer: Option<&'a PorousLayerConfig>,
        cavity: &'a CavityConfig,
    ) -> Self {
        let abs_series = match device_type {
            DeviceType::RigidBackedPorousAbsorber => vec![
                SeriesData {
                    name: constants::TXT_AIR_GAP,
                    plot_points: vec![],
                },
                SeriesData {
                    name: constants::TXT_NO_AIR_GAP,
                    plot_points: vec![],
                },
            ],
            DeviceType::PerforatedPanelAbsorber => vec![
                SeriesData {
                    name: constants::TXT_NO_AIR_GAP,
                    plot_points: vec![],
                },
                SeriesData {
                    name: constants::TXT_ABS_AGAINST_PANEL,
                    plot_points: vec![],
                },
                SeriesData {
                    name: constants::TXT_ABS_AGAINST_BACKING,
                    plot_points: vec![],
                },
            ],
            DeviceType::SlottedPanelAbsorber => vec![
                SeriesData {
                    name: constants::TXT_NO_AIR_GAP,
                    plot_points: vec![],
                },
                SeriesData {
                    name: constants::TXT_ABS_AGAINST_PANEL,
                    plot_points: vec![],
                },
                SeriesData {
                    name: constants::TXT_ABS_AGAINST_BACKING,
                    plot_points: vec![],
                },
            ],
            DeviceType::MicroperforatedPanelAbsorber => vec![SeriesData {
                name: constants::TXT_MP_PANEL,
                plot_points: vec![],
            }],
        };

        GenericDeviceInfo {
            device_type,
            abs_series,
            sl_panel,
            pf_panel,
            mp_panel,
            porous_layer,
            cavity,
        }
    }
}
