/***********************************************************************************************************************
 * Porous Absorber Calculator - Draw absorption chart within an HTML canvas
 *
 * (c) Chris Whealy 2020
 */
use wasm_bindgen::{JsCast, JsValue};

use crate::{
    chart::{
        render,
        render::{bezier, canvas_utils::*},
    },
    config::{
        chart::*,
        trace_flags::trace_flag_for,
    },
    trace::*,
    utils::maths_functions::TAU,
};
use crate::devices::generic_device::{DeviceType, GenericDeviceInfo};

pub const MOD_NAME: &str = "chart::render::draw";

/***********************************************************************************************************************
 * Define a subdivision of an image
 */
struct ImageSubdiv {
    sub_top_left: PlotPoint,
    sub_dims: DimensionPair,
    top_left: PlotPoint,
    dims: DimensionPair,
}

/***********************************************************************************************************************
 * Draw the device diagram
 */
const QUARTER_TURN: f64 = TAU / 4.0;

pub fn device_diagram(device: &GenericDeviceInfo, widest_y_tick_label: f64, y_axis_length: &f64, y_axis_inset: &f64) {
    const FN_NAME: &str = "device_diagram";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace_boundary = make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    trace_boundary(TraceAction::Enter);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_el = document.get_element_by_id(render::constants::GRAPH_CANVAS_ID).unwrap();
    let canvas = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let ctx = get_2d_context(&canvas);

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Calculate the overall depth of the device in mm
    // One millimetre of device depth will be rendered as one pixel on the canvas up until the point that the device
    // depth exceeds the number of pixels.  After this, the images will be scaled down to fit the available space
    let air_gap_mm = device.cavity.air_gap_mm as f64;

    let absorber_thickness_mm = match device.porous_layer {
        Some(p) => p.thickness_mm as f64,
        None => 0.0,
    };

    let (panel_thickness_mm, between_voids_mm, void_mm) = match device.device_type {
        DeviceType::SlottedPanelAbsorber => {
            let panel = device.sl_panel.unwrap();
            (panel.thickness_mm, panel.slot_distance_mm, panel.slot_width_mm)
        },
        DeviceType::PerforatedPanelAbsorber => {
            let panel = device.pf_panel.unwrap();
            (
                panel.thickness_mm,
                panel.hole_centres_mm - (2.0 * panel.hole_radius_mm),
                2.0 * panel.hole_radius_mm,
            )
        },
        DeviceType::MicroperforatedPanelAbsorber => {
            let panel = device.mp_panel.unwrap();
            (
                panel.thickness_mm,
                panel.hole_centres_mm - (2.0 * panel.hole_radius_mm),
                2.0 * panel.hole_radius_mm,
            )
        },
        DeviceType::RigidBackedPorousAbsorber => (0.0, 0.0, 0.0),
    };

    let dev_depth_mm = air_gap_mm + absorber_thickness_mm + panel_thickness_mm;

    // Calculate the amount of space available for the diagram
    let available_pxls = y_axis_name_x_pos(widest_y_tick_label, y_axis_inset)
        - render::constants::LEFT_MARGIN_INSET
        - render::constants::WALL_IMG_WIDTH
        - crate::chart::constants::LABEL_FONT_SIZE;

    let horiz_pixels_per_mm = if dev_depth_mm > available_pxls { available_pxls / dev_depth_mm } else { 1.0 };

    trace(format!("Overall device depth = {} mm", dev_depth_mm));
    trace(format!("Available space for diagram = {} px", available_pxls));
    trace(format!("Pixels per mm = {}", horiz_pixels_per_mm));

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Fetch image elements from the DOM
    let wall_img = fetch_image(&document, render::constants::WALL_IMG_ID);
    let panel_img = fetch_image(&document, render::constants::PANEL_IMG_ID);
    let absorber_img = fetch_image(&document, render::constants::ABSORBER_IMG_ID);

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Draw fixed wall image
    let wall_pos_x = render::constants::LEFT_MARGIN_INSET - render::constants::WALL_IMG_WIDTH;
    let wall_pos_y = render::constants::X_AXIS_INSET;

    trace(format!("Drawing wall at location ({},{})", wall_pos_x, wall_pos_y));

    draw_image(
        &ctx,
        &wall_img,
        PlotPoint { x: wall_pos_x, y: wall_pos_y },
        DimensionPair {
            width: render::constants::WALL_IMG_WIDTH,
            height: *y_axis_length,
        },
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Draw an optional absorber layer - this layer is absent for the microperforated panel device
    // Firefox crashes if you attempt to draw a zero-width image, but Chrome and Brave are fine with this
    let half_height = *y_axis_length / 2.0;

    let abs_pos_x = render::constants::LEFT_MARGIN_INSET + (air_gap_mm * horiz_pixels_per_mm);
    let abs_pos_y = render::constants::X_AXIS_INSET;
    let abs_width_px = absorber_thickness_mm * horiz_pixels_per_mm;

    // Do we need to draw an absorber?
    if absorber_thickness_mm > 0.0 {
        // Yup
        trace(format!("Drawing absorber at location ({},{})", abs_pos_x, abs_pos_y));

        // Do we also need to draw a panel?
        if panel_thickness_mm > 0.0 {
            // Yup, so draw a half height absorber against the panel
            draw_partial_image(
                &ctx,
                &absorber_img,
                ImageSubdiv {
                    sub_top_left: render::constants::ORIGIN,
                    sub_dims: DimensionPair {
                        width: abs_width_px,
                        height: half_height,
                    },
                    top_left: PlotPoint { x: abs_pos_x, y: abs_pos_y },
                    dims: DimensionPair {
                        width: abs_width_px,
                        height: half_height,
                    },
                },
            );

            // Then below, draw another half height absorber against the backing
            draw_partial_image(
                &ctx,
                &absorber_img,
                ImageSubdiv {
                    sub_top_left: render::constants::ORIGIN,
                    sub_dims: DimensionPair {
                        width: abs_width_px,
                        height: half_height,
                    },
                    top_left: PlotPoint {
                        x: render::constants::LEFT_MARGIN_INSET,
                        y: abs_pos_y + half_height,
                    },
                    dims: DimensionPair {
                        width: abs_width_px,
                        height: half_height,
                    },
                },
            );
        } else {
            // Nope, so draw a full height absorber
            draw_partial_image(
                &ctx,
                &absorber_img,
                ImageSubdiv {
                    sub_top_left: render::constants::ORIGIN,
                    sub_dims: DimensionPair {
                        width: abs_width_px,
                        height: *y_axis_length,
                    },
                    top_left: PlotPoint { x: abs_pos_x, y: abs_pos_y },
                    dims: DimensionPair {
                        width: abs_width_px,
                        height: *y_axis_length,
                    },
                },
            );
        }
    } else {
        // Nope, no absorbent layer here...
        trace("Not drawing absorber - zero thickness".to_string());
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Draw an optional panel - this layer is absent for the rigid backed porous absorber device
    // Firefox crashes if you attempt to draw a zero-width image, but Chrome and Brave are fine with this
    let panel_width_px = panel_thickness_mm * horiz_pixels_per_mm;
    let panel_pos_x = abs_pos_x + abs_width_px;
    let panel_pos_y = render::constants::X_AXIS_INSET;

    // Do we need to draw a panel?
    if panel_thickness_mm > 0.0 {
        // Yup...
        trace(format!("Drawing panel at location ({},{})", panel_pos_x, panel_pos_y));

        draw_partial_image(
            &ctx,
            &panel_img,
            ImageSubdiv {
                sub_top_left: render::constants::ORIGIN,
                sub_dims: DimensionPair {
                    width: panel_width_px,
                    height: *y_axis_length,
                },
                top_left: PlotPoint { x: panel_pos_x, y: panel_pos_y },
                dims: DimensionPair {
                    width: panel_width_px,
                    height: *y_axis_length,
                },
            },
        );

        // On the microperforated panel, the holes are so small that without the use of a scale factor to magnify them,
        // they would be almost invisible
        let scale_factor = match device.device_type {
            DeviceType::MicroperforatedPanelAbsorber => render::constants::MP_SCALE_FACTOR,
            _ => 1.0,
        };

        // "void" represents the size of either the hole or the slot in the panel
        let bg_colour = JsValue::from(crate::chart::constants::RGB_OFF_WHITE);
        let scaled_void = scale_factor * void_mm;
        let scaled_between_voids = scale_factor * between_voids_mm;
        let interval = scaled_between_voids + scaled_void;
        let mut gap_pos = panel_pos_y + scaled_between_voids;

        trace(format!("Voids centred every = {} mm", interval));

        // Draw background-coloured blocks over the panel to indicate the position and width of the voids
        while gap_pos < (panel_pos_y + *y_axis_length) {
            draw_box(&ctx, &panel_pos_x, &gap_pos, &panel_width_px, &scaled_void, &bg_colour);
            gap_pos += interval;
        }
    } else {
        // Nope, no panels here...
        trace("Not drawing panel - zero thickness".to_string());
    }

    trace_boundary(TraceAction::Exit);
}

/***********************************************************************************************************************
 * Draw chart title and key
 */
pub fn title_and_key(
    canvas: &web_sys::HtmlCanvasElement,
    title: &str,
    title_font: &FontMetadata,
    key_font: &FontMetadata,
    series_list: Vec<&SeriesMetadata>,
) {
    const FN_NAME: &str = "title_and_key";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace_boundary = make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    trace_boundary(TraceAction::Enter);

    let ctx = get_2d_context(&canvas);
    ctx.save();

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Set font and stroke colour, then measure title width
    ctx.set_font(&title_font.font());
    ctx.set_stroke_style(&JsValue::from(title_font.stroke_style));
    let title_width = ctx.measure_text(title).unwrap().width();

    // Add chart title
    ctx.fill_text(
        &title,
        render::constants::LEFT_MARGIN_INSET,
        render::constants::TOP_MARGIN_INSET,
    )
    .unwrap();

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Key spacing
    ctx.set_font(&key_font.font());
    ctx.set_stroke_style(&JsValue::from(key_font.stroke_style));

    // It is assumed that there will be no more than 6 series to plot on the same graph
    let (mut key_rows, mut key_columns): (usize, usize) = match series_list.len() {
        1 => (1, 1),
        2 => (1, 2),
        3 => (1, 3),
        4 => (2, 2),
        5 => (2, 3),
        6 => (2, 3),
        _ => (0, 0),
    };

    // Find the length of the longest key text
    let longest_key_text = series_list
        .iter()
        .fold(0.0, |acc: f64, s| acc.max(ctx.measure_text(s.name).unwrap().width()));

    // Calculate the required and available space
    let key_entry_width =
        render::constants::KEY_SYMBOL_LENGTH + (3.0 * render::constants::SYMBOL_TEXT_GAP) + longest_key_text;
    let available_key_width = canvas.width() as f64
        - title_width
        - render::constants::LEFT_MARGIN_INSET
        - render::constants::RIGHT_MARGIN_INSET
        - render::constants::TITLE_KEY_GAP;

    let mut required_key_width = key_entry_width * key_columns as f64;

    trace(format!("key_entry_width     = {}", key_entry_width));
    trace(format!("available_key_width = {}", available_key_width));
    trace(format!("required_key_width  = {}", required_key_width));

    if required_key_width > available_key_width {
        key_columns -= 1;
        key_rows = (series_list.len() as f64 / key_columns as f64).ceil() as usize;
        required_key_width = key_entry_width * key_columns as f64;
    }

    trace(format!("Key table contains {} columns and {} rows", key_columns, key_rows));

    let start_x = canvas.width() as f64 - render::constants::RIGHT_MARGIN_INSET - required_key_width;

    let mut x = start_x;
    let mut y = render::constants::TOP_MARGIN_INSET - (title_font.font_size / 2.0);

    for row_idx in 0..key_rows {
        for col_idx in 0..key_columns {
            let series_idx = row_idx * key_columns + col_idx;

            if series_idx < series_list.len() {
                trace(format!(
                    "row_idx = {}, col_idx = {}, series_idx = {}",
                    row_idx, col_idx, series_idx
                ));
                trace(format!(
                    "Drawing key symbol at {},{}",
                    x + (render::constants::KEY_SYMBOL_LENGTH / 2.0),
                    y
                ));

                draw_key_symbol(
                    &ctx,
                    &PlotPoint { x, y },
                    &JsValue::from(series_list[series_idx].plot_colour),
                    &render::constants::KEY_SYMBOL_LENGTH,
                );

                // Draw key text
                ctx.fill_text(series_list[series_idx].name, x + 40.0, y + (key_font.font_size / 2.0) - 3.0)
                    .unwrap();

                x += key_entry_width;
            }
        }

        // Reset x coordinate back to the start of the row and move the y axis down one row
        x = start_x;
        y += key_font.font_size + 4.0;
    }

    ctx.restore();

    trace_boundary(TraceAction::Exit);
}

/***********************************************************************************************************************
 * Draw graph axes
 *
 * This function returns a tuple containing firstly the chart box dimensions within which the cross-hairs are drawn and
 * secondly, the width of the widest Y axis tick label
 *
 * The widest tick label value is needed as part of the calculation to determine the available width within which to
 * draw the device diagram
 */
pub fn axes(canvas: &web_sys::HtmlCanvasElement, chart_cfg: &ChartConfig, y_axis_inset: &f64) -> (ChartBox, f64) {
    const FN_NAME: &str = "axes";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace_boundary = make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    trace_boundary(TraceAction::Enter);

    let chart_origin = &PlotPoint {
        x: *y_axis_inset,
        y: canvas.height() as f64 - (2.0 * render::constants::TOP_MARGIN_INSET),
    };

    let label_font = &FontMetadata {
        typeface: &crate::chart::constants::BASE_TYPEFACE,
        font_size: crate::chart::constants::LABEL_FONT_SIZE,
        stroke_style: &crate::chart::constants::RGB_BLACK,
    };

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Draw Y axis
    trace("Drawing Y axis".to_string());

    let abs_strs = vec![
        "0.0".to_string(),
        "0.1".to_string(),
        "0.2".to_string(),
        "0.3".to_string(),
        "0.4".to_string(),
        "0.5".to_string(),
        "0.6".to_string(),
        "0.7".to_string(),
        "0.8".to_string(),
        "0.9".to_string(),
        "1.0".to_string(),
    ];

    let y_axis_end_point = PlotPoint {
        x: *y_axis_inset,
        y: render::constants::X_AXIS_INSET,
    };

    let widest_tick_label = draw_axis(
        &canvas,
        Axis {
            title: crate::chart::constants::TXT_Y_AXIS_TITLE,
            start_point: &chart_origin,
            end_point: &y_axis_end_point,
            values: abs_strs,
            orientation: AxisOrientation::Vertical,
            label_font,
        },
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Draw X axis
    trace("Drawing X axis".to_string());

    let freq_strs: Vec<String> = chart_cfg.frequencies.iter().fold(vec![], |mut acc, f| {
        acc.push(if (*f - 62.5).abs() < f64::EPSILON {
            "62.5".to_string()
        } else {
            format!("{}", f.round() as u32)
        });
        acc
    });

    let x_axis_end_point = PlotPoint {
        x: canvas.width() as f64 - render::constants::RIGHT_MARGIN_INSET,
        y: canvas.height() as f64 - render::constants::X_AXIS_INSET,
    };

    draw_axis(
        &canvas,
        Axis {
            title: crate::chart::constants::TXT_X_AXIS_TITLE,
            start_point: &chart_origin,
            end_point: &x_axis_end_point,
            values: freq_strs,
            orientation: AxisOrientation::Horizontal,
            label_font,
        },
    );

    trace_boundary(TraceAction::Exit);

    (
        ChartBox {
            top_left: y_axis_end_point,
            bottom_right: x_axis_end_point,
        },
        widest_tick_label,
    )
}

/***********************************************************************************************************************
 * Draw curve splines
 */
pub fn splines(
    canvas: &web_sys::HtmlCanvasElement,
    mut abs_points: Vec<PlotAbsPoint>,
    stroke_colour: &JsValue,
    smooth_curve: &bool,
    x_axis_length: &f64,
    y_axis_length: &f64,
    y_axis_inset: &f64,
) -> Vec<PlotAbsPoint> {
    const FN_NAME: &str = "splines";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace_boundary = make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    trace_boundary(TraceAction::Enter);

    let ctx = get_2d_context(&canvas);

    let x_tick_interval = x_axis_length / (abs_points.len() - 1) as f64;
    let y_pos = scaled_y_pos(canvas.height() as f64 - render::constants::X_AXIS_INSET, *y_axis_length);

    // The frequency and absorption values need to be translated into canvas coordinates
    for (idx, abs_point) in abs_points.iter_mut().enumerate() {
        abs_point.at.x = y_axis_inset + x_tick_interval * idx as f64;
        abs_point.at.y = y_pos(abs_point.abs);

        trace(format!(
            "PlotPoint(At: {}, freq: {}, abs: {})",
            abs_point.at, abs_point.freq, abs_point.abs
        ));
    }

    // Between each triplet of plot points, there will be two invisible control points
    // The smooth curve between plot points can be removed simply by setting the tension to zero
    let mut cps: Vec<PlotPoint> = vec![];
    let tension: f64 = if *smooth_curve { 0.45 } else { 0.0 };

    for idx in 0..abs_points.len() - 2 {
        cps.append(&mut bezier::gen_control_points(
            &abs_points[idx],
            &abs_points[idx + 1],
            &abs_points[idx + 2],
            tension,
        ));
    }

    if trace_active {
        trace("Control points".to_string());
        for cp in cps.iter() {
            trace(format!("{}", cp));
        }
    }

    // Draw all the plot points
    trace("Drawing points".to_string());
    for abs_point in &abs_points {
        draw_point(&ctx, &abs_point.at, &stroke_colour)
    }

    // If tracing is switched on, also draw the control points
    trace("Drawing control points".to_string());
    if trace_active {
        draw_control_points(&ctx, &cps);
    }

    trace("Drawing curve".to_string());
    draw_curved_path(&ctx, &cps, &abs_points, &stroke_colour);

    trace_boundary(TraceAction::Exit);
    abs_points
}

//**********************************************************************************************************************
// Private API
//**********************************************************************************************************************

/***********************************************************************************************************************
 * Draw a single axis
 */
fn draw_axis(canvas: &web_sys::HtmlCanvasElement, axis_info: Axis) -> f64 {
    const FN_NAME: &str = "draw_axis";
    let trace_active = trace_flag_for(MOD_NAME);
    let trace_boundary = make_boundary_trace_fn(trace_active, MOD_NAME, FN_NAME);
    let trace = make_trace_fn(trace_active, MOD_NAME, FN_NAME);

    trace_boundary(TraceAction::Enter);

    let ctx = get_2d_context(&canvas);

    // Define context values
    let (mid_height, mid_width, bottom_margin_pos, _, _) = canvas_dimensions(&canvas, &axis_info.start_point.x);
    let tick_interval: f64 = axis_info.tick_interval();

    ctx.save();
    ctx.set_font(&axis_info.label_font.font());
    ctx.set_stroke_style(&JsValue::from(axis_info.label_font.stroke_style));

    let axis_label_width = ctx.measure_text(axis_info.title).unwrap().width();

    // Draw the axis line
    trace(format!(
        "Plotting axis from {} to {}",
        axis_info.start_point, axis_info.end_point
    ));
    ctx.begin_path();
    ctx.move_to(axis_info.start_point.x, axis_info.start_point.y);
    ctx.line_to(axis_info.end_point.x, axis_info.end_point.y);

    // Relocate origin to axis start point
    ctx.translate(axis_info.start_point.x, axis_info.start_point.y).unwrap();

    // For a horizontal axis, the tick labels must be rotated 90° anti-clockwise
    match axis_info.orientation {
        AxisOrientation::Horizontal => ctx.rotate(-QUARTER_TURN).unwrap(),
        AxisOrientation::Vertical => (),
    }

    let mut widest_tick_label: f64 = 0.0;
    let mut tick_label_width: f64 = 0.0;

    // Draw axis ticks and labels
    for val in axis_info.values.iter() {
        let tick_label = val.to_string();

        tick_label_width = ctx.measure_text(&tick_label).unwrap().width();
        widest_tick_label = widest_tick_label.max(tick_label_width);

        // Position the label away from the tick by the tick length plus a gap
        let label_offset = tick_label_width + render::constants::TICK_LENGTH + render::constants::TICK_LABEL_GAP;

        // Draw tick
        ctx.move_to(-render::constants::TICK_LENGTH, 0.0);
        ctx.line_to(render::constants::ORIGIN.x, render::constants::ORIGIN.y);

        // Add label text then move origin to next tick location
        match axis_info.orientation {
            AxisOrientation::Vertical => {
                ctx.fill_text(&tick_label, -label_offset, render::constants::TICK_LABEL_GAP)
                    .unwrap();
                ctx.translate(0.0, -tick_interval).unwrap();
            },

            AxisOrientation::Horizontal => {
                ctx.fill_text(&tick_label, -label_offset, axis_info.label_font.font_size / 2.0)
                    .unwrap();
                ctx.translate(0.0, tick_interval).unwrap();
            },
        }
    }

    ctx.stroke();

    // Reposition origin and set rotation based on axis orientation
    match axis_info.orientation {
        AxisOrientation::Horizontal => {
            ctx.rotate(QUARTER_TURN).unwrap();
            ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
            ctx.translate(mid_width - (axis_label_width / 2.0), bottom_margin_pos).unwrap()
        },

        AxisOrientation::Vertical => {
            ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
            ctx.translate(
                y_axis_name_x_pos(tick_label_width, &axis_info.start_point.x),
                mid_height + (axis_label_width / 2.0),
            )
            .unwrap();
            ctx.rotate(-QUARTER_TURN).unwrap();
        },
    }

    // Write axis title and restore context state
    ctx.fill_text(axis_info.title, 0.0, 0.0).unwrap();
    ctx.restore();
    trace_boundary(TraceAction::Exit);

    widest_tick_label
}
/***********************************************************************************************************************
 * Draw a key symbol
 */
fn draw_key_symbol(
    ctx: &web_sys::CanvasRenderingContext2d,
    location: &PlotPoint,
    colour: &JsValue,
    symbol_length: &f64,
) {
    draw_line(
        ctx,
        location,
        &PlotPoint {
            x: location.x + symbol_length,
            y: location.y,
        },
        colour,
    );
    draw_point(
        ctx,
        &PlotPoint {
            x: location.x + (symbol_length / 2.0),
            y: location.y,
        },
        colour,
    );
}

/***********************************************************************************************************************
 * Draw a straight line
 */
fn draw_line(ctx: &web_sys::CanvasRenderingContext2d, start: &PlotPoint, end: &PlotPoint, stroke_style: &JsValue) {
    ctx.begin_path();
    ctx.move_to(start.x, start.y);
    ctx.line_to(end.x, end.y);

    ctx.save();
    ctx.set_stroke_style(stroke_style);
    ctx.stroke();
    ctx.restore();
}

/***********************************************************************************************************************
 * Draw a box
 */
fn draw_box(
    ctx: &web_sys::CanvasRenderingContext2d,
    x: &f64,
    y: &f64,
    width: &f64,
    height: &f64,
    fill_style: &JsValue,
) {
    ctx.begin_path();
    ctx.save();
    ctx.set_fill_style(fill_style);
    ctx.fill_rect(*x, *y, *width, *height);
    ctx.restore();
}

/***********************************************************************************************************************
 * Draw a circular plot point
 */
fn draw_point(ctx: &web_sys::CanvasRenderingContext2d, point: &PlotPoint, fill_style: &JsValue) {
    ctx.begin_path();
    ctx.save();

    // Draw filled circle
    ctx.set_fill_style(fill_style);
    ctx.arc(point.x, point.y, render::constants::PLOT_POINT_RADIUS, 0.0, TAU)
        .unwrap();
    ctx.fill();

    // Draw black edge
    ctx.set_line_width(0.5);
    ctx.set_stroke_style(&JsValue::from(crate::chart::constants::RGB_BLACK));
    ctx.stroke();

    ctx.restore();
}

/***********************************************************************************************************************
 * Draw a smooth curve between the plot points
 */
fn draw_curved_path(
    ctx: &web_sys::CanvasRenderingContext2d,
    cps: &[PlotPoint],
    points: &[PlotAbsPoint],
    stroke_style: &JsValue,
) {
    // As long as we have at least two points...
    if points.len() >= 2 {
        ctx.save();
        ctx.set_stroke_style(&stroke_style);

        // First point
        ctx.begin_path();
        ctx.move_to(points[0].at.x, points[0].at.y);

        // Are there only 2 points?
        if points.len() == 2 {
            // Yup, so draw a straight line to the last point and we're done
            ctx.line_to(points[1].at.x, points[1].at.y);
        } else {
            // For 3 or more points...
            // Plot points 0 and 1 are connected with a quadratic Bezier that requires a single control point
            ctx.quadratic_curve_to(cps[0].x, cps[0].y, points[1].at.x, points[1].at.y);

            // All middle plot points are connected with a cubic Bezier that requires a pair of control points
            for (i, point) in points.iter().enumerate().take(points.len() - 1).skip(2) {
                let cp_idx1 = (i - 2) * 2 + 1;
                let cp_idx2 = (i - 1) * 2;

                ctx.bezier_curve_to(
                    cps[cp_idx1].x, cps[cp_idx1].y, cps[cp_idx2].x, cps[cp_idx2].y, point.at.x, point.at.y,
                );
            }

            // Last two plot points are connected with a quadratic Bezier that requires a single control point
            ctx.quadratic_curve_to(
                cps[cps.len() - 1].x,
                cps[cps.len() - 1].y,
                points[points.len() - 1].at.x,
                points[points.len() - 1].at.y,
            );
        }

        // Draw the curve
        ctx.stroke();
        ctx.restore();
    }
}

/***********************************************************************************************************************
 * Translate an absorption value ranging from 0.00 .. 0.99 to a canvas pixel location
 */
fn scaled_y_pos(start: f64, axis_length: f64) -> impl Fn(f64) -> f64 {
    move |this_y: f64| start - (this_y * axis_length)
}

/***********************************************************************************************************************
 * X coordinate of Y axis name
 */
fn y_axis_name_x_pos(tick_label_width: f64, y_axis_inset: &f64) -> f64 {
    y_axis_inset
        - render::constants::TICK_LENGTH
        - (2.0 * render::constants::TICK_LABEL_GAP)
        - tick_label_width
        - crate::chart::constants::LABEL_FONT_SIZE
}

/***********************************************************************************************************************
 * Fetch DOM HTML image
 */
fn fetch_image(document: &web_sys::Document, img_name: &str) -> web_sys::HtmlImageElement {
    document
        .get_element_by_id(img_name)
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap()
}

/***********************************************************************************************************************
 * Draw an image at the specified location
 */
fn draw_image(
    ctx: &web_sys::CanvasRenderingContext2d,
    img: &web_sys::HtmlImageElement,
    at: PlotPoint,
    size: DimensionPair,
) {
    draw_partial_image(
        ctx,
        img,
        ImageSubdiv {
            sub_top_left: render::constants::ORIGIN,
            sub_dims: DimensionPair {
                width: img.width() as f64,
                height: img.height() as f64,
            },
            top_left: at,
            dims: size,
        },
    );
}

/***********************************************************************************************************************
 * Draw some subsection of an image at the specified location
 */
fn draw_partial_image(
    ctx: &web_sys::CanvasRenderingContext2d,
    img: &web_sys::HtmlImageElement,
    img_subdiv: ImageSubdiv,
) {
    ctx
        // Possibly the longest function name I've ever seen...
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            img,
            img_subdiv.sub_top_left.x,
            img_subdiv.sub_top_left.y,
            img_subdiv.sub_dims.width,
            img_subdiv.sub_dims.height,
            img_subdiv.top_left.x,
            img_subdiv.top_left.y,
            img_subdiv.dims.width,
            img_subdiv.dims.height,
        )
        .unwrap();
}

/***********************************************************************************************************************
 * Draw the control points
 * This function is only called if the TRACE_ACTIVE flag is switched on
 */
fn draw_control_points(ctx: &web_sys::CanvasRenderingContext2d, cps: &[PlotPoint]) {
    for i in 0..(cps.len() / 2) {
        let idx = 2 * i;
        draw_point(ctx, &cps[idx], &JsValue::from(crate::chart::constants::RGB_LIGHT_PINK));
        draw_point(ctx, &cps[idx + 1], &JsValue::from(crate::chart::constants::RGB_LIGHT_PINK));

        draw_line(
            ctx,
            &cps[idx],
            &cps[idx + 1],
            &JsValue::from(crate::chart::constants::RGB_LIGHT_PINK),
        );
    }
}
