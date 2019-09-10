// *********************************************************************************************************************
// Plot graph
// *********************************************************************************************************************
extern crate wasm_bindgen;
extern crate web_sys;

use crate::struct_lib::{AbsInfo, Point};

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};

use std::f64::consts::PI;

use libm::{sqrt, pow};

const LIB_NAME: &str = "render";
const ENTER_FN: &str = "---->";
const EXIT_FN : &str = "<----";

const DEBUG: bool = false;

const PI_OVER_TWO : f64 = PI / 2.0;
const TWO_PI      : f64 = 2.0 * PI;

// *********************************************************************************************************************
// Canvas constants
// *********************************************************************************************************************
const LEFT_AXIS_INSET     : f64 = 100.0;
const LEFT_MARGIN_INSET   : f64 = 35.0;
const TOP_MARGIN_INSET    : f64 = 50.0;
const BOTTOM_AXIS_INSET   : f64 = 100.0;
const BOTTOM_MARGIN_INSET : f64 = 17.5;

const PLOT_AREA_COLOUR : &str = &"rgba(255, 255, 238)";
const RGB_BLACK        : &str = &"rgba(0, 0, 0)";
const RGB_PINK         : &str = &"rgb(234, 51, 247)";
const RGB_DARK_BLUE    : &str = &"rgb(6, 1, 123)";
const BASE_FONT        : &str = &"Arial";

const TITLE_FONT_HEIGHT      : f64 = 36.0;
const LABEL_FONT_HEIGHT      : f64 = 20.0;
const HALF_LABEL_FONT_HEIGHT : f64 = LABEL_FONT_HEIGHT / 2.0;

const TICK_LENGTH    : f64 = 10.0;
const LABEL_TICK_GAP : f64 = 5.0;
const POINT_RADIUS   : f64 = 5.0;
const TENSION        : f64 = 0.45;

// *********************************************************************************************************************
// Interface to browser functionality
// *********************************************************************************************************************
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

fn fn_boundary_trace(is_debug: bool, fn_name: &'static str) -> impl Fn(bool) -> () {
  move | is_enter: bool |
    if is_debug {
      log(&format!("{} {}::{}()", if is_enter { ENTER_FN } else { EXIT_FN }, LIB_NAME, fn_name))
    }
    else {
      ()
    }
}

fn fn_trace(is_debug: bool, fn_name: &'static str) -> impl Fn(&str) -> () {
  move | txt: &str |
    if is_debug {
      log(&format!("      {}::{}() {}", LIB_NAME, fn_name, txt))
    }
    else {
      ()
    }
}

// *********************************************************************************************************************
//
// Public API
//
// *********************************************************************************************************************
pub fn plot(absorber_info: &crate::AbsInfo) {
  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id("graph_canvas").unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let rgb_pink      = JsValue::from(RGB_PINK);
  let rgb_dark_blue = JsValue::from(RGB_DARK_BLUE);

  clear(&canvas);
  draw_axes(&canvas, &absorber_info);
  draw_splines(&canvas, &absorber_info.air_gap,    &"With air gap",    &rgb_pink);
  draw_splines(&canvas, &absorber_info.no_air_gap, &"Without air gap", &rgb_dark_blue);
}


// *********************************************************************************************************************
//
// Private API
//
// *********************************************************************************************************************

