/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * Unit conversion functions
 *
 * (c) Chris Whealy 2020
 **********************************************************************************************************************/

import {$id} from "./domAccess.js"
import {isNullOrUndef} from "./utils.js"
import defineTrace from "./appConfig.js"

const {traceFnBoundary, traceInfo} = defineTrace("unitConversion")

// *********************************************************************************************************************
//                                                 P R I V A T E   A P I
// *********************************************************************************************************************

// *********************************************************************************************************************
// Convert metric units to imperial
const toImperial = (units, val) => {
    let result = null

    switch (units) {
        // Metres to feet and inches
        case "m": {
            let m_in_inches = val * 39.3701
            let feet = Math.floor(m_in_inches / 12)
            let inches = m_in_inches % 12
            result = `(${feet} ft ${inches.toFixed(2)} in)`
            break
        }

        // Millimetres to inches
        case "mm": {
            let inches = (val / 25.4).toFixed(3)
            result = `(${inches} in)`
            break
        }

        // Degrees Centigrade to degrees Fahrenheit
        case "°C": {
            let fahrenheit = ((val * 9.0 / 5.0) + 32.0).toFixed(1)
            result = `(${fahrenheit}°F)`
            break
        }

        default:
    }

    return result
}

// *********************************************************************************************************************
// Partial function to display a value in a DOM element after being formatted by the supplied function
const showValueFn =
    (field_suffix, fn) =>
        (val, field_config) => {
            let el = $id(`${field_config.id}${field_suffix}`)

            return isNullOrUndef(el)
                ? traceInfo("showValue")(`DOM element ${field_config.id}${field_suffix} not found`)
                : el.innerHTML =
                    fn === "imperial"
                        ? toImperial(field_config.units, val)
                        : field_config.units === "each"
                            ? `${val.toLocaleString()}`
                            : `${val.toLocaleString()} ${field_config.units}`
        }

const showValue = traceFnBoundary("showValue", showValueFn)

// *********************************************************************************************************************
// Display range slider value in separate DOM element
const showUnits = showValue("_value", "default")
const convertUnits = showValue("_alt_units", "imperial")

// *********************************************************************************************************************
// Display range slider value unit conversion if necessary
const showAndConvertUnitsFn =
    field_config => {
        let displayValue

        switch (field_config.id) {
            case "porosity":
                displayValue = (
                    Math.PI
                    * (($id("hole_radius_mm").value / 1000) ** 2)
                    / (($id("repeat_distance_mm").value / 1000) ** 2)
                ).toFixed(6)
                $id(field_config.id).innerHTML = displayValue
                break

            case "slotted_porosity":
                let sw = $id("slot_width_mm").value / 1000
                displayValue = (sw / (sw + ($id("slot_distance_mm").value / 1000))).toFixed(6)
                $id(field_config.id).innerHTML = displayValue
                break

            case "cavity_depth_mm":
                displayValue = (1 * $id("air_gap_mm").value) + (1 * $id("absorber_thickness_mm").value)
                break

            default:
                displayValue = field_config.getter(field_config.id)
        }

        showUnits(displayValue, field_config)
        convertUnits(displayValue, field_config)
    }

// *********************************************************************************************************************
//                                                  P U B L I C   A P I
// *********************************************************************************************************************

// *********************************************************************************************************************
// Wrap private API functions in boundary trace functionality then expose as public API
const showAndConvertUnits = traceFnBoundary("showAndConvertUnits", showAndConvertUnitsFn)

export {
    showAndConvertUnits
}
