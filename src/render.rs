// *********************************************************************************************************************
// Plot graph
// *********************************************************************************************************************
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::{JsValue, JsCast};
use std::f64::consts::PI;
use libm::{sqrt, pow};

use crate::structs::display::{
  PlotPoint
, PlotAbsPoint
, Axis
, AxisOrientation
, SeriesData
, SeriesMetadata
, FontMetadata
, DisplayConfig
, ChartBox
, ChartInfo
};

use crate::structs::porous_absorber::PorousAbsInfo;
use crate::structs::perforated_panel::PerforatedAbsInfo;
use crate::structs::microperforated_panel::MicroperforatedAbsInfo;
use crate::structs::slotted_panel::SlottedAbsInfo;
use crate::structs::sound::SoundConfig;

use crate::trace::Trace;

const PI_OVER_TWO : f64 = PI / 2.0;
const TWO_PI      : f64 = 2.0 * PI;

// *********************************************************************************************************************
// Canvas constants
// *********************************************************************************************************************
const CANVAS_NAME : &str = "graph_canvas";

const LEFT_AXIS_INSET     : f64 = 100.0;
const HORIZ_MARGIN_INSET  : f64 = 35.0;
const VERT_MARGIN_INSET   : f64 = 50.0;
const BOTTOM_AXIS_INSET   : f64 = 100.0;
const BOTTOM_MARGIN_INSET : f64 = 17.5;

const RGB_BLACK           : &str = &"rgba(0, 0, 0)";
const RGB_PINK            : &str = &"rgb(234, 51, 247)";
const RGB_LIGHT_PINK      : &str = &"rgb(246, 195, 203)";
const RGB_DARK_BLUE       : &str = &"rgb(6, 1, 123)";
const RGB_GREEN           : &str = &"rgb(20, 255, 20)";

const BASE_TYPEFACE       : &str = &"Arial";
const TITLE_FONT_SIZE     : f64  = 36.0;
const LABEL_FONT_SIZE     : f64  = 20.0;

const TICK_LENGTH         : f64 = 10.0;
const TICK_LABEL_GAP      : f64 = 5.0;
const PLOT_POINT_RADIUS   : f64 = 5.0;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const LIB_NAME     : &str  = &"render";
const TRACE_ACTIVE : &bool = &false;



// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************

// *********************************************************************************************************************
// Porous Absorber
pub fn plot_porous_absorber<'a>(
  absorber_info : &'a PorousAbsInfo
, display_cfg   : &DisplayConfig
, sound_cfg     : &SoundConfig
) -> ChartInfo<'a> {
  const SERIES1 : &str = &"Air Gap";
  const SERIES2 : &str = &"No Air Gap";

  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id(CANVAS_NAME).unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let air_gap_series    = SeriesMetadata { name : SERIES1, plot_colour : JsValue::from(RGB_PINK) };
  let no_air_gap_series = SeriesMetadata { name : SERIES2, plot_colour : JsValue::from(RGB_DARK_BLUE) };

  clear(&canvas);

  draw_title_and_key(
    &canvas
  , &format!("Overall absorption at {}°", sound_cfg.angle)
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : TITLE_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : LABEL_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , vec!(&air_gap_series, &no_air_gap_series)
  );

  return ChartInfo {
    chart_box   : draw_axes(&canvas, &display_cfg)
  , series_data : vec!(
      SeriesData {
        name        : SERIES1
      , plot_points : draw_splines(&canvas, &absorber_info.air_gap, &air_gap_series.plot_colour, &display_cfg.smooth_curve)
      }
    , SeriesData {
        name        : SERIES2
      , plot_points : draw_splines(&canvas, &absorber_info.no_air_gap, &no_air_gap_series.plot_colour, &display_cfg.smooth_curve)
      })
  };
}

// *********************************************************************************************************************
// Perforated Panel Absorber
pub fn plot_perforated_panel<'a>(
  absorber_info : &'a PerforatedAbsInfo
