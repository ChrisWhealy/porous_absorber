/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

 // Identity funtion
const idiot = val => val

// A useful version of the Array.push()
const push = (el, arr) => (_ => arr)(arr.push(el))

const onlyDirection = dir => (acc, field) => field.direction === dir ? push(field, acc) : acc

// Partial functions for filtering in/outbound parameters from the metadata object
const filterInbound  = onlyDirection("in")
const filterOutbound = onlyDirection("out")

// Fetch required input values from the DOM
const fetchElementValue = parseFn => elementId => parseFn(document.getElementById(elementId).value)

const fetchFloat = fetchElementValue(parseFloat)
const fetchInt   = fetchElementValue(parseInt)
const fetchText  = fetchElementValue(idiot)

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
const writeString = (elementId, val) => document.getElementById(elementId).value = val


// Display range slider value and convert its metric value to imperial units
const show_and_convert_units =
  field => {
    // console.log(`${JSON.stringify(field)}`)

    let value    = field.fetch(field.id)
    let value_el = document.getElementById(`${field.id}_value`)
    let unit_el  = document.getElementById(`${field.id}_units`)

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

/***********************************************************************************************************************
 * The metadata object defines which HTML elements the WASM module should expect already to be present in the DOM.
 * 
 * The order of the fields with direction:"in" must match the argument order expected by the WASM function pa_calculator
 *
 * The value of the "id" property below must match the corresponding id of the input field in the DOM
 *
 * The value of "units" property is needed primarily for metric to imperial conversion but is maintained for all
 * values for consistency and potential future use
 *
 * The "direction" field indicates the direction of data flow with respect to the WASM module.  Fields having a
 * direction of "out" correspond to read only HTML elements
 *
 * The "fetch" and "update" properties are set to the function name needed to read or write the value to or from the
 * corresponding HTML element
 *
 * The WASM moduile returns either an array of error messages or the string "Ok"
 **********************************************************************************************************************/
const dom_metadata = [
  { id : "absorber_thickness_mm", type : "int",    units : "mm",      fetch : fetchInt   }
, { id : "flow_resistivity",      type : "int",    units : "rayls/m", fetch : fetchInt   }
, { id : "air_gap_mm",            type : "int",    units : "mm",      fetch : fetchInt   }
, { id : "angle",                 type : "int",    units : "°",       fetch : fetchInt   }
, { id : "start_graph_freq",      type : "float",  units : "Hz",      fetch : fetchFloat }
, { id : "subdivision",           type : "int",    units : "each",    fetch : fetchRadio }
, { id : "air_temp",              type : "int",    units : "°C",      fetch : fetchInt   }
, { id : "air_pressure",          type : "int",    units : "bar",     fetch : fetchFloat }
]


/***********************************************************************************************************************
 * Invoke the appropriate WASM function
 */
const calculate_pa = () => {
  let current_field_values = dom_metadata.map(field => field.fetch(field.id))
  let wasm_response        = pa_calculator.apply(null, current_field_values)

  // console.log(wasm_response)
}

/***********************************************************************************************************************
 * This function must be called everytime an input value changes
 */
const update_screen = () => {
  dom_metadata.map(show_and_convert_units)
  calculate_pa()
}

