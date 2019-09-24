/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Unit conversion functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

// *********************************************************************************************************************
// Partial function to display a value in a DOM element after being formatted by the supplied function
const show_value =
  (field_suffix, fn) =>
    (val, field_config) => {
      let el = $(`${field_config.id}${field_suffix}`)

      if (el) {
        el.innerHTML = (fn === "imperial")
                       ? to_imperial(field_config.units, val)
                       : field_config.units === "each"
                         ? `${val.toLocaleString('en')}`
                         : `${val.toLocaleString('en')} ${field_config.units}`
      }
      else {
        trace(`     ${field_config.id}${field_suffix} element not found`)
      }
    }

// *********************************************************************************************************************
// Display range slider value in separate DOM element
const show_units    = show_value("_value",     "default")
const convert_units = show_value("_alt_units", "imperial")

// *********************************************************************************************************************
// Display range slider value unit conversion if necessary
const show_and_convert_units =
  field_config => {
    trace(`---> show_and_convert_units(${field_config.id})`)
    // trace(`${JSON.stringify(field_config)}`)

    let displayValue = null

    switch(field_config.id) {
      case "porosity":
        displayValue = (
            Math.PI
          * (($("hole_radius_mm").value / 1000) ** 2)
          / (($("repeat_distance_mm").value / 1000) ** 2)
          ).toFixed(6)
        $("porosity").innerHTML = displayValue
        break

      case "cavity_depth_mm":
        displayValue = (1 * $("air_gap_mm").value) + (1 * $("absorber_thickness_mm").value)
        break

      default:
        displayValue = field_config.getter(field_config.id)
      }

      show_units(displayValue, field_config)
      convert_units(displayValue, field_config)

    trace(`<--- show_and_convert_units(${field_config.id})`)
  }

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
      result = `(${Number.parseFloat(mm_as_inches).toFixed(2)} in)`
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
