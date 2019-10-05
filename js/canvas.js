/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * HTML Canvas utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { isBetween, isInsideRect } from "./utils.js"

const MIN_CANVAS_WIDTH  = 1000
const PLOT_POINT_RADIUS = 5
const RGB_LIGHT_GREY    = "rgb(223, 223, 223)"

const ABS_INFO_OFFSET = 10

const CANVAS_CONTAINER = "canvas_container"
const GRAPH            = "graph_canvas"
const GRAPH_OVERLAY    = "graph_canvas_overlay"

// *********************************************************************************************************************
// Calculate mouse position when over a canvas element
// *********************************************************************************************************************
const mousePositionOnCanvas =
  e =>
    isNaN(e.offsetX)
    ? mousePositionViaElementHierarchy(e)
    : (el =>
        ({ "x" : e.offsetX * (el.width  / el.offsetWidth  || 1)
         , "y" : e.offsetY * (el.height / el.offsetHeight || 1)
        })
      )
      (e.target)

const mousePositionViaElementHierarchy =
  e =>
    ((x, y, el) => {
      do {
        x  -= el.offsetLeft
        y  -= el.offsetTop
        el  = el.offsetParent
      }
      // Stop when we reach the top of the DOM hierarchy
      while (el)

      return {
        "x" : x * (el.width  / el.offsetWidth  || 1)
      , "y" : y * (el.height / el.offsetHeight || 1)
      }
    })
    (e.pageX, e.pageY, e.target)

// *********************************************************************************************************************
// Draw horizontal and vertical grid lines on the canvas within the chart boundary and create information popup when the
// mouse pointer hovers over an absorption plot point
// These cross hairs are drawn in the canvas overlay element, not the canvas containing the actual graph.  These two
// canvas elements must sit exactly on top of each other
// *********************************************************************************************************************
const canvasMouseOverHandler =
  (canvas, chartBox, seriestData) =>
    e => {
      let mousePos = mousePositionOnCanvas(e)
      let ctx = canvas.getContext("2d")

      // A quick, but non-intuitive way to blank out the entire canvas... :-)
      canvas.width = canvas.width

      drawCrossHairs(ctx, mousePos, chartBox)

      ctx.font = "10pt Arial"
    
      // For each X value in the inverted data series
      Object
        .keys(seriestData)
        .map(
          xValStr => 
            (xVal =>
              // Does the mouse pointer's current X position fall within the width of a plot point?
              (isBetween(xVal + PLOT_POINT_RADIUS, xVal - PLOT_POINT_RADIUS, mousePos.x))
              // Yup, so check mouse pointer Y position
              ? seriestData[xValStr].map(
                  plotPoint => 
                    (yVal =>
                      // Does the mouse pointer's current Y position also fall within the height of a plot point?
                      isBetween(yVal + PLOT_POINT_RADIUS, yVal - PLOT_POINT_RADIUS, mousePos.y)
                      // Yup, so display the absorption information
                      ? showAbsInfo(ctx, mousePos, plotPoint)
                      // Nope...
                      : null
                    )
                    (parseFloat(plotPoint.y))
                )
              // Nope...
              : null
            )
            (parseFloat(xValStr))
        )
    }

// *********************************************************************************************************************
// Set canvas size and maintain aspect ratio of 21:9
// *********************************************************************************************************************
const setCanvasSize =
  canvas =>
    (screen_width =>
      // Only resize the canvas if the screen width has changed
      // The canvas height does not need to change if only the screen height changes
      (canvas.width !== screen_width)
      ? (canvas.width = screen_width, canvas.height = (canvas.width / 21) * 9)
      : null
    )
    (Math.max(MIN_CANVAS_WIDTH, parseInt(window.getComputedStyle(document.body).width) - 2))

// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  mousePositionOnCanvas
, mousePositionViaElementHierarchy
, canvasMouseOverHandler
, setCanvasSize

, CANVAS_CONTAINER
, GRAPH
, GRAPH_OVERLAY
}



// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************

const drawCrossHairs =
  (ctx, mousePos, chartBox) => {
    // Define a function to determine whether the mouse pointer's current location falls inside the chart area
    let boxFn = isInsideRect(chartBox.top_left.x, chartBox.top_left.y, chartBox.bottom_right.x, chartBox.bottom_right.y)

    // Can we haz cross hairs?
    if (boxFn(mousePos)) {
      ctx.beginPath()
      ctx.strokeStyle = RGB_LIGHT_GREY

      // Draw vertical line
      ctx.moveTo(mousePos.x, chartBox.top_left.y)
      ctx.lineTo(mousePos.x, chartBox.bottom_right.y)

      // Draw horizontal line
      ctx.moveTo(chartBox.top_left.x,     mousePos.y)
      ctx.lineTo(chartBox.bottom_right.x, mousePos.y)

      ctx.stroke()
    }
}


// *********************************************************************************************************************
// Add "absorption @ frequency" text to canvas
// Before calling this function, you should have at least already set the canvas font
const showAbsInfo = (ctx, mousePos, plotPoint) => {
  ctx.fillText(`${plotPoint.abs} @ ${plotPoint.freq.toFixed(0)}Hz`, mousePos.x + ABS_INFO_OFFSET, mousePos.y - ABS_INFO_OFFSET)
}
