/***********************************************************************************************************************
 * Porous Absorber Calculator - HTML Canvas Utilities
 *
 * (c) Chris Whealy 2020, 2024
 */
use libm::{pow, sqrt};
use wasm_bindgen::JsCast;

use crate::{
    chart::render::constants::{BOTTOM_MARGIN_INSET, RIGHT_MARGIN_INSET, X_AXIS_INSET},
    config::chart::PlotAbsPoint,
};

pub fn get_2d_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn canvas_dimensions(canvas: &web_sys::HtmlCanvasElement, y_axis_inset: &f64) -> (f64, f64, f64, f64, f64) {
    let h = canvas.height() as f64;
    let w = canvas.width() as f64;

    (
        h / 2.0,                               // vertical midpoint
        w / 2.0,                               // horizontal midpoint
        h - BOTTOM_MARGIN_INSET,               // bottom margin position
        w - y_axis_inset - RIGHT_MARGIN_INSET, // X axis length
        h - (2.0 * X_AXIS_INSET),              // Y axis length
    )
}

pub fn clear(canvas: &web_sys::HtmlCanvasElement) {
    // A simple, but non-inuitive way to clear the entire canvas...
    canvas.set_width(canvas.width());
}

pub fn distance(pt1: &PlotAbsPoint, pt2: &PlotAbsPoint) -> f64 {
    sqrt(pow(pt1.at.x_diff(&pt2.at), 2.0) + pow(pt1.at.y_diff(&pt2.at), 2.0))
}
