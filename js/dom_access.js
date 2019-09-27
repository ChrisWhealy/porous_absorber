/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * DOM access utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

// *********************************************************************************************************************
// Fetch DOM elements by id, class name or name
function $(elementId) {
  return document.getElementById(elementId)
}

function $class(elementId) {
  return document.getElementsByClassName(elementId)
}

function $name(elementId) {
  return document.getElementsByName(elementId)
}

// *********************************************************************************************************************
// Partial function to fetch, then parse a DOM element value
const getParsedElementValue =
  parseFn =>
    elementId =>
      (el =>
         el ? parseFn(el.value)
            : console.log(`Element '${elementId}' not found`)
      )
      ($(elementId))

const getFloat     = getParsedElementValue(parseFloat)
const getInt       = getParsedElementValue(parseInt)
const getText      = getParsedElementValue(idiot)

const getInnerHTML = elementId => $(elementId).innerHTML
const getCheckbox  = elementId => $(elementId).checked

const getRadio =
  elementId => {
    for (var rButton of $name(elementId)) {
      if (rButton.checked) {
        return rButton.value
      }
    }

    return 1
  }

// *********************************************************************************************************************
// Write values to DOM elements
const setDomElementProperty =
  (elementId, propName, val) => 
    (el => el ? el[propName] = val : console.log(`DOM element '${elementId}' not found`))
    ($(elementId))

const setString   = (elementId, val) => setDomElementProperty(elementId, "value", val)
const setInt      = (elementId, val) => setDomElementProperty(elementId, "value", parseInt(val))
const setFloat    = (elementId, val) => setDomElementProperty(elementId, "value", parseFloat(val))
const setCheckbox = (elementId, val) => setDomElementProperty(elementId, "checked", !!val)

// Set radio button
const setRadio = (elementId, val) => {
  for (var rButton of $name(elementId)) {
    rButton.checked = (rButton.value === val)
  }
}

