/***********************************************************************************************************************
 * Porous Absorber Calculator - Chart Constants
 *
 * (c) Chris Whealy 2020
 */
use crate::structs::config_display::{FontMetadata, SeriesMetadata};
use std::f64::consts::PI;

/***********************************************************************************************************************
 * Canvas constants
 */
pub const PI_OVER_TWO: f64 = PI / 2.0;

// These names must correspond to the element ids used in index.html
pub const GRAPH_CANVAS_ID: &str = "graph_canvas";
pub const WALL_IMG_ID: &str = "wall_img";
pub const ABSORBER_IMG_ID: &str = "absorber_img";
pub const PANEL_IMG_ID: &str = "panel_img";

pub const WALL_IMG_WIDTH: f64 = 20.0;

pub const Y_AXIS_INSET_DIAGRAM: f64 = 300.0; // Distance of Y axis from left edge if diagram is displayed
pub const Y_AXIS_INSET_NO_DIAGRAM: f64 = 100.0; // Distance of Y axis from left edge if diagram is not displayed
pub const X_AXIS_INSET: f64 = 100.0; // Distance of X axis from bottom edge

pub const LEFT_MARGIN_INSET: f64 = 35.0;
pub const RIGHT_MARGIN_INSET: f64 = 50.0;
pub const TOP_MARGIN_INSET: f64 = 50.0;
pub const BOTTOM_MARGIN_INSET: f64 = 17.5;

pub const RGB_BLACK: &str = "rgb(0, 0, 0)";
pub const RGB_PINK: &str = "rgb(234, 51, 247)";
pub const RGB_LIGHT_PINK: &str = "rgb(246, 195, 203)";
pub const RGB_DARK_BLUE: &str = "rgb(6, 1, 123)";
pub const RGB_GREEN: &str = "rgb(20, 255, 20)";
pub const RGB_OFF_WHITE: &str = "rgb(255, 255, 238)";

pub const BASE_TYPEFACE: &str = "Arial";
pub const TITLE_FONT_SIZE: f64 = 36.0;
pub const LABEL_FONT_SIZE: f64 = 20.0;

pub const TICK_LENGTH: f64 = 10.0;
pub const TICK_LABEL_GAP: f64 = 5.0;
pub const PLOT_POINT_RADIUS: f64 = 5.0;

// Scale factor for magnifying the holes in a microperforated panel
pub const MP_SCALE_FACTOR: f64 = 20.0;

/***********************************************************************************************************************
 * Text constants
 */
pub const TXT_AIR_GAP: &str = "Air Gap";
pub const TXT_NO_AIR_GAP: &str = "No Air Gap";
pub const TXT_ABS_PANEL: &str = "Absorber Against Panel";
pub const TXT_ABS_BACKING: &str = "Absorber Against Backing";
pub const TXT_MP_PANEL: &str = "Microperforated Panel";

pub const TXT_Y_AXIS_TITLE: &str = "Absorption";
pub const TXT_X_AXIS_TITLE: &str = "Frequency (Hz)";

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
  name: TXT_ABS_PANEL,
  plot_colour: RGB_DARK_BLUE,
};
pub const METADATA_ABS_BACKING: SeriesMetadata = SeriesMetadata {
  name: TXT_ABS_BACKING,
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