, display_cfg   : &DisplayConfig
) -> ChartInfo<'a> {
  const SERIES1 : &str = &"No Air Gap";
  const SERIES2 : &str = &"Absorber Against Panel";
  const SERIES3 : &str = &"Absorber Against Backing";

  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id(CANVAS_NAME).unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let no_air_gap_series          = SeriesMetadata { name : SERIES1, plot_colour : JsValue::from(RGB_GREEN) }; 
  let abs_against_panel_series   = SeriesMetadata { name : SERIES2, plot_colour : JsValue::from(RGB_DARK_BLUE) };
  let abs_against_backing_series = SeriesMetadata { name : SERIES3, plot_colour : JsValue::from(RGB_PINK) };

  clear(&canvas);

  draw_title_and_key(
    &canvas
  , &"Normal Incidence Absorption"
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : TITLE_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : LABEL_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , vec!(&abs_against_panel_series, &abs_against_backing_series, &no_air_gap_series)
  );

  return ChartInfo {
    chart_box   : draw_axes(&canvas, &display_cfg)
  , series_data : vec!(
      SeriesData {
        name        : SERIES1
      , plot_points : draw_splines(&canvas, &absorber_info.no_air_gap, &no_air_gap_series.plot_colour, &display_cfg.smooth_curve)
      }
    , SeriesData {
        name        : SERIES2
      , plot_points : draw_splines(&canvas, &absorber_info.abs_against_panel, &abs_against_panel_series.plot_colour, &display_cfg.smooth_curve)
      }
    , SeriesData {
        name        : SERIES3
      , plot_points : draw_splines(&canvas, &absorber_info.abs_against_backing, &abs_against_backing_series.plot_colour, &display_cfg.smooth_curve)
      })
  };
}


// *********************************************************************************************************************
// Microerforated Panel Absorber
pub fn plot_microperforated_panel<'a>(
  absorber_info : &'a MicroperforatedAbsInfo
, display_cfg   : &DisplayConfig
, sound_cfg     : &SoundConfig
) -> ChartInfo<'a> {
  const SERIES1 : &str = &"Microperforated Panel";

  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id(CANVAS_NAME).unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let mp_panel_series = SeriesMetadata { name : SERIES1, plot_colour : JsValue::from(RGB_DARK_BLUE) }; 

  clear(&canvas);

  draw_title_and_key(
    &canvas
  , &format!("Overall absorption at {}°", sound_cfg.angle)
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : TITLE_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : LABEL_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , vec!(&mp_panel_series)
  );

  return ChartInfo {
    chart_box   : draw_axes(&canvas, &display_cfg)
  , series_data : vec!(
      SeriesData {
        name        : SERIES1
      , plot_points : draw_splines(&canvas, &absorber_info.data, &mp_panel_series.plot_colour, &display_cfg.smooth_curve)
      })
  };
}


// *********************************************************************************************************************
// Slotted Panel Absorber
pub fn plot_slotted_panel<'a>(
  absorber_info : &'a SlottedAbsInfo
