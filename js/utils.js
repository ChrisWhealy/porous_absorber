/***********************************************************************************************************************
 * Porous Absorber Calculator Utilities
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

 // Identity function (which, according to Dave Keenan, should more appropriately be called the idiot function.
 // See http://dkeenan.com/Lambda/ for details)
const idiot = val => val

// Fetch required input values from the DOM
function $(elementId) {
  return document.getElementById(elementId)
}

const fetchParsedElementValue = parseFn => elementId => parseFn($(elementId).value)

const fetchFloat = fetchParsedElementValue(parseFloat)
const fetchInt   = fetchParsedElementValue(parseInt)
const fetchText  = fetchParsedElementValue(idiot)

const fetchCheckbox = elementId => $(elementId).checked

const fetchRadio =
  elementId => {
    for (var rButton of document.getElementsByName(elementId)) {
      if (rButton.checked) {
        return rButton.value
      }
    }

    return 1
  }

// Write string to element value
const writeString = (elementId, val) => $(elementId).value = val

// Display input values and convert to alternative units if necessary
const show_and_convert_units =
  field => {
    // console.log(`${JSON.stringify(field)}`)

    let value    = field.fetch(field.id)
    let value_el = $(`${field.id}_value`)
    let unit_el  = $(`${field.id}_units`)

    value_el ? show_units(value, value_el, field)   : undefined
    unit_el  ? convert_units(value, unit_el, field) : undefined
  }

// Display range slider value
const show_units =
  (val, el, field) =>
    el.innerHTML = `${val} ${field.units !== "each" ? field.units : ""}`

// Convert units
const convert_units =
  (val, el, field) =>
    el ? el.innerHTML = to_imperial(field.units, val)
       : undefined

// Convert metric units to imperial
const to_imperial = (units, val) => {
  let result = null

  switch(units) {
    case "m":
      let m_as_inches = val * 39.3701
      result = `(${Math.floor(m_as_inches / 12)} ft ${Number.parseFloat(m_as_inches % 12).toFixed(2)} in)`
      break

    case "mm":
      let mm_as_inches = val / 25.4
      result = `(${Number.parseFloat(mm_as_inches).toFixed(2)} in)`
      break

    case "°":
      result = `${val === 0 ? "Normal" : "Oblique"} incidence`
      break

    default:
  }

  return result
}

