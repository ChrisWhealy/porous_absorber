/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * Unit conversion functions
 *
 * (c) Chris Whealy 2019, 2025
 **********************************************************************************************************************/

import { $id } from "./domAccess.js"
import { isNullOrUndef } from "./utils.js"
import defineTrace from "./appConfig.js"

const { traceFnBoundary, traceInfo } = defineTrace("unitConversion")
const INCHES_PER_METRE = 39.3701
const MILLIMETRES_PER_INCH = 25.4

// *********************************************************************************************************************
// Convert metric units to imperial
const toImperial = (units, val) => {
  let result = null

  switch (units) {
    // Metres to feet and inches
    case "m":
      let m_as_inches = val * INCHES_PER_METRE
      result = `(${Math.floor(m_as_inches / 12)} ft ${Number.parseFloat(m_as_inches % 12).toFixed(2)} in)`
      break

    // Milliemetres to inches
    case "mm":
      result = `(${Number.parseFloat(val / MILLIMETRES_PER_INCH).toFixed(3)} in)`
      break

    // Degrees Centigrade to degrees Fahrenheit
    case "°C":
      result = `(${Number.parseFloat((val * 9.0 / 5.0) + 32.0).toFixed(1)}°F)`
      break

    default:
  }

  return result
}

// *********************************************************************************************************************
// Partial function to display a value in a DOM element after being formatted by the supplied function
const showValueFn = (field_suffix, fn) =>
  (val, field_config) => {
    let el = $id(`${field_config.id}${field_suffix}`)

    isNullOrUndef(el)
      ? traceInfo("showValue")(`DOM element ${field_config.id}${field_suffix} not found`)
      : el.innerHTML = fn === "imperial"
        ? toImperial(field_config.units, val)
        : field_config.units === "each"
          ? `${val.toLocaleString('en')}`
          : `${val.toLocaleString('en')} ${field_config.units}`
  }

const showValue = traceFnBoundary("showValue", showValueFn)

// *********************************************************************************************************************
// Display range slider value in separate DOM element
const showUnits = showValue("_value", "default")
const convertUnits = showValue("_alt_units", "imperial")

// *********************************************************************************************************************
// Display range slider value unit conversion if necessary
const showAndConvertUnitsFn = field_config => {
  let displayValue = null

  switch (field_config.id) {
    case "porosity":
      displayValue = (
        Math.PI * (($id("hole_radius_mm").valueAsNumber / 1000) ** 2) / (($id("repeat_distance_mm").valueAsNumber / 1000) ** 2)
      ).toFixed(6)
      $id(field_config.id).innerHTML = displayValue
      break

    case "slotted_porosity":
      let sw = $id("slot_width_mm").value / 1000
      displayValue = (sw / (sw + ($id("slot_distance_mm").valueAsNumber / 1000))).toFixed(6)
      $id(field_config.id).innerHTML = displayValue
      break

    case "lock_abs_dims":
      if ($id("lock_abs_dims").checked) {
        let air_gap = $id("air_gap_mm")
        let abs_thickness = $id("absorber_thickness_mm")

        displayValue = `${air_gap.valueAsNumber + abs_thickness.valueAsNumber} mm`
      } else {
        displayValue = ""
      }

      break

    default:
      displayValue = field_config.getter(field_config.id)
  }

  showUnits(displayValue, field_config)
  convertUnits(displayValue, field_config)
}

const showAndConvertUnits = traceFnBoundary("showAndConvertUnits", showAndConvertUnitsFn)

export {
  showUnits,
  showAndConvertUnits
}
