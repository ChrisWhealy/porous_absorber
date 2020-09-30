/***********************************************************************************************************************
 * Porous Absorber Calculator - Chart Series, Axis Label, Font metadata
 *
 * (c) Chris Whealy 2020
 */
use crate::chart::render;
use crate::config::display::{FontMetadata, SeriesMetadata};

pub const TXT_AIR_GAP: &str = "Air Gap";
pub const TXT_NO_AIR_GAP: &str = "No Air Gap";
pub const TXT_ABS_AGAINST_PANEL: &str = "Absorber Against Panel";
pub const TXT_ABS_AGAINST_BACKING: &str = "Absorber Against Backing";

pub const TXT_MP_PANEL: &str = "Microperforated Panel";

pub const TXT_Y_AXIS_TITLE: &str = "Absorption";
pub const TXT_X_AXIS_TITLE: &str = "Frequency (Hz)";

/***********************************************************************************************************************
 * Chart titles
 */
pub const CHART_TITLE_OVERALL_ABS: &str = "Overall Absorption";
pub const CHART_TITLE_NORMAL_INCIDENCE: &str = "Normal Incidence Absorption";

pub fn chart_title_at_incident_angle(title: &str, angle: u16) -> String {
  format!("{} at {}°", title, angle)
}
/***********************************************************************************************************************
 * Chart series and font metadata
 */
pub const METADATA_AIR_GAP: SeriesMetadata = SeriesMetadata {
  name: TXT_AIR_GAP,
  plot_colour: render::constants::RGB_PINK,
};
pub const METADATA_NO_AIR_GAP: SeriesMetadata = SeriesMetadata {
  name: TXT_NO_AIR_GAP,
  plot_colour: render::constants::RGB_GREEN,
};
pub const METADATA_ABS_PANEL: SeriesMetadata = SeriesMetadata {
  name: TXT_ABS_AGAINST_PANEL,
  plot_colour: render::constants::RGB_DARK_BLUE,
};
pub const METADATA_ABS_BACKING: SeriesMetadata = SeriesMetadata {
  name: TXT_ABS_AGAINST_BACKING,
  plot_colour: render::constants::RGB_PINK,
};
pub const METADATA_MP_PANEL: SeriesMetadata = SeriesMetadata {
  name: TXT_MP_PANEL,
  plot_colour: render::constants::RGB_DARK_BLUE,
};

pub const FONT_METADATA_TITLE: FontMetadata = FontMetadata {
  typeface: &render::constants::BASE_TYPEFACE,
  font_size: render::constants::TITLE_FONT_SIZE,
  stroke_style: render::constants::RGB_BLACK,
};
pub const FONT_METADATA_LABEL: FontMetadata = FontMetadata {
  typeface: &render::constants::BASE_TYPEFACE,
  font_size: render::constants::LABEL_FONT_SIZE,
  stroke_style: render::constants::RGB_BLACK,
};
