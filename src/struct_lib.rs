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
  pub name        : &'a str
, pub plot_colour : JsValue
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Font metadata
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct FontMetadata<'a> {
  pub typeface     : &'a str
, pub font_size    : f64
, pub stroke_style : &'a JsValue
}

impl<'a> FontMetadata<'a> {
  pub fn font(&self) -> String {
    format!("{}px {}", self.font_size, self.typeface)
  }
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
// Absorption data for a Rigib Backed Porous Absorber
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct PorousAbsInfo {
  pub air_gap    : Vec<PlotPoint>
, pub no_air_gap : Vec<PlotPoint>
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Absorption data for a Perforated Panel Absorber
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct PerforatedAbsInfo {
  pub abs_against_panel   : Vec<PlotPoint>
, pub abs_against_backing : Vec<PlotPoint>
, pub no_air_gap          : Vec<PlotPoint>
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Absorption data for a Slotted Panel Absorber
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct SlottedAbsInfo {
  pub abs_against_panel   : Vec<PlotPoint>
, pub abs_against_backing : Vec<PlotPoint>
, pub no_air_gap          : Vec<PlotPoint>
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Absorption data for a Microperforated Panel Absorber
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Debug)]
pub struct MicroperforatedAbsInfo {
  pub data : Vec<PlotPoint>
}
