/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * HTML Canvas utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

const MIN_CANVAS_WIDTH = 1200
const RGB_LIGHT_GREY = "rgb(223, 223, 223)"

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
  e => ((x, y, el) => {

    do {
      x  -= el.offsetLeft
      y  -= el.offsetTop
      el  = el.offsetParent
    }
    // Stop when we reach the top of the DOM hierarchy
    while (el)

    // Since "el" is now falsey, we must reference the previous element to obtain the required values
    return {
      "x" : x * (el.width  / el.offsetWidth  || 1)
    , "y" : y * (el.height / el.offsetHeight || 1)
    }
  })
  (e.pageX, e.pageY, e.target)

// *********************************************************************************************************************
// Draw horizontal and vertical grid lines on canvas and create information popup when teh mouse pointer hovers over an
// absorption plot point
// *********************************************************************************************************************
const canvasMouseOverHandler =
  (canvas, xsAndYs) =>
    e => {
      let mousePos = mousePositionOnCanvas(e)
      let ctx = canvas.getContext("2d")

      // A quick, but non-intuitive way to blank out the canvas... :-)
      canvas.width = canvas.width

      drawCrossHairs(ctx, mousePos, canvas.width, canvas.height)
      // console.log(`xs and ys = ${JSON.stringify(xsAndYs)}`)
    }


// *********************************************************************************************************************
// Set canvas size and maintain aspect ratio of 21:9
// *********************************************************************************************************************
const setCanvasSize =
  canvas => {
    canvas.width  = Math.max(MIN_CANVAS_WIDTH, parseInt(window.getComputedStyle(document.body).width) - 2)
    canvas.height = (canvas.width / 21) * 9
  }

  // *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  mousePositionOnCanvas
, mousePositionViaElementHierarchy
, canvasMouseOverHandler
, setCanvasSize
}

// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************
const drawCrossHairs =
  (ctx, mousePos, width, height) => {
    // Plot a vertical line on the canvas
    ctx.beginPath()
    ctx.strokeStyle = RGB_LIGHT_GREY

    // Draw vertical line
    ctx.moveTo(mousePos.x, 0)
    ctx.lineTo(mousePos.x, height)

    // Draw horizontal line
    ctx.moveTo(0, mousePos.y)
    ctx.lineTo(width, mousePos.y)

    ctx.stroke()

}