, display_cfg   : &DisplayConfig
) -> ChartInfo<'a> {
  const SERIES1 : &str = &"No Air Gap";
  const SERIES2 : &str = &"Absorber Against Panel";
  const SERIES3 : &str = &"Absorber Against Backing";

  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id(CANVAS_NAME).unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let no_air_gap_series          = SeriesMetadata { name : SERIES1, plot_colour : JsValue::from(RGB_GREEN) }; 
  let abs_against_panel_series   = SeriesMetadata { name : SERIES2, plot_colour : JsValue::from(RGB_DARK_BLUE) };
  let abs_against_backing_series = SeriesMetadata { name : SERIES3, plot_colour : JsValue::from(RGB_PINK) };

  clear(&canvas);

  draw_title_and_key(
    &canvas
  , &"Normal Incidence Absorption"
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : TITLE_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , &FontMetadata { typeface : &BASE_TYPEFACE, font_size : LABEL_FONT_SIZE, stroke_style : &JsValue::from(RGB_BLACK) }
  , vec!(&abs_against_panel_series, &abs_against_backing_series, &no_air_gap_series)
  );

  return ChartInfo {
    chart_box   : draw_axes(&canvas, &display_cfg)
  , series_data : vec!(
      SeriesData {
        name        : SERIES1
      , plot_points : draw_splines(&canvas, &absorber_info.no_air_gap, &no_air_gap_series.plot_colour, &display_cfg.smooth_curve)
      }
    , SeriesData {
        name        : SERIES2
      , plot_points : draw_splines(&canvas, &absorber_info.abs_against_panel,   &abs_against_panel_series.plot_colour, &display_cfg.smooth_curve)
      }
    , SeriesData {
        name        : SERIES3
      , plot_points : draw_splines(&canvas, &absorber_info.abs_against_backing, &abs_against_backing_series.plot_colour, &display_cfg.smooth_curve)
      })
  };
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
fn draw_title_and_key(
  canvas      : &web_sys::HtmlCanvasElement
, title       : &str
, title_font  : &FontMetadata
, key_font    : &FontMetadata
, series_list : Vec<&SeriesMetadata>
) {
  const FN_NAME : &str = &"draw_title_and_key";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  const TITLE_KEY_GAP     : f64 = 50.0;
  const KEY_SYMBOL_LENGTH : f64 = 30.0;
  const SYMBOL_TEXT_GAP   : f64 = 10.0;

  let ctx = get_2d_context(&canvas);
  ctx.save();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Set font and stroke colour, then measure title width
  ctx.set_font(&title_font.font());
  ctx.set_stroke_style(title_font.stroke_style);
  let title_width = ctx.measure_text(title).unwrap().width();

  // Add chart title
  ctx.fill_text(&title, HORIZ_MARGIN_INSET, VERT_MARGIN_INSET).unwrap();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Key spacing
  ctx.set_font(&key_font.font());
  ctx.set_stroke_style(key_font.stroke_style);

  // It is assumed that there will be no more than 6 series to plot on the same graph
  let (mut key_rows, mut key_columns) : (usize, usize) = match series_list.len() {
    1 => (1, 1)
  , 2 => (1, 2)
  , 3 => (1, 3)
  , 4 => (2, 2)
  , 5 => (2, 3)
  , 6 => (2, 3)
  , _ => (0, 0)
  };

  // Find the length of the longest key text
  let longest_key_text = series_list
    .iter()
    .fold(
      0.0
    , | acc, s | {
        let text_length = ctx.measure_text(s.name).unwrap().width();
        if text_length > acc { text_length } else { acc }
      }
    );

  // Calculate the required and available space
  let key_entry_width     = KEY_SYMBOL_LENGTH + (3.0 * SYMBOL_TEXT_GAP) + longest_key_text;
  let available_key_width = canvas.width() as f64 - title_width - (2.0 * HORIZ_MARGIN_INSET) - TITLE_KEY_GAP;

  let mut required_key_width = key_entry_width * key_columns as f64;

  trace(&format!("key_entry_width     = {}", key_entry_width));
  trace(&format!("available_key_width = {}", available_key_width));
  trace(&format!("required_key_width  = {}", required_key_width));

  if required_key_width > available_key_width {
    key_columns        -= 1;
    key_rows            = (series_list.len() as f64 / key_columns as f64).ceil() as usize;
    required_key_width  = key_entry_width * key_columns as f64;
  }

  trace(&format!("Key table contains {} columns and {} rows", key_columns, key_rows));

  let start_x = canvas.width() as f64 - HORIZ_MARGIN_INSET - required_key_width;

  let mut x = start_x;
  let mut y = VERT_MARGIN_INSET - (title_font.font_size / 2.0);

  for row_idx in 0..key_rows {
    for col_idx in 0..key_columns {
      let series_idx = row_idx * key_columns + col_idx;

      if series_idx < series_list.len() {
        trace(&format!("row_idx = {}, col_idx = {}, series_idx = {}", row_idx, col_idx, series_idx));
        trace(&format!("Drawing key symbol at {},{}", x + (KEY_SYMBOL_LENGTH / 2.0), y));

        draw_key_symbol(&ctx, &PlotPoint {x: x, y: y}, &series_list[series_idx].plot_colour, &KEY_SYMBOL_LENGTH);

        // Draw key text
        ctx.fill_text(series_list[series_idx].name, x + 40.0, y + (key_font.font_size / 2.0) - 3.0).unwrap();

        x += key_entry_width;
      }
    }

    // Reset x coordinate back to the start of the row and move the y axis down one row
    x = start_x;
    y += key_font.font_size + 4.0;
  }

  ctx.restore();

  trace_boundary(&Some(false));
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw graph axes
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn draw_axes(canvas: &web_sys::HtmlCanvasElement, display_cfg: &DisplayConfig) -> ChartBox {
  const FN_NAME : &str = &"draw_axes";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let chart_origin = &PlotPoint { x : LEFT_AXIS_INSET, y : canvas.height() as f64 - BOTTOM_AXIS_INSET };

  let label_font = &FontMetadata {
    typeface     : &BASE_TYPEFACE
  , font_size    : LABEL_FONT_SIZE
  , stroke_style : &JsValue::from(RGB_BLACK)
  };

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Draw Y axis
  trace(&"Drawing Y axis");

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

  let y_axis_end_point = PlotPoint { x : LEFT_AXIS_INSET, y : BOTTOM_AXIS_INSET };

  draw_axis(&canvas, &Axis {
    title          : &"Absorption"
  , start_point    : &chart_origin
  , end_point      : &y_axis_end_point
  , values         : abs_strs
  , orientation    : AxisOrientation::Vertical
  , label_font     : label_font.clone()
  , tick_length    : TICK_LENGTH
  , tick_label_gap : TICK_LABEL_GAP
  });

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Draw X axis
  trace(&"Drawing X axis");

  let freq_strs : Vec<String> = display_cfg
    .frequencies
    .iter()
    .fold(
      vec!()
    , | mut acc, f | {
        acc.push(if f == &62.5 {
          String::from("62.5")
        }
        else {
          String::from(format!("{}",f.round() as u32))
        });

        return acc;
      }
    );

  let x_axis_end_point = PlotPoint { x : canvas.width() as f64 - LEFT_AXIS_INSET, y : canvas.height() as f64 - BOTTOM_AXIS_INSET };

  draw_axis(&canvas, &Axis {
    title          : &"Frequency (Hz)"
  , start_point    : &chart_origin
  , end_point      : &x_axis_end_point
  , values         : freq_strs
  , orientation    : AxisOrientation::Horizontal
  , label_font     : label_font.clone()
  , tick_length    : TICK_LENGTH
  , tick_label_gap : TICK_LABEL_GAP
  });

  trace_boundary(&Some(false));

  return ChartBox {
    top_left     : y_axis_end_point
  , bottom_right : x_axis_end_point
  }
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw an axis
fn draw_axis(canvas: &web_sys::HtmlCanvasElement, axis_info: &Axis) {
  const FN_NAME : &str = &"draw_axis";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));
  trace(&"Plotting axis");

  let ctx = get_2d_context(&canvas);

  // Define context values
  let (mid_height, mid_width, bottom_margin_pos, _, _) = canvas_dimensions(&canvas);
  let tick_interval : f64 = axis_info.tick_interval();

  ctx.save();
  ctx.set_font(&axis_info.label_font.font());
  ctx.set_stroke_style(&axis_info.label_font.stroke_style);
  
  let axis_label_width = ctx.measure_text(axis_info.title).unwrap().width();

  // Draw the axis line
  ctx.begin_path();
  ctx.move_to(axis_info.start_point.x, axis_info.start_point.y);
  ctx.line_to(axis_info.end_point.x, axis_info.end_point.y);

  // Relocate origin to axis start point
  ctx.translate(axis_info.start_point.x, axis_info.start_point.y).unwrap();

  // For a horizontal axis, the tick labels must be rotated 90° anti-clockwise
  match axis_info.orientation {
    AxisOrientation::Horizontal => ctx.rotate(-PI_OVER_TWO).unwrap()
  , AxisOrientation::Vertical   => ()
  }

  // Draw axis ticks and labels
  for val in axis_info.values.iter() {
    let tick_label  = &format!("{}", val);
    let label_width = ctx.measure_text(tick_label).unwrap().width();

    // Position the label away from the tick by the tick length plus a gap
    let label_offset = label_width + axis_info.tick_length + axis_info.tick_label_gap;

    // Draw tick
    ctx.move_to(-axis_info.tick_length, 0.0);
    ctx.line_to(0.0, 0.0);

    // Add label text then move origin to next tick location
    match axis_info.orientation {
      AxisOrientation::Vertical   => {
        ctx.fill_text(tick_label, -label_offset, axis_info.tick_label_gap).unwrap();
        ctx.translate(0.0, -tick_interval).unwrap();
      }

    , AxisOrientation::Horizontal => {
        ctx.fill_text(tick_label, -label_offset, axis_info.label_font.font_size / 2.0).unwrap();
        ctx.translate(0.0, tick_interval).unwrap();
      }
    }
  }

  ctx.stroke();

  // Reposition origin and set rotation based on axis orientation
  match axis_info.orientation {
    AxisOrientation::Horizontal => {
      ctx.rotate(PI_OVER_TWO).unwrap();
      ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
      ctx.translate(mid_width - (axis_label_width / 2.0), bottom_margin_pos).unwrap()
    }

  , AxisOrientation::Vertical => {
      ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
      ctx.translate(HORIZ_MARGIN_INSET, mid_height + (axis_label_width / 2.0)).unwrap();
      ctx.rotate(-PI_OVER_TWO).unwrap();
    }
  }

  // Write axis title and restore context state
  ctx.fill_text(axis_info.title, 0.0, 0.0).unwrap();
  ctx.restore();
  trace_boundary(&Some(false));
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
// A simple, but non-inuitive way to clear the entire canvas... :-)
fn clear(canvas: &web_sys::HtmlCanvasElement) {
  canvas.set_width(canvas.width());
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Return the distance between two points
fn distance(pt1_x: f64, pt1_y: f64, pt2_x: f64, pt2_y: f64, ) -> f64 {
  sqrt(pow(pt1_x - pt2_x, 2.0) + pow(pt1_y - pt2_y, 2.0))
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Generate the two control points that lie between the three supplied plot points
fn gen_control_points(pt1: &PlotAbsPoint, pt2: &PlotAbsPoint, pt3: &PlotAbsPoint, tension : f64) -> Vec<PlotPoint> {
  // Vector from start point to finish point
  // This is used to determine the gradient of the lines through the control points
  let x_vec = pt3.x - pt1.x;
  let y_vec = pt3.y - pt1.y;

  let d01  = distance(pt1.x, pt1.y, pt2.x, pt2.y);
  let d12  = distance(pt2.x, pt2.y, pt3.x, pt3.y);
  let d012 = d01 + d12;

  // Return the coordinates of the two control points between the three current points
  return vec![
    PlotPoint { x : pt2.x - x_vec * tension * d01 / d012, y : pt2.y - y_vec * tension * d01 / d012 }
  , PlotPoint { x : pt2.x + x_vec * tension * d12 / d012, y : pt2.y + y_vec * tension * d12 / d012 }
  ]
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw a control point
fn draw_control_points(ctx : &web_sys::CanvasRenderingContext2d, cps : &Vec<PlotPoint>) {
  for i in 0..(cps.len() / 2) {
    let idx = 2 * i;
    draw_point(ctx, &cps[idx].x,     &cps[idx].y,     &JsValue::from(RGB_LIGHT_PINK));
    draw_point(ctx, &cps[idx + 1].x, &cps[idx + 1].y, &JsValue::from(RGB_LIGHT_PINK));

    draw_line(ctx, &cps[idx], &cps[idx + 1], &JsValue::from(RGB_LIGHT_PINK));
  }
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw key symbol
fn draw_key_symbol(ctx : &web_sys::CanvasRenderingContext2d, location : &PlotPoint, colour : &JsValue, symbol_length : &f64) {
  draw_line(ctx, location, &PlotPoint {x: location.x + symbol_length, y: location.y}, colour);
  draw_point(ctx, &(location.x + (symbol_length / 2.0)), &location.y, colour);
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw a straight line
fn draw_line(ctx : &web_sys::CanvasRenderingContext2d, start : &PlotPoint, end : &PlotPoint, stroke_style: &JsValue) {
  ctx.begin_path();
  ctx.move_to(start.x, start.y);
  ctx.line_to(end.x,   end.y);

  ctx.save();
  ctx.set_stroke_style(stroke_style);
  ctx.stroke();
  ctx.restore();
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw a plot point
fn draw_point(
  ctx        : &web_sys::CanvasRenderingContext2d
, x          : &f64
, y          : &f64
, fill_style : &JsValue
) {
  ctx.begin_path();
  ctx.save();
  ctx.set_fill_style(fill_style);
  ctx.arc(*x, *y, PLOT_POINT_RADIUS, 0.0, TWO_PI).unwrap();
  ctx.fill();
  ctx.restore();
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw curve splines
fn draw_splines(
  canvas        : &web_sys::HtmlCanvasElement
, abs_points    : &Vec<PlotPoint>
, stroke_colour : &JsValue
, smooth_curve  : &bool
) -> Vec<PlotAbsPoint> {
  const FN_NAME : &str = &"draw_splines";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let (_, _, _, x_axis_length, y_axis_length) = canvas_dimensions(&canvas);

  let ctx = get_2d_context(&canvas);

  let x_tick_interval = x_axis_length / (abs_points.len() - 1) as f64;
  let y_pos           = scaled_y_pos(canvas.height() as f64 - BOTTOM_AXIS_INSET, y_axis_length);

  // The frequency and absorption values need to be translated into canvas coordinates
  let mut points : Vec<PlotAbsPoint> = vec!();

  for idx in 0..abs_points.len() {
    let plot_abs_point = PlotAbsPoint {
      x    : LEFT_AXIS_INSET + x_tick_interval * idx as f64
    , y    : y_pos(abs_points[idx].y)
    , freq : abs_points[idx].x
    , abs  : abs_points[idx].y
    };
    trace(&format!("PlotPoint(x: {}, y: {}, freq: {}, abs: {})", plot_abs_point.x, plot_abs_point.y, plot_abs_point.freq, plot_abs_point.abs));
    points.push(plot_abs_point);
  }

  // Between each triplet of plot points, there will be two invisible control points
  // The smooth curve between plot points can be removed simply by setting the tension to zero
  let mut cps: Vec<PlotPoint> = vec!();
  let tension : f64 = if *smooth_curve { 0.45 } else { 0.0 };
  
  for idx in 0..points.len() - 2 {
    cps.append(&mut gen_control_points(&points[idx], &points[idx + 1], &points[idx + 2], tension));
  }

  if TRACE_ACTIVE == &true {
    trace(&format!("Control points"));
    for cp in cps.iter() {
      trace(&format!("({},{})", cp.x, cp.y));
    }
  }

  // Draw all the plot points
  trace(&format!("Drawing points"));
  for idx in 0..points.len() {
    draw_point(&ctx, &points[idx].x, &points[idx].y, &stroke_colour)
  }

  // If tracing is switched on, also draw the control points
  trace(&format!("Drawing control points"));
  if TRACE_ACTIVE == &true {
    draw_control_points(&ctx, &cps);
  }

  trace(&format!("Drawing curve"));
  draw_curved_path(&ctx, &cps, &points, &stroke_colour);

  trace_boundary(&Some(false));
  return points;
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Draw a smooth curve between the plot points
fn draw_curved_path(
  ctx: &web_sys::CanvasRenderingContext2d
, cps: &Vec<PlotPoint>
, points: &Vec<PlotAbsPoint>
, stroke_style: &JsValue
) {
  // As long as we have at least two points...
  if points.len() >= 2 {
    ctx.save();
    ctx.set_stroke_style(&stroke_style);

    // First point
    ctx.begin_path();
    ctx.move_to(points[0].x, points[0].y);

    // Are there only 2 points?
    if points.len() == 2 {
      // Yup, so draw a straight line to the last point and we're done
      ctx.line_to(points[1].x, points[1].y);
    }
    else {
      // For 3 or more points...
      // Plot points 0 and 1 are connected with a quadratic Bezier that requires a single control point
      ctx.quadratic_curve_to(cps[0].x, cps[0].y, points[1].x, points[1].y);

      // All middle plot points are connected with a cubic Bezier that requires a pair of control points
      for i in 2..points.len() - 1 {
        let cp_idx1 = (i - 2) * 2 + 1;
        let cp_idx2 = (i - 1) * 2;

        ctx.bezier_curve_to(cps[cp_idx1].x, cps[cp_idx1].y, cps[cp_idx2].x, cps[cp_idx2].y, points[i].x, points[i].y);
      }

      // Last two plot points are connected with a quadratic Bezier that requires a single control point
      ctx.quadratic_curve_to(cps[cps.len() - 1].x, cps[cps.len() - 1].y, points[points.len() - 1].x, points[points.len() - 1].y);
    }

    // Draw the curve
    ctx.stroke();
    ctx.restore();
  }
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Scale the Y axis (absorption value) to a canvas pixel location
fn scaled_y_pos(start: f64, axis_length: f64) ->
  impl Fn(f64) -> f64 {
    move | this_y: f64 | start - (this_y * axis_length)
  }
