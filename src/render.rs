// *********************************************************************************************************************
// Porous Absorber Calculator - Chart Renderer
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::{JsValue, JsCast};
use std::f64::consts::PI;
use libm::{sqrt, pow};

use crate::structs::generic_device::{DeviceType, GenericDeviceInfo};
use crate::structs::config_display::*;


// *********************************************************************************************************************
// Trace functionality
// *********************************************************************************************************************
use crate::trace::Trace;

const LIB_NAME     : &str  = &"render";
const TRACE_ACTIVE : &bool = &false;

// *********************************************************************************************************************
// Canvas constants
// *********************************************************************************************************************
const PI_OVER_TWO : f64 = PI / 2.0;

// These names must correspond to the element ids used in index.html
const GRAPH_CANVAS_ID : &str = "graph_canvas";
const WALL_IMG_ID     : &str = "wall_img";
const ABSORBER_IMG_ID : &str = "absorber_img";
const PANEL_IMG_ID    : &str = "panel_img";

const WALL_IMG_WIDTH : f64 = 20.0;

const Y_AXIS_INSET_DIAGRAM    : f64 = 300.0; // Distance of Y axis from left edge if diagram is displayed
const Y_AXIS_INSET_NO_DIAGRAM : f64 = 100.0; // Distance of Y axis from left edge if diagram is not displayed
const X_AXIS_INSET            : f64 = 100.0; // Distance of X axis from bottom edge

const LEFT_MARGIN_INSET   : f64 = 35.0;
const RIGHT_MARGIN_INSET  : f64 = 50.0;
const TOP_MARGIN_INSET    : f64 = 50.0;
const BOTTOM_MARGIN_INSET : f64 = 17.5;

const RGB_BLACK           : &str = &"rgb(0, 0, 0)";
const RGB_PINK            : &str = &"rgb(234, 51, 247)";
const RGB_LIGHT_PINK      : &str = &"rgb(246, 195, 203)";
const RGB_DARK_BLUE       : &str = &"rgb(6, 1, 123)";
const RGB_GREEN           : &str = &"rgb(20, 255, 20)";
const RGB_OFF_WHITE       : &str = &"rgb(255, 255, 238)";

const BASE_TYPEFACE       : &str = &"Arial";
const TITLE_FONT_SIZE     : f64  = 36.0;
const LABEL_FONT_SIZE     : f64  = 20.0;

const TICK_LENGTH         : f64 = 10.0;
const TICK_LABEL_GAP      : f64 = 5.0;
const PLOT_POINT_RADIUS   : f64 = 5.0;

// Scale factor for magnifying the holes in a microperforated panel
const MP_SCALE_FACTOR : f64 = 20.0;

// *********************************************************************************************************************
// Text constants
// *********************************************************************************************************************
const TXT_AIR_GAP     : &str = &"Air Gap";
const TXT_NO_AIR_GAP  : &str = &"No Air Gap";
const TXT_ABS_PANEL   : &str = &"Absorber Against Panel";
const TXT_ABS_BACKING : &str = &"Absorber Against Backing";
const TXT_MP_PANEL    : &str = &"Microperforated Panel";

const TXT_Y_AXIS_TITLE : &str = &"Absorption";
const TXT_X_AXIS_TITLE : &str = &"Frequency (Hz)";

// *********************************************************************************************************************
// Chart series and font metadata
// *********************************************************************************************************************
const METADATA_AIR_GAP     : SeriesMetadata = SeriesMetadata { name : TXT_AIR_GAP,     plot_colour : RGB_PINK };
const METADATA_NO_AIR_GAP  : SeriesMetadata = SeriesMetadata { name : TXT_NO_AIR_GAP,  plot_colour : RGB_GREEN };
const METADATA_ABS_PANEL   : SeriesMetadata = SeriesMetadata { name : TXT_ABS_PANEL,   plot_colour : RGB_DARK_BLUE };
const METADATA_ABS_BACKING : SeriesMetadata = SeriesMetadata { name : TXT_ABS_BACKING, plot_colour : RGB_PINK };
const METADATA_MP_PANEL    : SeriesMetadata = SeriesMetadata { name : TXT_MP_PANEL,    plot_colour : RGB_DARK_BLUE };

