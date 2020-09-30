/***********************************************************************************************************************
 * Porous Absorber Calculator - Display properties
 *
 * (c) Chris Whealy 2020
 */
use libm::{fabs, log2, pow};
use serde::Serialize;
use std::fmt;

use crate::config::{constants, ranges::Range};
use crate::utils::validation;

/***********************************************************************************************************************
 * Graph start frequency and octave subdivision range check values
 */
const FREQ_RANGE: Range<f64> = Range {
  name: constants::TXT_FREQ_RANGE,
  units: constants::UNITS_FREQ,
  min: 20.0,
  default: 62.5,
  max: 100.0,
};

const SUBDIVISIONS: [u16; 4] = [1, 2, 3, 6];
const DEFAULT_SUBDIVISION: u16 = 3;

const DISPLAY_OCTAVES: u16 = 8;

/***********************************************************************************************************************
 * Possible errors when creating display struct
 */
enum ErrType {
  Graph,
  Subdivision,
}

#[derive(Debug)]
pub struct DisplayError {
  msg: String,
}

impl DisplayError {
  fn new(err_type: ErrType, err_val: f64) -> DisplayError {
    match err_type {
      ErrType::Graph => DisplayError {
        msg: validation::start_freq_err(FREQ_RANGE, err_val),
      },
      ErrType::Subdivision => DisplayError {
        msg: validation::oct_subdiv_err(err_val),
      },
    }
  }
}

impl fmt::Display for DisplayError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

/***********************************************************************************************************************
 * Generate the required frequencies from the number of display octaves and the subdivisions
 * The range upper bound must be included in order not to omit the 16KHz value
 */
fn gen_frequencies(graph_start_freq: &f64, subdivisions: &u16) -> Vec<f64> {
  let intervals: Vec<u16> = (0..=(DISPLAY_OCTAVES * subdivisions)).collect();

  intervals.iter().fold(vec![], |mut acc, interval_no| {
    acc.push(if interval_no == &0 {
      *graph_start_freq
    } else {
      pow(
        2.0,
        log2(*graph_start_freq) + (*interval_no as f64) / (*subdivisions as f64),
      )
    });

    acc
  })
}

/***********************************************************************************************************************
 * Display configuration
 */
pub struct DisplayConfig {
  pub graph_start_freq: f64,
  pub smooth_curve: bool,
  pub subdivisions: u16,
  pub show_diagram: bool,
  pub frequencies: Vec<f64>,
}

impl DisplayConfig {
  pub fn default() -> DisplayConfig {
    DisplayConfig::new(FREQ_RANGE.default, false, DEFAULT_SUBDIVISION, false).unwrap()
  }

  pub fn new(
    start_freq_arg: f64,
    smooth_curve: bool,
    subdivisions_arg: u16,
    show_diagram: bool,
  ) -> Result<DisplayConfig, DisplayError> {
    if !FREQ_RANGE.contains(start_freq_arg) {
      return Err(DisplayError::new(ErrType::Graph, start_freq_arg));
    }

    if !SUBDIVISIONS.contains(&subdivisions_arg) {
      return Err(DisplayError::new(ErrType::Subdivision, subdivisions_arg as f64));
    }

    Ok(DisplayConfig {
      graph_start_freq: start_freq_arg,
      subdivisions: subdivisions_arg,
      smooth_curve,
      show_diagram,
      frequencies: gen_frequencies(&start_freq_arg, &subdivisions_arg),
    })
  }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Dimension pair
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub struct DimensionPair {
  pub width: f64,
  pub height: f64,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Plot point for simple canvas locations
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Clone)]
pub struct PlotPoint {
  pub x: f64,
  pub y: f64,
}

impl PlotPoint {
  pub fn x_diff(&self, other_point: &Self) -> f64 {
    self.x - other_point.x
  }

  pub fn y_diff(&self, other_point: &Self) -> f64 {
    self.y - other_point.y
  }
}

impl std::fmt::Display for PlotPoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Plot Absorption Point links a {Freqency, Absorption} pair with a canvas (x,y) location
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Clone)]
pub struct PlotAbsPoint {
  pub at: PlotPoint,
  pub freq: f64,
  pub abs: f64,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Series data and metadata
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub struct SeriesData<'a> {
  pub name: &'a str,
  pub plot_points: Vec<PlotAbsPoint>,
}

#[derive(Debug)]
pub struct SeriesMetadata<'a> {
  pub name: &'a str,
  pub plot_colour: &'a str,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Font metadata
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct FontMetadata<'a> {
  pub typeface: &'a str,
  pub font_size: f64,
  pub stroke_style: &'a str,
}

impl<'a> FontMetadata<'a> {
  pub fn font(&self) -> String {
    format!("{}px {}", self.font_size, self.typeface)
  }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Axis orientation
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub enum AxisOrientation {
  Horizontal,
  Vertical,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Axis properties
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct Axis<'a> {
  pub title: &'static str,
  pub start_point: &'a PlotPoint,
  pub end_point: &'a PlotPoint,
  pub values: Vec<String>,
  pub orientation: AxisOrientation,
  pub label_font: &'a FontMetadata<'a>,
}

impl<'a> Axis<'a> {
  pub fn length(&self) -> f64 {
    match &self.orientation {
      AxisOrientation::Horizontal => fabs(self.end_point.x_diff(self.start_point)),
      AxisOrientation::Vertical => fabs(self.end_point.y_diff(self.start_point)),
    }
  }

  pub fn tick_interval(&self) -> f64 {
    self.length() / (&self.values.len() - 1) as f64
  }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Bounding box for the chart.  This defines the bounding box within which the cross-hairs appear
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub struct ChartBox {
  pub top_left: PlotPoint,
  pub bottom_right: PlotPoint,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Chart Information to be returned to JavaScript
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub struct ChartInfo<'a> {
  pub chart_box: ChartBox,
  pub series_data: Vec<SeriesData<'a>>,
}
