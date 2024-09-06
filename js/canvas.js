/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * HTML Canvas utility functions
 *
 * (c) Chris Whealy 2020
 **********************************************************************************************************************/

import {isBetween, isInsideRect} from "./utils.js"
import {MIN_CANVAS_WIDTH} from "./appConfig.js"

const PLOT_POINT_RADIUS = 5
const RGB_LIGHT_GREY = "rgb(223, 223, 223)"
const ABS_INFO_OFFSET = 10
const CANVAS_CONTAINER = "canvas_container"
const GRAPH = "graph_canvas"
const GRAPH_OVERLAY = "graph_canvas_overlay"

// *********************************************************************************************************************
//                                                 P R I V A T E   A P I
// *********************************************************************************************************************

const drawCrossHairs =
    (ctx, mousePos, chartBox) => {
        ctx.beginPath()
        ctx.strokeStyle = RGB_LIGHT_GREY

        // Draw vertical line
        ctx.moveTo(mousePos.x, chartBox.top_left.y)
        ctx.lineTo(mousePos.x, chartBox.bottom_right.y)

        // Draw horizontal line
        ctx.moveTo(chartBox.top_left.x, mousePos.y)
        ctx.lineTo(chartBox.bottom_right.x, mousePos.y)

        ctx.stroke()
    }

// *********************************************************************************************************************
// Add "absorption @ frequency" text to canvas
// Before calling this function, you should have at least already set the canvas font
const showAbsInfo = (ctx, mousePos, canvasWidth, plotPoint) => {
    // Build the text then find out how wide it is
    let txt = `${plotPoint.abs} @ ${plotPoint.freq.toFixed(0)}Hz`
    let txtWidth = ctx.measureText(txt).width

    // Add the absorption text to the canvas
    ctx.fillText(
        txt,
        // On which side of the plot point should the text be positioned?
        txtWidth + mousePos.x + ABS_INFO_OFFSET < canvasWidth
            ? mousePos.x + ABS_INFO_OFFSET
            : mousePos.x - ABS_INFO_OFFSET - txtWidth,
        mousePos.y - ABS_INFO_OFFSET
    )
}

// *********************************************************************************************************************
//                                                  P U B L I C   A P I
// *********************************************************************************************************************

// *********************************************************************************************************************
// Calculate mouse position when over a canvas element
const mousePositionOnCanvas =
    e => {
        if (isNaN(e.offsetX)) {
            return mousePositionViaElementHierarchy(e)
        } else {
            let el = e.target
            return ({
                "x": e.offsetX * (el.width / el.offsetWidth || 1),
                "y": e.offsetY * (el.height / el.offsetHeight || 1)
            })
        }
    }

const mousePositionViaElementHierarchy =
    e => {
        let x = e.pageX
        let y = e.pageY
        let el = e.target

        // Stop when we reach the top of the DOM hierarchy
        do {
            x -= el.offsetLeft
            y -= el.offsetTop
            el = el.offsetParent
        }
        while (el)

        return {
            "x": x * (el.width / el.offsetWidth || 1),
            "y": y * (el.height / el.offsetHeight || 1)
        }
    }

// *********************************************************************************************************************
// Draw horizontal and vertical grid lines on the canvas within the chart boundary and create information popup when the
// mouse pointer hovers over an absorption plot point
// These cross-hairs are drawn in the canvas overlay element, not the canvas containing the actual graph.
// These two canvas elements must sit exactly on top of each other
const canvasMouseOverHandler =
    (canvas, chartBox, seriesData) => {
        // Generate a function to determine whether the mouse pointer's current location is within the chart box area
        let boxContains = isInsideRect(chartBox.top_left.x, chartBox.top_left.y, chartBox.bottom_right.x, chartBox.bottom_right.y)
        let ctx = canvas.getContext("2d")

        return e => {
            let mousePos = mousePositionOnCanvas(e)

            // A quick, but non-intuitive way to blank out the entire canvas... :-)
            canvas.width = canvas.width
            ctx.font = "11pt Arial"

            // Draw cross-hairs if the mouse pointer is within the chart box area
            if (boxContains(mousePos)) {
                drawCrossHairs(ctx, mousePos, chartBox)
            }

            // For each X value in the inverted data series
            Object
                .keys(seriesData)
                .map(
                    xValStr => {
                        let xVal = parseFloat(xValStr)

                        // Does the mouse pointer's current X position fall within the width of a plot point?
                        return (isBetween(xVal + PLOT_POINT_RADIUS, xVal - PLOT_POINT_RADIUS, mousePos.x))
                            // Yup, so check mouse pointer Y position
                            ? seriesData[xValStr].map(
                                plotPoint => {
                                    let yVal = parseFloat(plotPoint.y)
                                    // Does the mouse pointer's current Y position also fall within the height of a plot point?
                                    return isBetween(yVal + PLOT_POINT_RADIUS, yVal - PLOT_POINT_RADIUS, mousePos.y)
                                        // Yup, so display the absorption information
                                        ? showAbsInfo(ctx, mousePos, canvas.width, plotPoint)
                                        // Nope...
                                        : null
                                }
                            )
                            // Nope...
                            : null
                    }
                )
        }
    }

// *********************************************************************************************************************
// Set canvas size and maintain aspect ratio of 21:9
const setCanvasSize =
    canvas => {
        let screen_width = Math.max(
            MIN_CANVAS_WIDTH,
            parseInt(window.getComputedStyle(document.body).width) - 2
        )

        // Only resize the canvas if the screen width has changed
        // The canvas height does not need to change if only the screen height changes
        return canvas.width !== screen_width
            ? (canvas.width = screen_width, canvas.height = (canvas.width / 21) * 9)
            : null
    }

export {
    mousePositionOnCanvas,
    mousePositionViaElementHierarchy,
    canvasMouseOverHandler,
    setCanvasSize,
    CANVAS_CONTAINER,
    GRAPH,
    GRAPH_OVERLAY
}