const FONT_METADATA_TITLE : FontMetadata = FontMetadata {
  typeface     : &BASE_TYPEFACE
, font_size    : TITLE_FONT_SIZE
, stroke_style : RGB_BLACK
};
const FONT_METADATA_LABEL : FontMetadata = FontMetadata {
  typeface     : &BASE_TYPEFACE
, font_size    : LABEL_FONT_SIZE
, stroke_style : RGB_BLACK
};


// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************


// *********************************************************************************************************************
// Generic device calculator
// *********************************************************************************************************************
pub fn plot_generic_device<'a> (
  device_info : GenericDeviceInfo<'a>
, display_cfg : &DisplayConfig
, chart_title : &str
) -> ChartInfo<'a> {
  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id(GRAPH_CANVAS_ID).unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let y_axis_inset : f64 = if display_cfg.show_diagram { Y_AXIS_INSET_DIAGRAM } else { Y_AXIS_INSET_NO_DIAGRAM };
  
  let (_, _, _, x_axis_length, y_axis_length) = canvas_dimensions( &canvas, &y_axis_inset);
  
  clear(&canvas);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Define metadata for series name and plot colour
  let series_metadata = match device_info.device_type {
    DeviceType::RigidBackedPorousAbsorber    => vec!(&METADATA_NO_AIR_GAP, &METADATA_AIR_GAP)
  , DeviceType::PerforatedPanelAbsorber      => vec!(&METADATA_NO_AIR_GAP, &METADATA_ABS_PANEL, &METADATA_ABS_BACKING)
  , DeviceType::SlottedPanelAbsorber         => vec!(&METADATA_NO_AIR_GAP, &METADATA_ABS_PANEL, &METADATA_ABS_BACKING)
  , DeviceType::MicroperforatedPanelAbsorber => vec!(&METADATA_MP_PANEL)
  };

  let (chart_box, widest_y_tick_label) = draw_axes(&canvas, &display_cfg, &y_axis_inset);

  draw_title_and_key(&canvas, chart_title, &FONT_METADATA_TITLE, &FONT_METADATA_LABEL, series_metadata);

  // Draw the device diagram if necessary
  if display_cfg.show_diagram {
    draw_device_diagram(&device_info, widest_y_tick_label, &y_axis_length, &Y_AXIS_INSET_DIAGRAM);
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Define series data
  //
  // The order of plot point information in the device_info.abs_series vector must match the order of data generated by
  // the calculate_<device_type> functions in the calc_engine module

  let series_data = match device_info.device_type {
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Rigid backed porous absorber
    DeviceType::RigidBackedPorousAbsorber => vec!(
      SeriesData {
        name        : METADATA_AIR_GAP.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[0].plot_points.to_vec()
                      , &JsValue::from(METADATA_AIR_GAP.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      }
    , SeriesData {
        name        : METADATA_NO_AIR_GAP.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[1].plot_points.to_vec()
                      , &JsValue::from(METADATA_NO_AIR_GAP.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      })

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Perforated panel absorber
  , DeviceType::PerforatedPanelAbsorber => vec!(
      SeriesData {
        name        : METADATA_AIR_GAP.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[0].plot_points.to_vec()
                      , &JsValue::from(METADATA_NO_AIR_GAP.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      }
    , SeriesData {
        name        : METADATA_ABS_PANEL.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[1].plot_points.to_vec()
                      , &JsValue::from(METADATA_ABS_PANEL.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      }
    , SeriesData {
        name        : METADATA_ABS_BACKING.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[2].plot_points.to_vec()
                      , &JsValue::from(METADATA_ABS_BACKING.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                     )
      })

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Slotted panel absorber
  , DeviceType::SlottedPanelAbsorber => vec!(
      SeriesData {
        name        : METADATA_AIR_GAP.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[0].plot_points.to_vec()
                      , &JsValue::from(METADATA_NO_AIR_GAP.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      }
    , SeriesData {
        name        : METADATA_ABS_PANEL.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[1].plot_points.to_vec()
                      , &JsValue::from(METADATA_ABS_PANEL.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      }
    , SeriesData {
        name        : METADATA_ABS_BACKING.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[2].plot_points.to_vec()
                      , &JsValue::from(METADATA_ABS_BACKING.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      })

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Microperforated panel absorber
  , DeviceType::MicroperforatedPanelAbsorber => vec!(
      SeriesData {
        name        : METADATA_MP_PANEL.name
      , plot_points : draw_splines(
                        &canvas
                      , device_info.abs_series[0].plot_points.to_vec()
                      , &JsValue::from(METADATA_MP_PANEL.plot_colour)
                      , &display_cfg.smooth_curve
                      , &x_axis_length
                      , &y_axis_length
                      , &y_axis_inset
                      )
      })
  };

  return ChartInfo {
    chart_box   : chart_box
  , series_data : series_data
  };
}


// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                 P R I V A T E   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************

fn get_2d_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
  canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap()
}

fn fetch_image(document : &web_sys::Document, img_name : &str) -> web_sys::HtmlImageElement {
  document.get_element_by_id(img_name).unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap()
}


// *********************************************************************************************************************
// Draw chart title and key
// *********************************************************************************************************************
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
  ctx.set_stroke_style(&JsValue::from(title_font.stroke_style));
  let title_width = ctx.measure_text(title).unwrap().width();

  // Add chart title
  ctx.fill_text(&title, LEFT_MARGIN_INSET, TOP_MARGIN_INSET).unwrap();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Key spacing
  ctx.set_font(&key_font.font());
  ctx.set_stroke_style(&JsValue::from(key_font.stroke_style));

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
  let available_key_width = canvas.width() as f64 - title_width - LEFT_MARGIN_INSET - RIGHT_MARGIN_INSET - TITLE_KEY_GAP;

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

  let start_x = canvas.width() as f64 - RIGHT_MARGIN_INSET - required_key_width;

  let mut x = start_x;
  let mut y = TOP_MARGIN_INSET - (title_font.font_size / 2.0);

  for row_idx in 0..key_rows {
    for col_idx in 0..key_columns {
      let series_idx = row_idx * key_columns + col_idx;

      if series_idx < series_list.len() {
        trace(&format!("row_idx = {}, col_idx = {}, series_idx = {}", row_idx, col_idx, series_idx));
        trace(&format!("Drawing key symbol at {},{}", x + (KEY_SYMBOL_LENGTH / 2.0), y));

        draw_key_symbol(&ctx, &PlotPoint {x: x, y: y}, &JsValue::from(series_list[series_idx].plot_colour), &KEY_SYMBOL_LENGTH);

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


// *********************************************************************************************************************
// Draw graph axes
// This function returns a tuple containing firstly, the chart box dimensions within which the cross-hairs are drawn and
// secondly, the width of the widest Y axis tick label
//
// The widest tick label value is needed as part of the calculation to determine the available width within which to
// draw the device diagram
// *********************************************************************************************************************
fn draw_axes(canvas: &web_sys::HtmlCanvasElement, display_cfg: &DisplayConfig, y_axis_inset : &f64) -> (ChartBox, f64) {
  const FN_NAME : &str = &"draw_axes";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let chart_origin = &PlotPoint {
    x : *y_axis_inset
  , y : canvas.height() as f64 - (2.0 * TOP_MARGIN_INSET)
  };

  let label_font = &FontMetadata {
    typeface     : &BASE_TYPEFACE
  , font_size    : LABEL_FONT_SIZE
  , stroke_style : &RGB_BLACK
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

  let y_axis_end_point = PlotPoint { x : *y_axis_inset, y : X_AXIS_INSET };

  let widest_tick_label = draw_axis(&canvas, &Axis {
    title       : TXT_Y_AXIS_TITLE
  , start_point : &chart_origin
  , end_point   : &y_axis_end_point
  , values      : abs_strs
  , orientation : AxisOrientation::Vertical
  , label_font  : label_font.clone()
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
        let y_txt = if f == &62.5 { String::from("62.5") } else { String::from(format!("{}",f.round() as u32)) };
        acc.push(y_txt);
        return acc;
      }
    );

  let x_axis_end_point = PlotPoint { x : canvas.width()  as f64 - RIGHT_MARGIN_INSET
                                   , y : canvas.height() as f64 - X_AXIS_INSET };

  draw_axis(&canvas, &Axis {
    title       : TXT_X_AXIS_TITLE
  , start_point : &chart_origin
  , end_point   : &x_axis_end_point
  , values      : freq_strs
  , orientation : AxisOrientation::Horizontal
  , label_font  : label_font.clone()
  });

  trace_boundary(&Some(false));

  return (
    ChartBox {
      top_left     : y_axis_end_point
    , bottom_right : x_axis_end_point
    }
  , widest_tick_label
  )
}


// *********************************************************************************************************************
// Draw a single axis
// *********************************************************************************************************************
fn draw_axis(canvas: &web_sys::HtmlCanvasElement, axis_info: &Axis) -> f64 {
  const FN_NAME : &str = &"draw_axis";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let ctx = get_2d_context(&canvas);

  // Define context values
  let (mid_height, mid_width, bottom_margin_pos, _, _) = canvas_dimensions(&canvas, &axis_info.start_point.x);
  let tick_interval : f64 = axis_info.tick_interval();

  ctx.save();
  ctx.set_font(&axis_info.label_font.font());
  ctx.set_stroke_style(&JsValue::from(axis_info.label_font.stroke_style));
  
  let axis_label_width = ctx.measure_text(axis_info.title).unwrap().width();

  // Draw the axis line
  trace(&format!( "Plotting axis from ({},{}) to ({},{})"
                , axis_info.start_point.x
                , axis_info.start_point.y
                , axis_info.end_point.x
                , axis_info.end_point.y
                ));
  ctx.begin_path();
  ctx.move_to(axis_info.start_point.x, axis_info.start_point.y);
  ctx.line_to(axis_info.end_point.x,   axis_info.end_point.y);

  // Relocate origin to axis start point
  ctx.translate(axis_info.start_point.x, axis_info.start_point.y).unwrap();

  // For a horizontal axis, the tick labels must be rotated 90° anti-clockwise
  match axis_info.orientation {
    AxisOrientation::Horizontal => ctx.rotate(-PI_OVER_TWO).unwrap()
  , AxisOrientation::Vertical   => ()
  }

  let mut widest_tick_label : f64 = 0.0;
  let mut tick_label_width  : f64 = 0.0;

  // Draw axis ticks and labels
  for val in axis_info.values.iter() {
    let tick_label  = &format!("{}", val);

    tick_label_width  = ctx.measure_text(tick_label).unwrap().width();
    widest_tick_label = max(widest_tick_label, tick_label_width);

    // Position the label away from the tick by the tick length plus a gap
    let label_offset = tick_label_width + TICK_LENGTH + TICK_LABEL_GAP;

    // Draw tick
    ctx.move_to(-TICK_LENGTH, 0.0);
    ctx.line_to(0.0, 0.0);

    // Add label text then move origin to next tick location
    match axis_info.orientation {
      AxisOrientation::Vertical => {
        ctx.fill_text(tick_label, -label_offset, TICK_LABEL_GAP).unwrap();
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
      ctx.translate(
        y_axis_name_x_pos(tick_label_width, &axis_info.start_point.x)
      , mid_height + (axis_label_width / 2.0)
      ).unwrap();
      ctx.rotate(-PI_OVER_TWO).unwrap();
    }
  }

  // Write axis title and restore context state
  ctx.fill_text(axis_info.title, 0.0, 0.0).unwrap();
  ctx.restore();
  trace_boundary(&Some(false));

  return widest_tick_label;
}


// *********************************************************************************************************************
// Draw the device diagram
// *********************************************************************************************************************
fn draw_device_diagram(device: &GenericDeviceInfo, widest_y_tick_label: f64, y_axis_length : &f64, y_axis_inset : &f64) {
  const FN_NAME : &str  = &"draw_device_diagram";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let document  = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id(GRAPH_CANVAS_ID).unwrap();
  let canvas    = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
  let ctx       = get_2d_context(&canvas);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Calculate the overall depth of the device in mm
  // One millimetre of device depth will be rendered as one pixel on the canvas up until the point that the device depth
  // exceeds the number of pixels.  After this, the images will be scaled down to fit the available space
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  let air_gap_mm = device.cavity.unwrap().air_gap_mm as f64;

  let absorber_thickness_mm = match device.porous_layer {
      Some(p) => p.thickness_mm as f64
    , None    => 0.0
    };

  let (panel_thickness_mm, between_voids_mm, void_mm) = match device.device_type {
      DeviceType::SlottedPanelAbsorber => {
        let panel = device.sl_panel.unwrap();
        (panel.thickness_mm, panel.slot_distance_mm, panel.slot_width_mm)
        }
    , DeviceType::PerforatedPanelAbsorber => {
        let panel = device.pf_panel.unwrap();
        (panel.thickness_mm, panel.hole_centres_mm - (2.0 * panel.hole_radius_mm), 2.0 * panel.hole_radius_mm)
      }
    , DeviceType::MicroperforatedPanelAbsorber => {
        let panel = device.mp_panel.unwrap();
        (panel.thickness_mm, panel.hole_centres_mm - (2.0 * panel.hole_radius_mm), 2.0 * panel.hole_radius_mm)
      }
    , DeviceType::RigidBackedPorousAbsorber => {
        (0.0, 0.0, 0.0)
      }
    };

  let dev_depth_mm = air_gap_mm + absorber_thickness_mm + panel_thickness_mm;

  // Calculate the amount of space available for the diagram
  let available_pxls = y_axis_name_x_pos(widest_y_tick_label, y_axis_inset)
                       - LEFT_MARGIN_INSET
                       - WALL_IMG_WIDTH
                       - LABEL_FONT_SIZE;
  
  let horiz_pixels_per_mm = if dev_depth_mm > available_pxls { available_pxls / dev_depth_mm } else { 1.0 };

  trace(&format!("Overall device depth = {} mm", dev_depth_mm));
  trace(&format!("Available space for diagram = {} px", available_pxls));
  trace(&format!("Pixels per mm = {}", horiz_pixels_per_mm));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Fetch image elements from the DOM
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  let wall_img     = fetch_image(&document, WALL_IMG_ID);
  let panel_img    = fetch_image(&document, PANEL_IMG_ID);
  let absorber_img = fetch_image(&document, ABSORBER_IMG_ID);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Draw fixed wall image
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  let wall_pos_x = LEFT_MARGIN_INSET - WALL_IMG_WIDTH;
  let wall_pos_y = X_AXIS_INSET;

  trace(&format!("Drawing wall at location ({},{})", wall_pos_x, wall_pos_y));

  draw_image(&ctx, &wall_img, wall_pos_x, wall_pos_y, WALL_IMG_WIDTH, *y_axis_length);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Draw an optional absorber layer - this layer is absent for the microperforated panel device
  // Firefox crashes if you attempt to draw a zero-width image, but Chrome and Brave are fine with this
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  let half_height = *y_axis_length / 2.0;

  let abs_pos_x    = LEFT_MARGIN_INSET + (air_gap_mm * horiz_pixels_per_mm);
  let abs_pos_y    = X_AXIS_INSET;
  let abs_width_px = absorber_thickness_mm * horiz_pixels_per_mm;

  // Do we need to draw an absorber?
  if absorber_thickness_mm > 0.0 {
    // Yup
    trace(&format!("Drawing absorber at location ({},{})", abs_pos_x, abs_pos_y));

    // Do we also need to draw a panel?
    if panel_thickness_mm > 0.0 {
      // Yup, so draw a half height absorber against the panel
      draw_partial_image(
        &ctx, &absorber_img
      , 0.0, 0.0
      , abs_width_px, half_height
      , abs_pos_x,    abs_pos_y
      , abs_width_px, half_height
      );

      // Then below, draw another half height absorber against the backing
      draw_partial_image(
        &ctx, &absorber_img
      , 0.0, 0.0
      , abs_width_px,      half_height
      , LEFT_MARGIN_INSET, abs_pos_y + half_height
      , abs_width_px,      half_height
      );
    }
    else {
      // Nope, so draw a full height absorber
      draw_partial_image(
        &ctx, &absorber_img
      , 0.0, 0.0
      , abs_width_px, *y_axis_length
      , abs_pos_x,    abs_pos_y
      , abs_width_px, *y_axis_length
      );
    }
  }
  else {
    // Nope, no absorbent layer here...
    trace(&"Not drawing absorber - zero thickness");
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Draw an optional panel - this layer is absent for the rigid backed porous absorber device
  // Firefox crashes if you attempt to draw a zero-width image, but Chrome and Brave are fine with this
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  let panel_width_px = panel_thickness_mm * horiz_pixels_per_mm;
  let panel_pos_x    = abs_pos_x + abs_width_px;
  let panel_pos_y    = X_AXIS_INSET;

  // Do we need to draw a panel?
  if panel_thickness_mm > 0.0 {
    // Yup...
    trace(&format!("Drawing panel at location ({},{})", panel_pos_x, panel_pos_y));

    draw_partial_image(
      &ctx, &panel_img
    , 0.0, 0.0
    , panel_width_px, *y_axis_length
    , panel_pos_x,    panel_pos_y
    , panel_width_px, *y_axis_length
    );

    // On the microperforated panel, the hole diameter is so small that without the use of a scale factor to magnify
    // them, the holes would be almost invisible
    let scale_factor = match device.device_type {
      DeviceType::MicroperforatedPanelAbsorber => MP_SCALE_FACTOR
    , _                                        => 1.0
    };

    // "void" represents the size of either the hole or the slot in the panel
    let     bg_colour            = JsValue::from(RGB_OFF_WHITE);
    let     scaled_void          = scale_factor * void_mm;
    let     scaled_between_voids = scale_factor * between_voids_mm;
    let     interval             = scaled_between_voids + scaled_void;
    let mut gap_pos              = panel_pos_y + scaled_between_voids;

    trace(&format!("Voids centred every = {} mm", interval));

    // Draw background-coloured blocks over the panel to indicate the position and width of the voids
    while gap_pos < (panel_pos_y + *y_axis_length) {
      draw_box(&ctx, &panel_pos_x, &gap_pos, &panel_width_px, &scaled_void, &bg_colour);
      gap_pos += interval;
    }
  }
  else {
    // Nope, no panels here...
    trace(&"Not drawing panel - zero thickness");
  }

  trace_boundary(&Some(false));
}

// *********************************************************************************************************************
// Return a tuple of various frequently used canvas dimensions
// *********************************************************************************************************************
fn canvas_dimensions(canvas: &web_sys::HtmlCanvasElement, y_axis_inset : &f64) -> (f64, f64, f64, f64, f64) {
  let h = canvas.height() as f64;
  let w = canvas.width() as f64;

  ( h / 2.0                                // vertical midpoint
  , w / 2.0                                // horizontal midpoint
  , h - BOTTOM_MARGIN_INSET                // bottom margin position
  , w - y_axis_inset - RIGHT_MARGIN_INSET  // X axis length
  , h - (2.0 * X_AXIS_INSET)               // Y axis length
  )
}


// *********************************************************************************************************************
// A simple, but non-inuitive way to clear the entire canvas...
// :-)
// *********************************************************************************************************************
fn clear(canvas: &web_sys::HtmlCanvasElement) {
  canvas.set_width(canvas.width());
}


// *********************************************************************************************************************
// Return the distance between two points
// *********************************************************************************************************************
fn distance(pt1: &PlotAbsPoint, pt2: &PlotAbsPoint) -> f64 {
  sqrt(pow(pt1.x - pt2.x, 2.0) + pow(pt1.y - pt2.y, 2.0))
}


// *********************************************************************************************************************
// Generate two Bézier control points that lie between the three supplied plot points
// The tension parameter indicates the degree of curvature between the three points.
// Setting the tension to zero results in straight lines
// *********************************************************************************************************************
fn gen_control_points(pt1: &PlotAbsPoint, pt2: &PlotAbsPoint, pt3: &PlotAbsPoint, tension : f64) -> Vec<PlotPoint> {
  // Calculate the length of the two line segments
  let seg_1_len   = distance(pt1, pt2);
  let seg_2_len   = distance(pt2, pt3);
  let total_len   = seg_1_len + seg_2_len;
  let seg_1_ratio = seg_1_len / total_len;
  let seg_2_ratio = seg_2_len / total_len;

  // Calculate the gradient between the start and finish points.  The control points then live on a line that has this
  // gradient and passes through the middle point
  let x_vec = pt3.x - pt1.x;
  let y_vec = pt3.y - pt1.y;

  // Return the coordinates of the two control points between the three current points
  return vec![
    PlotPoint { x : pt2.x - x_vec * tension * seg_1_ratio, y : pt2.y - y_vec * tension * seg_1_ratio }
  , PlotPoint { x : pt2.x + x_vec * tension * seg_2_ratio, y : pt2.y + y_vec * tension * seg_2_ratio }
  ];

}


// *********************************************************************************************************************
// Draw the control points
// This function is only called if the TRACE_ACTIVE flag is switched on
// *********************************************************************************************************************
fn draw_control_points(ctx : &web_sys::CanvasRenderingContext2d, cps : &Vec<PlotPoint>) {
  for i in 0..(cps.len() / 2) {
    let idx = 2 * i;
    draw_point(ctx, &cps[idx].x,     &cps[idx].y,     &JsValue::from(RGB_LIGHT_PINK));
    draw_point(ctx, &cps[idx + 1].x, &cps[idx + 1].y, &JsValue::from(RGB_LIGHT_PINK));

    draw_line(ctx, &cps[idx], &cps[idx + 1], &JsValue::from(RGB_LIGHT_PINK));
  }
}


// *********************************************************************************************************************
// Draw a key symbol
// *********************************************************************************************************************
fn draw_key_symbol(ctx : &web_sys::CanvasRenderingContext2d, location : &PlotPoint, colour : &JsValue, symbol_length : &f64) {
  draw_line(ctx, location, &PlotPoint {x: location.x + symbol_length, y: location.y}, colour);
  draw_point(ctx, &(location.x + (symbol_length / 2.0)), &location.y, colour);
}


// *********************************************************************************************************************
// Draw a straight line
// *********************************************************************************************************************
fn draw_line(ctx : &web_sys::CanvasRenderingContext2d, start : &PlotPoint, end : &PlotPoint, stroke_style: &JsValue) {
  ctx.begin_path();
  ctx.move_to(start.x, start.y);
  ctx.line_to(end.x,   end.y);

  ctx.save();
  ctx.set_stroke_style(stroke_style);
  ctx.stroke();
  ctx.restore();
}


// *********************************************************************************************************************
// Draw a rectangular box
// *********************************************************************************************************************
fn draw_box(
  ctx        : &web_sys::CanvasRenderingContext2d
, x          : &f64
, y          : &f64
, width      : &f64
, height     : &f64
, fill_style : &JsValue
) {
  ctx.begin_path();
  ctx.save();
  ctx.set_fill_style(fill_style);
  ctx.fill_rect(*x, *y, *width, *height);
  ctx.restore();
}


// *********************************************************************************************************************
// Draw a circular plot point
// *********************************************************************************************************************
fn draw_point(
  ctx        : &web_sys::CanvasRenderingContext2d
, x          : &f64
, y          : &f64
, fill_style : &JsValue
) {
  ctx.begin_path();
  ctx.save();
  ctx.set_fill_style(fill_style);
  ctx.arc(*x, *y, PLOT_POINT_RADIUS, 0.0, 2.0 * PI).unwrap();
  ctx.fill();
  ctx.restore();
}


// *********************************************************************************************************************
// Draw curve splines
// *********************************************************************************************************************
fn draw_splines<'a>(
      canvas        : &web_sys::HtmlCanvasElement
, mut abs_points    : Vec<PlotAbsPoint>
,     stroke_colour : &JsValue
,     smooth_curve  : &bool
,     x_axis_length : &f64
,     y_axis_length : &f64
,     y_axis_inset  : &f64
) -> Vec<PlotAbsPoint> {
  const FN_NAME : &str = &"draw_splines";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let ctx = get_2d_context(&canvas);

  let x_tick_interval = x_axis_length / (abs_points.len() - 1) as f64;
  let y_pos           = scaled_y_pos(canvas.height() as f64 - X_AXIS_INSET, *y_axis_length);

  // The frequency and absorption values need to be translated into canvas coordinates
  for idx in 0..abs_points.len() {
    abs_points[idx].x = y_axis_inset + x_tick_interval * idx as f64;
    abs_points[idx].y = y_pos(abs_points[idx].abs);

    trace(&format!("PlotPoint(x: {}, y: {}, freq: {}, abs: {})", abs_points[idx].x, abs_points[idx].y, abs_points[idx].freq, abs_points[idx].abs));
  }

  // Between each triplet of plot points, there will be two invisible control points
  // The smooth curve between plot points can be removed simply by setting the tension to zero
  let mut cps: Vec<PlotPoint> = vec!();
  let tension : f64 = if *smooth_curve { 0.45 } else { 0.0 };
  
  for idx in 0..abs_points.len() - 2 {
    cps.append(&mut gen_control_points(&abs_points[idx], &abs_points[idx + 1], &abs_points[idx + 2], tension));
  }

  if TRACE_ACTIVE == &true {
    trace(&format!("Control points"));
    for cp in cps.iter() {
      trace(&format!("({},{})", cp.x, cp.y));
    }
  }

  // Draw all the plot points
  trace(&format!("Drawing points"));
  for idx in 0..abs_points.len() {
    draw_point(&ctx, &abs_points[idx].x, &abs_points[idx].y, &stroke_colour)
  }

  // If tracing is switched on, also draw the control points
  trace(&format!("Drawing control points"));
  if TRACE_ACTIVE == &true {
    draw_control_points(&ctx, &cps);
  }

  trace(&format!("Drawing curve"));
  draw_curved_path(&ctx, &cps, &abs_points, &stroke_colour);

  trace_boundary(&Some(false));
  return abs_points;
}


// *********************************************************************************************************************
// Draw a smooth curve between the plot points
// *********************************************************************************************************************
fn draw_curved_path(
  ctx          : &web_sys::CanvasRenderingContext2d
, cps          : &Vec<PlotPoint>
, points       : &Vec<PlotAbsPoint>
, stroke_style : &JsValue
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


// *********************************************************************************************************************
// Translate an absorption value ranging from 0.00 .. 0.99 to a canvas pixel location
// *********************************************************************************************************************
fn scaled_y_pos(start: f64, axis_length: f64) ->
  impl Fn(f64) -> f64 {
    move | this_y: f64 | start - (this_y * axis_length)
  }

// *********************************************************************************************************************
// X coordinate of Y axis name
// *********************************************************************************************************************
fn y_axis_name_x_pos(tick_label_width: f64, y_axis_inset : &f64) -> f64 {
  y_axis_inset - TICK_LENGTH - (2.0 * TICK_LABEL_GAP) - tick_label_width - LABEL_FONT_SIZE
}

// *********************************************************************************************************************
// Max is not implemented for f64s...
// *********************************************************************************************************************
fn max(a: f64, b: f64) -> f64 {
  if a < b { b } else { a }
}

// *********************************************************************************************************************
// Draw an image at the specified location
// *********************************************************************************************************************
fn draw_image(
  ctx : &web_sys::CanvasRenderingContext2d
, img : &web_sys::HtmlImageElement
, top_left_x : f64, top_left_y : f64, width : f64, height : f64
) {
  draw_partial_image(ctx, img, 0.0, 0.0, *&img.width() as f64, *&img.height() as f64, top_left_x, top_left_y, width, height);
}

// *********************************************************************************************************************
// Draw some subsection of an image at the specified location
// *********************************************************************************************************************
fn draw_partial_image(
  ctx : &web_sys::CanvasRenderingContext2d
, img : &web_sys::HtmlImageElement
, sub_top_left_x : f64, sub_top_left_y : f64, sub_width : f64, sub_height : f64
,     top_left_x : f64,     top_left_y : f64,     width : f64,     height : f64
) {
  ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
    img
  , sub_top_left_x, sub_top_left_y, sub_width, sub_height
  ,     top_left_x,     top_left_y,     width,     height
  ).unwrap();
}