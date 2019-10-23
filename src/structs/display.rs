// *********************************************************************************************************************
// Porous Absorber Calculator
//
// Display properties
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************

use std::error::Error;
use std::fmt;
use libm::{pow, log2, fabs};
use serde::Serialize;

/***********************************************************************************************************************
 * Graph start frequency and octave subdivision range check values
 */
const START_FREQ   : f64  = 20.0;
const DEFAULT_FREQ : f64  = 62.5;
const END_FREQ     : f64  = 100.0;

const UNITS_FREQ : &str = "Hz";

const SUBDIVISIONS        : [u32; 4] = [1,2,3,6];
const DEFAULT_SUBDIVISION : u32 = 3;

const DISPLAY_OCTAVES : u32 = 8;

/***********************************************************************************************************************
 * Possible errors when creating display struct
 */
enum ErrType {
  Graph
, Subdivision
}

#[derive(Debug)]
pub struct DisplayError {
  msg : String
}

impl DisplayError {
  fn new(err_type: ErrType, err_val: f64) -> DisplayError {
    return match err_type {
      ErrType::Graph => 
        DisplayError {
          msg : format!(
                  "Graph start frequency must be a value in {} between {:?} and {:?}, not '{:?}'"
                , UNITS_FREQ
                , START_FREQ
                , END_FREQ
                , err_val
                )
        }
    , ErrType::Subdivision => 
        DisplayError {
          msg : format!("Octave subdivisions argument must be either 1, 2, 3 or 6, not '{}'", err_val.round())
        }
    };
  }
}

impl fmt::Display for DisplayError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for DisplayError {
  fn description(&self) -> &str {
    &self.msg
  }
}

/***********************************************************************************************************************
 * Generate the required frequencies from the number of display octaves and the subdivisions
 * The range upper bound must be included in order not to omit the 16KHz value
 */
fn gen_frequencies(graph_start_freq : &f64, subdivisions : &u32) -> Vec<f64> {
  let intervals: Vec<u32> = (0..=(DISPLAY_OCTAVES * subdivisions)).collect();

  intervals
    .iter()
    .fold(
      vec!()
    , | mut acc, interval_no | {
        acc.push(
          if interval_no == &0 {
            *graph_start_freq
          }
          else {
            pow(2.0,log2(*graph_start_freq) + (*interval_no as f64)/(*subdivisions as f64))
          }
        );

        acc
      } 
    )
}

/***********************************************************************************************************************
 * Display configuration
 */
pub struct DisplayConfig {
  pub graph_start_freq : f64
, pub smooth_curve     : bool
, pub subdivisions     : u32
, pub frequencies      : Vec<f64>
}


impl DisplayConfig {
  pub fn default() -> DisplayConfig {
    DisplayConfig::new(DEFAULT_FREQ, false, DEFAULT_SUBDIVISION).unwrap()
  }

  pub fn new(start_freq_arg: f64, smooth_curve: bool, subdivisions_arg: u32) -> Result<DisplayConfig, DisplayError> {
    if start_freq_arg < START_FREQ ||
       start_freq_arg > END_FREQ {
      return Err(DisplayError::new(ErrType::Graph, start_freq_arg))
    }

    if !SUBDIVISIONS.contains(&subdivisions_arg) {
      return Err(DisplayError::new(ErrType::Subdivision, subdivisions_arg as f64))
    }

    return
      Ok(DisplayConfig {
          graph_start_freq : start_freq_arg
        , subdivisions     : subdivisions_arg
        , smooth_curve     : smooth_curve
        , frequencies      : gen_frequencies(&start_freq_arg, &subdivisions_arg)
        })
  }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Plot point for simple canvas locations
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub struct PlotPoint {
  pub x : f64
, pub y : f64
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Plot Absorption Point links a {Freqency, Absorption} pair with a canvas (x,y) location
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize, Clone)]
pub struct PlotAbsPoint {
  pub x    : f64
, pub y    : f64
, pub freq : f64
, pub abs  : f64
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Series data and metadata
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub struct SeriesData<'a> {
  pub name        : &'a str
, pub plot_points : Vec<PlotAbsPoint>
}

#[derive(Debug)]
pub struct SeriesMetadata<'a> {
  pub name        : &'a str
, pub plot_colour : &'a str
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Font metadata
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct FontMetadata<'a> {
  pub typeface     : &'a str
, pub font_size    : f64
, pub stroke_style : &'a str
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
  Horizontal
, Vertical
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Axis properties
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct Axis<'a> {
  pub title          : &'static str
, pub start_point    : &'a PlotPoint
, pub end_point      : &'a PlotPoint
, pub values         : Vec<String>
, pub orientation    : AxisOrientation
, pub label_font     : &'a FontMetadata<'a>
, pub tick_length    : f64
, pub tick_label_gap : f64
}

impl<'a> Axis<'a> {
  pub fn length(&self) -> f64 {
    match &self.orientation {
      AxisOrientation::Horizontal => fabs(self.end_point.x - self.start_point.x)
    , AxisOrientation::Vertical   => fabs(self.end_point.y - self.start_point.y)
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
  pub top_left     : PlotPoint
, pub bottom_right : PlotPoint
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Chart Information to be returned to JavaScript
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug, Serialize)]
pub struct ChartInfo<'a> {
  pub chart_box   : ChartBox
, pub series_data : Vec<SeriesData<'a>>
}

