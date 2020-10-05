/***********************************************************************************************************************
 * Porous Absorber Calculator - Chart Renderer
 *
 * (c) Chris Whealy 2020
 */
pub mod constants;

mod bezier;
mod canvas_utils;
pub mod draw;

use wasm_bindgen::{JsCast, JsValue};

use crate::chart::{constants::*, render::constants::*};
use crate::config::{
  chart::{ChartConfig, ChartInfo, SeriesData},
  generic_device::{DeviceType, GenericDeviceInfo},
};

/***********************************************************************************************************************
 * Trace functionality
 */
use crate::{
  config::trace_flags::trace_flag_for,
  trace::function_boundaries::{make_boundary_trace_fn, TraceAction},
};

pub const MOD_NAME: &str = "chart::render";

/***********************************************************************************************************************
 * Plot a chart for a generic device
 */
pub fn generic_device<'a>(
  device_info: GenericDeviceInfo<'a>,
  chart_cfg: &ChartConfig,
  chart_title: &str,
) -> ChartInfo<'a> {
  const FN_NAME: &str = "generic_device";

  let trace_boundary = make_boundary_trace_fn(trace_flag_for(MOD_NAME), MOD_NAME.to_string(), FN_NAME.to_string());

  trace_boundary(TraceAction::Enter);

  let document = web_sys::window().unwrap().document().unwrap();
  let canvas_el = document.get_element_by_id(GRAPH_CANVAS_ID).unwrap();
  let canvas = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

  let y_axis_inset: f64 = if chart_cfg.show_diagram {
    Y_AXIS_INSET_DIAGRAM
  } else {
    Y_AXIS_INSET_NO_DIAGRAM
  };

  let (_, _, _, x_axis_length, y_axis_length) = canvas_utils::canvas_dimensions(&canvas, &y_axis_inset);

  canvas_utils::clear(&canvas);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Each device type has one or more plot series, each defined by a relevant metadata object
  let series_metadata = match device_info.device_type {
    DeviceType::RigidBackedPorousAbsorber => vec![&METADATA_NO_AIR_GAP, &METADATA_AIR_GAP],
    DeviceType::PerforatedPanelAbsorber => vec![&METADATA_NO_AIR_GAP, &METADATA_ABS_PANEL, &METADATA_ABS_BACKING],
    DeviceType::SlottedPanelAbsorber => vec![&METADATA_NO_AIR_GAP, &METADATA_ABS_PANEL, &METADATA_ABS_BACKING],
    DeviceType::MicroperforatedPanelAbsorber => vec![&METADATA_MP_PANEL],
  };

  let (chart_box, widest_y_tick_label) = draw::axes(&canvas, &chart_cfg, &y_axis_inset);

  draw::title_and_key(
    &canvas,
    chart_title,
    &FONT_METADATA_TITLE,
    &FONT_METADATA_LABEL,
    series_metadata,
  );

  // Draw the device diagram if necessary
  if chart_cfg.show_diagram {
    draw::device_diagram(&device_info, widest_y_tick_label, &y_axis_length, &Y_AXIS_INSET_DIAGRAM);
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Define series data
  //
  // The order of plot point information in the device_info.abs_series vector must match the order of data generated by
  // the calculate::<device_type> functions in the calc_engine modules

  let series_data = match device_info.device_type {
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Plot series for rigid backed porous absorber
    DeviceType::RigidBackedPorousAbsorber => vec![
      SeriesData {
        name: METADATA_AIR_GAP.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[0].plot_points.to_vec(),
          &JsValue::from(METADATA_AIR_GAP.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
      SeriesData {
        name: METADATA_NO_AIR_GAP.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[1].plot_points.to_vec(),
          &JsValue::from(METADATA_NO_AIR_GAP.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
    ],

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Plot series for perforated panel absorber
    DeviceType::PerforatedPanelAbsorber => vec![
      SeriesData {
        name: METADATA_AIR_GAP.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[0].plot_points.to_vec(),
          &JsValue::from(METADATA_NO_AIR_GAP.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
      SeriesData {
        name: METADATA_ABS_PANEL.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[1].plot_points.to_vec(),
          &JsValue::from(METADATA_ABS_PANEL.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
      SeriesData {
        name: METADATA_ABS_BACKING.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[2].plot_points.to_vec(),
          &JsValue::from(METADATA_ABS_BACKING.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
    ],

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Plot series for slotted panel absorber
    DeviceType::SlottedPanelAbsorber => vec![
      SeriesData {
        name: METADATA_AIR_GAP.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[0].plot_points.to_vec(),
          &JsValue::from(METADATA_NO_AIR_GAP.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
      SeriesData {
        name: METADATA_ABS_PANEL.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[1].plot_points.to_vec(),
          &JsValue::from(METADATA_ABS_PANEL.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
      SeriesData {
        name: METADATA_ABS_BACKING.name,
        plot_points: draw::splines(
          &canvas,
          device_info.abs_series[2].plot_points.to_vec(),
          &JsValue::from(METADATA_ABS_BACKING.plot_colour),
          &chart_cfg.smooth_curve,
          &x_axis_length,
          &y_axis_length,
          &y_axis_inset,
        ),
      },
    ],

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Plot series for microperforated panel absorber
    DeviceType::MicroperforatedPanelAbsorber => vec![SeriesData {
      name: METADATA_MP_PANEL.name,
      plot_points: draw::splines(
        &canvas,
        device_info.abs_series[0].plot_points.to_vec(),
        &JsValue::from(METADATA_MP_PANEL.plot_colour),
        &chart_cfg.smooth_curve,
        &x_axis_length,
        &y_axis_length,
        &y_axis_inset,
      ),
    }],
  };

  trace_boundary(TraceAction::Exit);
  ChartInfo { chart_box, series_data }
}
