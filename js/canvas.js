/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * HTML Canvas utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

const mousePositionOnCanvas =
  e =>
    (el =>
      isNaN(e.offsetX)
      ? mousePositionViaElementHierarchy(e)
      : { "x" : e.offsetX * (el.width  / el.offsetWidth  || 1)
        , "y" : e.offsetY * (el.height / el.offsetHeight || 1)
        })
    (e.target)

const mousePositionViaElementHierarchy =
  e => ((x, y, el) => {
    let prevEl

    do {
      x     -= el.offsetLeft
      y     -= el.offsetTop
      prevEl = el
      el     = el.offsetParent
    }
    // Stop when we reach the top of the DOM hierarchy
    while (el)

    // Since "el" is now falsey, we must reference the previous element to obtain the required values
    return {
      "x" : x * (prevEl.width  / prevEl.offsetWidth  || 1)
    , "y" : y * (prevEl.height / prevEl.offsetHeight || 1)
    }
  })
  (e.pageX, e.pageY, e.target)

// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  mousePositionOnCanvas
, mousePositionViaElementHierarchy
}