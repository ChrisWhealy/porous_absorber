// *********************************************************************************************************************
// Draw graph
// *********************************************************************************************************************
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};

use std::f64::consts::PI;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Canvas constants
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const LEFT_AXIS_INSET     : f64 = 100.0;
const LEFT_MARGIN_INSET   : f64 = 35.0;
const TOP_MARGIN_INSET    : f64 = 50.0;
const BOTTOM_AXIS_INSET   : f64 = 100.0;
const BOTTOM_MARGIN_INSET : f64 = 17.5;

const BASE_FONT: &str = &"Arial";

const TITLE_FONT_HEIGHT      : f64 = 36.0;
const LABEL_FONT_HEIGHT      : f64 = 20.0;
const HALF_LABEL_FONT_HEIGHT : f64 = LABEL_FONT_HEIGHT / 2.0;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Interface to browser functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
pub fn get_canvas() -> web_sys::HtmlCanvasElement {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("graph_canvas").unwrap();

  return canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();
}

pub fn get_2d_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
  canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw graph axes
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn draw_axes(canvas: &web_sys::HtmlCanvasElement, frequencies: &Vec<u32>) {
  let (mid_height, mid_width, bottom_margin_pos, x_axis_length, y_axis_length) = canvas_dimensions(&canvas);

  let ctx = get_2d_context(&canvas);

  let canvas_height = canvas.height() as f64;
  let canvas_width  = canvas.width() as f64;

  let rgb_black = JsValue::from("rgb(0, 0, 0)");


  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Set font, then measure title width and stroke colour
  let title          = String::from("Overall absorption at the specified angle");
  let title_typeface = String::from(format!("{}px {}", TITLE_FONT_HEIGHT, BASE_FONT));

  ctx.set_font(&title_typeface);
  ctx.set_stroke_style(&rgb_black);

  let title_width = ctx.measure_text(&title).unwrap().width();

  // Set font, then measure axis label widths
  let label_typeface = String::from(format!("{}px {}", LABEL_FONT_HEIGHT, BASE_FONT));
  ctx.set_font(&label_typeface);
  
  let x_axis_label       = String::from("Frequency (Hz)");
  let x_axis_label_width = ctx.measure_text(&x_axis_label).unwrap().width();

  let y_axis_label       = String::from("Absorption");
  let y_axis_label_width = ctx.measure_text(&y_axis_label).unwrap().width();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Start drawing
  ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
  ctx.begin_path();

  // Draw y axis (Absorption)
  ctx.move_to(LEFT_AXIS_INSET, BOTTOM_AXIS_INSET);
  ctx.line_to(LEFT_AXIS_INSET, canvas_height - BOTTOM_AXIS_INSET);

  // Draw x axis (Frequency)
  ctx.line_to(canvas_width - LEFT_AXIS_INSET, canvas_height - BOTTOM_AXIS_INSET);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add x axis scale
  let tick_length     = 10.0;
  let x_tick_interval = x_axis_length / (frequencies.len() - 1) as f64;

  ctx.translate(LEFT_AXIS_INSET , canvas_height - BOTTOM_AXIS_INSET).unwrap();
  ctx.rotate(-PI / 2.0).unwrap();

  for f in frequencies.iter() {
    let tick_label  = &format!("{:?}",f);
    let label_width = ctx.measure_text(tick_label).unwrap().width();

    // Position the label away from the tick by the tick length plus 5 pixels
    let label_offset = label_width + tick_length + 5.0;

    // Add tick
    ctx.move_to(0.0, 0.0);
    ctx.line_to(-tick_length, 0.0);

    // Add tick label
    ctx.fill_text(tick_label, -label_offset, HALF_LABEL_FONT_HEIGHT).unwrap();

    // Due to the 90˚ rotation currently active, the arguments to translate must be swapped
    ctx.translate(0.0, x_tick_interval).unwrap();
  }

  // Reset rotation and translation
  ctx.rotate(PI / 2.0).unwrap();
  ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();

  // Add x axis label
  ctx.fill_text(&x_axis_label, mid_width - (x_axis_label_width / 2.0), bottom_margin_pos).unwrap();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add y axis scale
  let y_tick_interval = y_axis_length / 9.0;

  ctx.translate(LEFT_AXIS_INSET , canvas_height - BOTTOM_AXIS_INSET).unwrap();

  for abs in [0.0, 0.1, 0.2, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0].iter() {
    let tick_label  = &format!("{:?}",abs);
    let label_width = ctx.measure_text(tick_label).unwrap().width();

    // Position the label away from the tick by the tick length plus 5 pixels
    let label_offset = label_width + tick_length + 5.0;

    // Add tick
    ctx.move_to(-tick_length, 0.0);
    ctx.line_to(0.0, 0.0);

    // Add tick label
    ctx.fill_text(tick_label, -label_offset, 7.5).unwrap();

    // Move origin to next tick location
    ctx.translate(0.0, -y_tick_interval).unwrap();
  }

  ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();

  // Add y axis label
  ctx.translate(LEFT_MARGIN_INSET, mid_height + (y_axis_label_width / 2.0)).unwrap();
  ctx.rotate(-PI / 2.0).unwrap();
  ctx.fill_text(&y_axis_label, 0.0, 0.0).unwrap();

  ctx.rotate(PI / 2.0).unwrap();
  ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add chart title
  ctx.set_font(&title_typeface);
  ctx.fill_text(&title, mid_width - (title_width / 2.0), TOP_MARGIN_INSET).unwrap();

  ctx.stroke();
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Plot absorption curve
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn draw_curve(
  canvas: &web_sys::HtmlCanvasElement
, abs_info: &crate::AbsInfo
) {
  let canvas_height = canvas.height() as f64;
  let ctx           = get_2d_context(&canvas);

  let (_, _, _, x_axis_length, y_axis_length) = canvas_dimensions(&canvas);

  let x_tick_interval = x_axis_length / (abs_info.frequencies.len() - 1) as f64;

  let rgb_pink      = JsValue::from("rgb(234, 51, 247)");
  let rgb_dark_blue = JsValue::from("rgb(6, 1, 123)");

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Plots absorption curves

  let mut x_pos = LEFT_AXIS_INSET;
  let mut y_pos;

  let y_start = canvas_height - BOTTOM_AXIS_INSET;

  ctx.begin_path();
  ctx.set_fill_style(&rgb_pink);
  ctx.set_stroke_style(&rgb_pink);

  // Plot absorption curve for absorber with air gap
  for abs in abs_info.air_gap.iter() {
    y_pos = y_start - (abs * y_axis_length);
    ctx.line_to(x_pos, y_pos);
    ctx.move_to(x_pos, y_pos);
    ctx.fill_rect(x_pos - 2.5, y_pos, 5.0, 5.0);
    x_pos += x_tick_interval;
  }

  ctx.stroke();

  // Reset draw locations and change drawing colour
  x_pos = LEFT_AXIS_INSET;

  ctx.begin_path();
  ctx.move_to(x_pos, y_start);
  ctx.set_fill_style(&rgb_dark_blue);
  ctx.set_stroke_style(&rgb_dark_blue);

  // Plot absorption curve for absorber without air gap
  for abs in abs_info.no_air_gap.iter() {
    y_pos = y_start - (abs * y_axis_length);
    ctx.line_to(x_pos, y_pos);
    ctx.move_to(x_pos, y_pos);
    ctx.fill_rect(x_pos - 2.5, y_pos, 5.0, 5.0);
    x_pos += x_tick_interval;
  }

  ctx.stroke();
}

// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************
fn canvas_dimensions(canvas: &web_sys::HtmlCanvasElement) -> (f64, f64, f64, f64, f64) {
  let h = canvas.height() as f64;
  let w = canvas.width() as f64;

  ( h / 2.0                       // mid height 
  , w / 2.0                       // mid width
  , h - BOTTOM_MARGIN_INSET       // bottom margin position
  , w - (2.0 * LEFT_AXIS_INSET)   // X axis length
  , h - (2.0 * BOTTOM_AXIS_INSET) // Y axis length
  )
}

