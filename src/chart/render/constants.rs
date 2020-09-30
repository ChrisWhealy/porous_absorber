/***********************************************************************************************************************
 * Porous Absorber Calculator - Chart Rendering Constants
 *
 * (c) Chris Whealy 2020
 */
use crate::config::display::PlotPoint;

pub const ORIGIN: PlotPoint = PlotPoint { x: 0.0, y: 0.0 };

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

pub const TICK_LENGTH: f64 = 10.0;
pub const TICK_LABEL_GAP: f64 = 5.0;
pub const PLOT_POINT_RADIUS: f64 = 5.0;

pub const TITLE_KEY_GAP: f64 = 50.0;
pub const KEY_SYMBOL_LENGTH: f64 = 30.0;
pub const SYMBOL_TEXT_GAP: f64 = 10.0;

// Scale factor for magnifying the holes in a microperforated panel
pub const MP_SCALE_FACTOR: f64 = 20.0;
