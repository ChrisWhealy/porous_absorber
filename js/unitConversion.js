/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Unit conversion functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { $id } from "./domAccess.js"

// *********************************************************************************************************************
// Define trace functions
import { define_trace } from "./appConfig.js"
const { traceBoundary, traceInfo } = define_trace("unitConversion")

// *********************************************************************************************************************
// Convert metric units to imperial
const to_imperial = (units, val) => {
  let result = null

  switch(units) {
    // Metres to feet and inches
    case "m":
      let m_as_inches = val * 39.3701
      result = `(${Math.floor(m_as_inches / 12)} ft ${Number.parseFloat(m_as_inches % 12).toFixed(2)} in)`
      break

    // Milliemetres to inches
    case "mm":
      let mm_as_inches = val / 25.4
      result = `(${Number.parseFloat(mm_as_inches).toFixed(3)} in)`
      break

    // Degrees Centigrade to degrees Fahrenheit
    case "°C":
      let c_as_f = (val * 9.0 / 5.0) + 32.0
      result = `(${Number.parseFloat(c_as_f).toFixed(1)}°F)`
      break

    default:
  }

  return result
}

// *********************************************************************************************************************
// Partial function to display a value in a DOM element after being formatted by the supplied function
const show_value =
  (field_suffix, fn) =>
    (val, field_config) => {
      const trace_bnd = traceBoundary("show_value", field_config.id)
      const trace     = traceInfo("show_value")

      trace_bnd(true)

      let el = $id(`${field_config.id}${field_suffix}`)

      if (el) {
        el.innerHTML = (fn === "imperial")
                       ? to_imperial(field_config.units, val)
                       : field_config.units === "each"
                         ? `${val.toLocaleString('en')}`
                         : `${val.toLocaleString('en')} ${field_config.units}`
      }
      else {
        trace(`DOM element ${field_config.id}${field_suffix} not found`)
      }

      trace_bnd(false)
    }

// *********************************************************************************************************************
// Display range slider value in separate DOM element
const show_units    = show_value("_value",     "default")
const convert_units = show_value("_alt_units", "imperial")

// *********************************************************************************************************************
// Display range slider value unit conversion if necessary
const showAndConvertUnits =
  field_config => {
    const trace_bnd = traceBoundary("showAndConvertUnits", field_config.id)
    trace_bnd(true)

    let displayValue = null

    switch(field_config.id) {
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

    show_units(displayValue, field_config)
    convert_units(displayValue, field_config)

    trace_bnd(false)
  }


// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  showAndConvertUnits
}
