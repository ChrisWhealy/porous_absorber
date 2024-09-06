/***********************************************************************************************************************
 * Porous Absorber Calculator - Chart Series, Axis Label, Font metadata
 *
 * (c) Chris Whealy 2020, 2024
 */
use crate::config::chart::{FontMetadata, SeriesMetadata};

pub const TXT_AIR_GAP: &str = "Air Gap";
pub const TXT_NO_AIR_GAP: &str = "No Air Gap";
pub const TXT_ABS_AGAINST_PANEL: &str = "Absorber Against Panel";
pub const TXT_ABS_AGAINST_BACKING: &str = "Absorber Against Backing";

pub const TXT_MP_PANEL: &str = "Microperforated Panel";

pub const TXT_Y_AXIS_TITLE: &str = "Absorption";
pub const TXT_X_AXIS_TITLE: &str = "Frequency (Hz)";

pub const RGB_BLACK: &str = "rgb(0, 0, 0)";
pub const RGB_PINK: &str = "rgb(234, 51, 247)";
pub const RGB_LIGHT_PINK: &str = "rgb(246, 195, 203)";
pub const RGB_DARK_BLUE: &str = "rgb(6, 1, 123)";
pub const RGB_GREEN: &str = "rgb(20, 255, 20)";
pub const RGB_OFF_WHITE: &str = "rgb(255, 255, 238)";

pub const BASE_TYPEFACE: &str = "Arial";
pub const TITLE_FONT_SIZE: f64 = 36.0;
pub const LABEL_FONT_SIZE: f64 = 20.0;

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
    plot_colour: RGB_PINK,
};

pub const METADATA_NO_AIR_GAP: SeriesMetadata = SeriesMetadata {
    name: TXT_NO_AIR_GAP,
    plot_colour: RGB_GREEN,
};

pub const METADATA_ABS_PANEL: SeriesMetadata = SeriesMetadata {
    name: TXT_ABS_AGAINST_PANEL,
    plot_colour: RGB_DARK_BLUE,
};

pub const METADATA_ABS_BACKING: SeriesMetadata = SeriesMetadata {
    name: TXT_ABS_AGAINST_BACKING,
    plot_colour: RGB_PINK,
};

pub const METADATA_MP_PANEL: SeriesMetadata = SeriesMetadata {
    name: TXT_MP_PANEL,
    plot_colour: RGB_DARK_BLUE,
};

pub const FONT_METADATA_TITLE: FontMetadata = FontMetadata {
    typeface: &BASE_TYPEFACE,
    font_size: TITLE_FONT_SIZE,
    stroke_style: RGB_BLACK,
};

pub const FONT_METADATA_LABEL: FontMetadata = FontMetadata {
    typeface: &BASE_TYPEFACE,
    font_size: LABEL_FONT_SIZE,
    stroke_style: RGB_BLACK,
};
