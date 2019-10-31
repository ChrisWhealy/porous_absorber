/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * DOM access utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { idiot } from "./utils.js"

// *********************************************************************************************************************
// Define trace functions
import { define_trace } from "./appConfig.js"
const { traceInfo } = define_trace("domAccess")

// *********************************************************************************************************************
// Fetch DOM elements by id, class name or name
function $id(elementId) {
  return document.getElementById(elementId)
}

function $class(className) {
  return document.getElementsByClassName(className)
}

function $name(elementName) {
  return document.getElementsByName(elementName)
}

// *********************************************************************************************************************
// Partial function to fetch, then parse a DOM element value
const getParsedElementValue =
  parseFn =>
    elementId =>
      (el =>
         el ? parseFn(el.value)
            : traceInfo("getParsedElementValue")(`Element '${elementId}' not found`)
      )
      ($id(elementId))

const getFloat     = getParsedElementValue(parseFloat)
const getInt       = getParsedElementValue(parseInt)
const getText      = getParsedElementValue(idiot)

const getInnerHTML = elementId => $id(elementId).innerHTML
const getCheckbox  = elementId => $id(elementId).checked

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
  (elementId, propName, parsedVal) => 
    (el =>
      el
      ? el[propName] = parsedVal
      : traceInfo("setDomElementProperty")(`DOM element '${elementId}' not found`)
    )
    ($id(elementId))

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

// *********************************************************************************************************************
// Fetch air temperature and pressure config values from the DOM
const fetchConfigFromDom = () => ({
  "air_temp"     : $id("air_temp").value
, "air_pressure" : $id("air_pressure").value
})


// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  $id
, $class
, $name

, getFloat
, getInt
, getText
, getInnerHTML
, getCheckbox
, getRadio

, setString
, setInt
, setFloat
, setCheckbox
, setRadio

, fetchConfigFromDom
}