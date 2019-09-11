use libm::fabs;
use wasm_bindgen::JsValue;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Plot point
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct PlotPoint {
  pub x : f64
, pub y : f64
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Series metadata
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct SeriesMetadata<'a> {
  pub name         : &'a str
, pub point_colour : JsValue
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Axis dimensions
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub enum AxisOrientation {
  Horizontal
, Vertical
}

#[derive(Debug)]
pub struct Axis<'a> {
  pub title          : &'static str
, pub start_point    : &'a PlotPoint
, pub end_point      : PlotPoint
, pub values         : Vec<String>
, pub orientation    : AxisOrientation
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
// Porous Absorber
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct PorousAbsInfo {
  pub air_gap    : Vec<PlotPoint>
, pub no_air_gap : Vec<PlotPoint>
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Perforated Panel Absorber
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct PerforatedPanelInfo {
  pub abs_against_panel   : Vec<PlotPoint>
, pub abs_against_backing : Vec<PlotPoint>
, pub no_air_gap          : Vec<PlotPoint>
}
