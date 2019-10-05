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
import { do_trace_info} from "./trace.js"

const MOD_NAME     = "dom_access"
const DEBUG_ACTIVE = false

const trace_info = do_trace_info(DEBUG_ACTIVE)(MOD_NAME)

// *********************************************************************************************************************
// Fetch DOM elements by id, class name or name
function $id(elementId) {
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
            : trace_info("getParsedElementValue")(`Element '${elementId}' not found`)
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
      : getParsedElementValue("setDomElementProperty")(`DOM element '${elementId}' not found`)
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
}