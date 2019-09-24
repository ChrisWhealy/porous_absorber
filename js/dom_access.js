/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * DOM access utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

// *********************************************************************************************************************
// Fetch DOM element by id or class name
function $(elementId) {
  return document.getElementById(elementId)
}

function $class(elementId) {
  return document.getElementsByClassName(elementId)
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
    for (var rButton of document.getElementsByName(elementId)) {
      if (rButton.checked) {
        return rButton.value
      }
    }

    return 1
  }

// *********************************************************************************************************************
// Coerce a value to string then write to DOM element
const setString = (elementId, val) => $(elementId).value = val + ""

// Write integer value to DOM element
const setInt = (elementId, val) => $(elementId).value = parseInt(val)

// Write float value to DOM element
const setFloat = (elementId, val) => $(elementId).value = parseFloat(val)

// Set checkbox value
const setCheckbox = (elementId, val) => $(elementId).checked = !!val

// Set radio button
const setRadio = (elementId, val) => {
  for (var rButton of document.getElementsByName(elementId)) {
    if (rButton.value === val) {
      rButton.checked = true
    }
  }
}