fn get_2d_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
  canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw graph axes
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn draw_axes(canvas: &web_sys::HtmlCanvasElement, abs_info: &AbsInfo) {
  let my_name = &"draw_axes";

  let fn_boundary      = fn_boundary_trace(DEBUG, my_name);
  let write_to_console = fn_trace(DEBUG, my_name);

  fn_boundary(true);

  let (mid_height, mid_width, bottom_margin_pos, x_axis_length, y_axis_length) = canvas_dimensions(&canvas);

  let ctx = get_2d_context(&canvas);

  let canvas_height = canvas.height() as f64;
  let canvas_width  = canvas.width() as f64;

  let rgb_black = JsValue::from(RGB_BLACK);


  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Set font and stroke colour, then measure title width
  let title_typeface = String::from(format!("{}px {}", TITLE_FONT_HEIGHT, BASE_FONT));

  ctx.set_font(&title_typeface);
  ctx.set_stroke_style(&rgb_black);

  let title       = String::from("Overall absorption at the specified angle");
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
  ctx.begin_path();

  // Draw Y axis (Absorption)
  ctx.move_to(LEFT_AXIS_INSET, BOTTOM_AXIS_INSET);
  ctx.line_to(LEFT_AXIS_INSET, canvas_height - BOTTOM_AXIS_INSET);

  // Draw x axis (Frequency)
  ctx.line_to(canvas_width - LEFT_AXIS_INSET, canvas_height - BOTTOM_AXIS_INSET);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add x axis scale
  write_to_console("Plotting X axis");

  let x_tick_interval = x_axis_length / (abs_info.air_gap.len() - 1) as f64;

  ctx.translate(LEFT_AXIS_INSET , canvas_height - BOTTOM_AXIS_INSET).unwrap();
  ctx.rotate(-PI_OVER_TWO).unwrap();

  for f in abs_info.air_gap.iter() {
    let tick_label  = &format!("{:?}", f.x.round() as u32);
    let label_width = ctx.measure_text(tick_label).unwrap().width();

    // Position the label away from the tick by the tick length plus a gap
    let label_offset = label_width + TICK_LENGTH + LABEL_TICK_GAP;

    // Add tick
    ctx.move_to(0.0, 0.0);
    ctx.line_to(-TICK_LENGTH, 0.0);

    // Add tick label
    ctx.fill_text(tick_label, -label_offset, HALF_LABEL_FONT_HEIGHT).unwrap();

    // Due to the 90˚ rotation currently active, we need to swap the order of arguments passed to translate
    ctx.translate(0.0, x_tick_interval).unwrap();
  }

  // Reset rotation and translation
  ctx.rotate(PI_OVER_TWO).unwrap();
  ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();

  // Add x axis label
  ctx.fill_text(&x_axis_label, mid_width - (x_axis_label_width / 2.0), bottom_margin_pos).unwrap();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add y axis scale
  write_to_console("Plotting Y axis");

  let y_tick_interval = y_axis_length / 10.0;

  ctx.translate(LEFT_AXIS_INSET , canvas_height - BOTTOM_AXIS_INSET).unwrap();

  for abs in [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0].iter() {
    let tick_label  = &format!("{:?}",abs);
    let label_width = ctx.measure_text(tick_label).unwrap().width();

    // Position the label away from the tick by the tick length plus a gap
    let label_offset = label_width + TICK_LENGTH + LABEL_TICK_GAP;

    // Add tick
    ctx.move_to(-TICK_LENGTH, 0.0);
    ctx.line_to(0.0, 0.0);

    // Add tick label
    ctx.fill_text(tick_label, -label_offset, LABEL_TICK_GAP).unwrap();

    // Move origin to next tick location
    ctx.translate(0.0, -y_tick_interval).unwrap();
  }

  ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();

  // Add y axis label
  ctx.translate(LEFT_MARGIN_INSET, mid_height + (y_axis_label_width / 2.0)).unwrap();
  ctx.rotate(-PI_OVER_TWO).unwrap();
  ctx.fill_text(&y_axis_label, 0.0, 0.0).unwrap();

  ctx.rotate(PI_OVER_TWO).unwrap();
  ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add chart title
  ctx.set_font(&title_typeface);
  ctx.fill_text(&title, mid_width - (title_width / 2.0), TOP_MARGIN_INSET).unwrap();

  ctx.stroke();

  fn_boundary(false);
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Return a tuple of various canvas dimensions
fn canvas_dimensions(canvas: &web_sys::HtmlCanvasElement) -> (f64, f64, f64, f64, f64) {
  let h = canvas.height() as f64;
  let w = canvas.width() as f64;

  ( h / 2.0                       // vertical midpoint
  , w / 2.0                       // horizontal midpoint
  , h - BOTTOM_MARGIN_INSET       // bottom margin position
  , w - (2.0 * LEFT_AXIS_INSET)   // X axis length
  , h - (2.0 * BOTTOM_AXIS_INSET) // Y axis length
  )
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Clear the entire canvas
fn clear(canvas: &web_sys::HtmlCanvasElement) {
  let ctx = get_2d_context(&canvas);

  ctx.save();
  ctx.set_fill_style(&JsValue::from(PLOT_AREA_COLOUR));
  ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
  ctx.restore();
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Return the distance between two points
fn distance(pt1: &Point, pt2: &Point) -> f64 {
  sqrt(pow(pt1.x - pt2.x, 2.0) + pow(pt1.y - pt2.y, 2.0))
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generate the two control points that lie between the three supplied plot points
fn gen_control_points(pt1: &Point, pt2: &Point, pt3: &Point) -> Vec<Point> {
  // Vector from start point to finish point
  // This is used to determine the gradient of the lines through the control points
  let x_vec = pt3.x - pt1.x;
  let y_vec = pt3.y - pt1.y;

  let d01  = distance(pt1, pt2);
  let d12  = distance(pt2, pt3);
  let d012 = d01 + d12;

  // Return the coordinates of the two control points between the three current points
  return vec![
    Point { x : pt2.x - x_vec * TENSION * d01 / d012, y : pt2.y - y_vec * TENSION * d01 / d012 }
  , Point { x : pt2.x + x_vec * TENSION * d12 / d012, y : pt2.y + y_vec * TENSION * d12 / d012 }
  ]
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw a plot point
fn draw_point(ctx: &web_sys::CanvasRenderingContext2d, pt: &Point, fill_style: &JsValue) {
  ctx.begin_path();
  ctx.save();
  ctx.set_fill_style(fill_style);
  ctx.arc(pt.x, pt.y, POINT_RADIUS, 0.0, TWO_PI).unwrap();
  ctx.fill();
  ctx.restore();
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw curve splines
fn draw_splines(
  canvas        : &web_sys::HtmlCanvasElement
, abs_points    : &Vec<Point>
, plot_name     : &str
, stroke_colour : &JsValue
) {
  let my_name          = &"draw_splines";
  let fn_boundary      = fn_boundary_trace(DEBUG, my_name);
  let write_to_console = fn_trace(DEBUG, my_name);

  fn_boundary(true);
  write_to_console(&format!("Plotting {}", plot_name));

  let (_, _, _, x_axis_length, y_axis_length) = canvas_dimensions(&canvas);

  let ctx = get_2d_context(&canvas);

  let x_tick_interval = x_axis_length / (abs_points.len() - 1) as f64;
  let y_pos           = scaled_y_pos(canvas.height() as f64 - BOTTOM_AXIS_INSET, y_axis_length);

  // The frequency and absorption values need to be translated into canvas coordinates
  let mut points : Vec<Point> = vec!();

  for idx in 0..abs_points.len() {
    points.push(Point {
      x : LEFT_AXIS_INSET + x_tick_interval * idx as f64
    , y : y_pos(abs_points[idx].y)
    })
  }

  // Between each triplet of plot points, there will be two invisible control points
  let mut cps: Vec<Point> = vec!();
  
  for idx in 0..points.len() - 2 {
    cps.append(&mut gen_control_points(&points[idx], &points[idx + 1], &points[idx + 2]));
  }

  // Draw all the plot points
  for idx in 0..points.len() {
    draw_point(&ctx, &points[idx], &stroke_colour)
  }

  draw_curved_path(&ctx, &cps, &points);

  fn_boundary(false);
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw a smooth curve between the plot points
fn draw_curved_path(ctx: &web_sys::CanvasRenderingContext2d, cps: &Vec<Point>, points: &Vec<Point>) {
  // As long as we have at least two points...
  if points.len() >= 2 {
    // First point
    ctx.begin_path();
    ctx.move_to(points[0].x, points[0].y);

    // Are there only 2 points?
    if points.len() == 2 {
      // Yup, so draw a line to the last point and we're done
      ctx.line_to(points[1].x, points[1].y);
    }
    else {
      // For 3 or more points...
      // Plot points 0 and 1 are connected with a quadratic Bezier
      ctx.quadratic_curve_to(cps[0].x, cps[0].y, points[1].x, points[1].y);

      // All middle plot points are connected with a cubic Bezier that requires a pair of control points
      for i in 2..points.len() - 1 {
        let cp_idx1 = (i - 2) * 2 + 1;
        let cp_idx2 = (i - 1) * 2;

        ctx.bezier_curve_to(cps[cp_idx1].x, cps[cp_idx1].y, cps[cp_idx2].x, cps[cp_idx2].y, points[i].x, points[i].y);
      }

      // Last two plot points are connected with a quadratic Bezier
      ctx.quadratic_curve_to(cps[cps.len() - 1].x, cps[cps.len() - 1].y, points[points.len() - 1].x, points[points.len() - 1].y);
    }

    // Draw the curve
    ctx.stroke();
  }
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Scale the Y axis (absorption value) to a canvas pixel location
fn scaled_y_pos(start: f64, axis_length: f64) -> impl Fn(f64) -> f64 {
  move | this_y: f64 | start - (this_y * axis_length)
}
