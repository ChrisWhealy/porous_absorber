/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * DOM access utility functions
 *
 * (c) Chris Whealy 2020
 **********************************************************************************************************************/

import {idiot} from "./utils.js"

// *********************************************************************************************************************
// Define trace functions
import defineTrace from "./appConfig.js"

const {traceInfo} = defineTrace("domAccess")

// *********************************************************************************************************************
//                                                 P R I V A T E   A P I
// *********************************************************************************************************************

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


// *********************************************************************************************************************
// Write values to DOM elements
const setDomElementProperty =
    (elementId, propName, parsedVal) =>
        (el =>
            el ? el[propName] = parsedVal
               : traceInfo("setDomElementProperty")(`DOM element '${elementId}' not found`)
        )
        ($id(elementId))

// *********************************************************************************************************************
//                                                  P U B L I C   A P I
// *********************************************************************************************************************

// *********************************************************************************************************************
// Fetch DOM elements by id, class name or name
// Functions with "$" in the name cannot be defined using the fat arrow syntax
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
// Getter functions for DOM elements of various data types
const getFloat = getParsedElementValue(parseFloat)
const getInt = getParsedElementValue(parseInt)
const getText = getParsedElementValue(idiot)

const getInnerHTML = elementId => $id(elementId).innerHTML
const getCheckbox = elementId => $id(elementId).checked

const getRadio =
    elementId => {
        for (var rButton of $name(elementId)) {
            if (rButton.checked) {
                // The radio button string value must be coerced to a number
                return +rButton.value
            }
        }

        return 1
    }

// *********************************************************************************************************************
// Setter functions for DOM element of various data types
const setString = (elementId, val) => setDomElementProperty(elementId, "value", val)
const setInt = (elementId, val) => setDomElementProperty(elementId, "value", parseInt(val))
const setFloat = (elementId, val) => setDomElementProperty(elementId, "value", parseFloat(val))
const setCheckbox = (elementId, val) => setDomElementProperty(elementId, "checked", !!val)

// Set radio button - the numeric argument value must be coerced to a string
const setRadio = (elementId, val) => {
    for (var rButton of $name(elementId)) {
        rButton.checked = (rButton.value === val + "")
    }
}

// *********************************************************************************************************************
// Fetch air temperature and pressure config values from the DOM
// *********************************************************************************************************************
const fetchConfigFromDom = () => ({
    "air_temp":     $id("air_temp").value,
    "air_pressure": $id("air_pressure").value
})

export {
    $id,
    $class,
    $name,
    getFloat,
    getInt,
    getText,
    getInnerHTML,
    getCheckbox,
    getRadio,
    setString,
    setInt,
    setFloat,
    setCheckbox,
    setRadio,
    fetchConfigFromDom
}
