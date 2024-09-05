/***********************************************************************************************************************
* Porous Absorber Calculator - Beziér control points
*
* (c) Chris Whealy 2020
*/
use crate::chart::render::canvas_utils::distance;
use crate::config::chart::{PlotAbsPoint, PlotPoint};

/***********************************************************************************************************************
 * Generate two Bézier control points that lie between the three supplied plot points
 * The tension parameter indicates the degree of curvature between the three points.
 * Setting the tension to zero results in straight lines
 */
pub fn gen_control_points(pt1: &PlotAbsPoint, pt2: &PlotAbsPoint, pt3: &PlotAbsPoint, tension: f64) -> Vec<PlotPoint> {
    // Calculate the length of the two line segments
    let seg_1_len = distance(pt1, pt2);
    let seg_2_len = distance(pt2, pt3);
    let total_len = seg_1_len + seg_2_len;
    let seg_1_ratio = seg_1_len / total_len;
    let seg_2_ratio = seg_2_len / total_len;

    // Calculate the gradient between the start and finish points.  The control points then live on a line that has this
    // gradient and passes through the middle point
    let x_vec = pt3.at.x_diff(&pt1.at);
    let y_vec = pt3.at.y_diff(&pt1.at);

    // Return the coordinates of the two control points lying between the three supplied points
    vec![
        PlotPoint {
            x: pt2.at.x - x_vec * tension * seg_1_ratio,
            y: pt2.at.y - y_vec * tension * seg_1_ratio,
        },
        PlotPoint {
            x: pt2.at.x + x_vec * tension * seg_2_ratio,
            y: pt2.at.y + y_vec * tension * seg_2_ratio,
        },
    ]
}
