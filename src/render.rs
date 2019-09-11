// *********************************************************************************************************************
// Plot graph
// *********************************************************************************************************************
extern crate wasm_bindgen;
extern crate web_sys;

use crate::struct_lib::{PlotPoint, Axis, AxisOrientation, SeriesMetadata};
use crate::display::DisplayConfig;

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

const TITLE_FONT_HEIGHT : f64 = 36.0;
const LABEL_FONT_HEIGHT : f64 = 20.0;

const TICK_LENGTH    : f64 = 10.0;
const TICK_LABEL_GAP : f64 = 5.0;
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
pub fn plot(absorber_info: &crate::PorousAbsInfo, display_cfg: &DisplayConfig) {
  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id("graph_canvas").unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let air_gap_series = SeriesMetadata {
    name  : &"Air Gap"
  , point_colour : JsValue::from(RGB_PINK)
  };

  let no_air_gap_series = SeriesMetadata {
    name  : &"No Air Gap"
  , point_colour : JsValue::from(RGB_DARK_BLUE)
  };

  clear(&canvas);
  draw_title_and_key(&canvas, &"Overall absorption at the specified angle", vec!(&air_gap_series, &no_air_gap_series));
  draw_axes(&canvas, &display_cfg);
  draw_splines(&canvas, &absorber_info.air_gap,    &air_gap_series.point_colour);
  draw_splines(&canvas, &absorber_info.no_air_gap, &no_air_gap_series.point_colour);
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
// Draw chart title and key
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn draw_title_and_key(canvas: &web_sys::HtmlCanvasElement, title: &str, series_list: Vec<&SeriesMetadata>) {
  let my_name     = &"draw_title_key";
  let fn_boundary = fn_boundary_trace(DEBUG, my_name);

  fn_boundary(true);

  let ctx = get_2d_context(&canvas);
  ctx.save();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Set font and stroke colour, then measure title width
  ctx.set_font(&format!("{}px {}", TITLE_FONT_HEIGHT, BASE_FONT));
  ctx.set_stroke_style(&JsValue::from(RGB_BLACK));

  let title_width = ctx.measure_text(title).unwrap().width();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add chart title
  ctx.fill_text(&title, LEFT_MARGIN_INSET, TOP_MARGIN_INSET).unwrap();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Add series key
  ctx.set_font(&format!("{}px {}", LABEL_FONT_HEIGHT, BASE_FONT));

  let mut x = LEFT_MARGIN_INSET + title_width + 50.0;
  let     y = TOP_MARGIN_INSET - (TITLE_FONT_HEIGHT / 2.0);

  for series in series_list {
    ctx.begin_path();
    ctx.move_to(x, y);
    ctx.line_to(x + 30.0, y);
    ctx.stroke();

    draw_point(&ctx, &PlotPoint { x : x + 15.0, y : y }, &series.point_colour);

    ctx.fill_text(series.name, x + 40.0, y + (LABEL_FONT_HEIGHT / 2.0) - 3.0).unwrap();

//    log(&format!("'{}' is {} pixels wide", series.name, ctx.measure_text(series.name).unwrap().width()));

    x += ctx.measure_text(series.name).unwrap().width() * 2.0;
  }

  ctx.restore();

  fn_boundary(false);
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw graph axes
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn draw_axes(canvas: &web_sys::HtmlCanvasElement, display_cfg: &DisplayConfig) {
  let my_name     = &"draw_axes";
  let fn_boundary = fn_boundary_trace(DEBUG, my_name);

  fn_boundary(true);

  let chart_origin = &PlotPoint { x : LEFT_AXIS_INSET, y : canvas.height() as f64 - BOTTOM_AXIS_INSET };

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Draw Y axis
  let abs_strs = vec!(
    String::from("0.0")
  , String::from("0.1")
  , String::from("0.2")
  , String::from("0.3")
  , String::from("0.4")
  , String::from("0.5")
  , String::from("0.6")
  , String::from("0.7")
  , String::from("0.8")
  , String::from("0.9")
  , String::from("1.0")
  );

  draw_axis(&canvas, &Axis {
    title          : &"Absorption"
  , start_point    : chart_origin.clone()
  , end_point      : PlotPoint { x : LEFT_AXIS_INSET, y : BOTTOM_AXIS_INSET }
  , values         : abs_strs
  , orientation    : AxisOrientation::Vertical
  , tick_length    : TICK_LENGTH
  , tick_label_gap : TICK_LABEL_GAP
  });

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Draw X axis
  let mut freq_strs : Vec<String> = vec!();
  display_cfg
    .frequencies
    .iter()
    .fold(
      ()
    , | _, f |
        if f == &62.5 {
          freq_strs.push(String::from("62.5"))
        }
        else {
          freq_strs.push(String::from(format!("{}",f.round() as u32)))
        }
    );

  draw_axis(&canvas, &Axis {
    title          : &"Frequency (Hz)"
  , start_point    : chart_origin.clone()
  , end_point      : PlotPoint { x : canvas.width() as f64 - LEFT_AXIS_INSET, y : canvas.height() as f64 - BOTTOM_AXIS_INSET }
  , values         : freq_strs
  , orientation    : AxisOrientation::Horizontal
  , tick_length    : TICK_LENGTH
  , tick_label_gap : TICK_LABEL_GAP
  });

  fn_boundary(false);
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw an axis
fn draw_axis(canvas: &web_sys::HtmlCanvasElement, axis_info: &Axis) {
  let my_name = &"draw_axis";

  let fn_boundary      = fn_boundary_trace(DEBUG, my_name);
  let write_to_console = fn_trace(DEBUG, my_name);

  fn_boundary(true);

  write_to_console("Plotting axis");

  // Define context values
  let (mid_height, mid_width, bottom_margin_pos, _, _) = canvas_dimensions(&canvas);
  let tick_interval : f64 = axis_info.tick_interval();

  let ctx = get_2d_context(&canvas);

  ctx.set_font(&format!("{}px {}", LABEL_FONT_HEIGHT, BASE_FONT));
  ctx.set_stroke_style(&JsValue::from(RGB_BLACK));
  
  let axis_label_width = ctx.measure_text(axis_info.title).unwrap().width();

  // Save context state
  ctx.save();
  ctx.begin_path();

  // Draw the axis line
  ctx.move_to(axis_info.start_point.x, axis_info.start_point.y);
  ctx.line_to(axis_info.end_point.x, axis_info.end_point.y);

  // Relocate origin to axis start point
  ctx.translate(axis_info.start_point.x, axis_info.start_point.y).unwrap();

  // For a horizontal axis, the tick labels must be rotated
  match axis_info.orientation {
    AxisOrientation::Horizontal => ctx.rotate(-PI_OVER_TWO).unwrap()
  , AxisOrientation::Vertical   => ()
  }

  for val in axis_info.values.iter() {
    let tick_label  = &format!("{}", val);
    let label_width = ctx.measure_text(tick_label).unwrap().width();

    // Position the label away from the tick by the tick length plus a gap
    let label_offset = label_width + axis_info.tick_length + axis_info.tick_label_gap;

    // Add tick
    ctx.move_to(-TICK_LENGTH, 0.0);
    ctx.line_to(0.0, 0.0);

    // Draw tick then move origin to next tick location
    match axis_info.orientation {
      AxisOrientation::Vertical   => {
        ctx.fill_text(tick_label, -label_offset, axis_info.tick_label_gap).unwrap();
        ctx.translate(0.0, -tick_interval).unwrap();
      }
    , AxisOrientation::Horizontal => {
        ctx.fill_text(tick_label, -label_offset, LABEL_FONT_HEIGHT / 2.0).unwrap();
        ctx.translate(0.0, tick_interval).unwrap();
      }
    }
  }

  // Reset horizontal axis rotation
  match axis_info.orientation {
    AxisOrientation::Horizontal => ctx.rotate(PI_OVER_TWO).unwrap()
  , AxisOrientation::Vertical   => ()
  }

  ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();

  // Reposition origin before writing axis title
  match &axis_info.orientation {
    // Y axis
    AxisOrientation::Vertical => {
      ctx.translate(LEFT_MARGIN_INSET, mid_height + (axis_label_width / 2.0)).unwrap();
      ctx.rotate(-PI_OVER_TWO).unwrap();
    }

    // X axis
  , AxisOrientation::Horizontal =>
      ctx.translate(mid_width - (axis_label_width / 2.0), bottom_margin_pos).unwrap()
  }

  // Write axis title
  ctx.fill_text(axis_info.title, 0.0, 0.0).unwrap();

  ctx.stroke();
  ctx.restore();
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
fn distance(pt1: &PlotPoint, pt2: &PlotPoint) -> f64 {
  sqrt(pow(pt1.x - pt2.x, 2.0) + pow(pt1.y - pt2.y, 2.0))
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generate the two control points that lie between the three supplied plot points
fn gen_control_points(pt1: &PlotPoint, pt2: &PlotPoint, pt3: &PlotPoint) -> Vec<PlotPoint> {
  // Vector from start point to finish point
  // This is used to determine the gradient of the lines through the control points
  let x_vec = pt3.x - pt1.x;
  let y_vec = pt3.y - pt1.y;

  let d01  = distance(pt1, pt2);
  let d12  = distance(pt2, pt3);
  let d012 = d01 + d12;

  // Return the coordinates of the two control points between the three current points
  return vec![
    PlotPoint { x : pt2.x - x_vec * TENSION * d01 / d012, y : pt2.y - y_vec * TENSION * d01 / d012 }
  , PlotPoint { x : pt2.x + x_vec * TENSION * d12 / d012, y : pt2.y + y_vec * TENSION * d12 / d012 }
  ]
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw a plot point
fn draw_point(ctx: &web_sys::CanvasRenderingContext2d, pt: &PlotPoint, fill_style: &JsValue) {
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
, abs_points    : &Vec<PlotPoint>
, stroke_colour : &JsValue
) {
  let my_name          = &"draw_splines";
  let fn_boundary      = fn_boundary_trace(DEBUG, my_name);
  let write_to_console = fn_trace(DEBUG, my_name);

  fn_boundary(true);

  let (_, _, _, x_axis_length, y_axis_length) = canvas_dimensions(&canvas);

  let ctx = get_2d_context(&canvas);

  let x_tick_interval = x_axis_length / (abs_points.len() - 1) as f64;
  let y_pos           = scaled_y_pos(canvas.height() as f64 - BOTTOM_AXIS_INSET, y_axis_length);

  // The frequency and absorption values need to be translated into canvas coordinates
  let mut points : Vec<PlotPoint> = vec!();

  for idx in 0..abs_points.len() {
    points.push(PlotPoint {
      x : LEFT_AXIS_INSET + x_tick_interval * idx as f64
    , y : y_pos(abs_points[idx].y)
    })
  }

  // Between each triplet of plot points, there will be two invisible control points
  let mut cps: Vec<PlotPoint> = vec!();
  
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
fn draw_curved_path(ctx: &web_sys::CanvasRenderingContext2d, cps: &Vec<PlotPoint>, points: &Vec<PlotPoint>) {
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
