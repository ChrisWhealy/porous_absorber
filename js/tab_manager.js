/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Tab management functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import {
  push
, setProperty
, isArray
, isNotNullOrUndef
} from "./utils.js"

import { $id, $class }            from "./dom_access.js"
import { tabConfig }              from "./config.js"
import { show_and_convert_units } from "./unit_conversion.js"
import { mousePositionOnCanvas }  from "./canvas.js"

// *********************************************************************************************************************
// Define trace functions
import { do_trace_boundary, do_trace_info} from "./trace.js"

const MOD_NAME     = "tab_manager"
const DEBUG_ACTIVE = false

const trace_boundary = do_trace_boundary(DEBUG_ACTIVE)(MOD_NAME)
const trace_info     = do_trace_info(DEBUG_ACTIVE)(MOD_NAME)

// *********************************************************************************************************************
// UI slider range limitation
// *********************************************************************************************************************

// Modifier function for use with the limit_max() function
const half   = val => val / 2.0
const double = val => val * 2.0

// Restrict the maximum value of the target UI element to the current value of the source element after adjusting it
// with some modifier function.  Default to the idiot function if no modifier is supplied
const limit_max =
  (srcEl, targetEl, upperLimit, modifierFn) =>
    (modFn => $id(targetEl).max = Math.min(modFn(srcEl.value), upperLimit))
    (modifierFn || idiot)


// *********************************************************************************************************************
// Tab management
// *********************************************************************************************************************
const open_tab = (evt, tabName) => {
  const trace_bnd = trace_boundary("openTab")
  trace_bnd(true)

  // Cache values from current tab and deactive that tab button
  cache_values_and_deactivate()
  hide_and_empty_all_tabs()

  // Make the selected tab button active
  evt.currentTarget.className += " active"
  $id(tabName).style.display = "block"
  
  fetch_tab(tabName)

  trace_bnd(false)
}

// *********************************************************************************************************************
// Hide tabs and remove their content except for the configuration tab
const hide_and_empty_all_tabs =
  () => {
    const trace_bnd = trace_boundary("hide_all_tabs")
    trace_bnd(true)

    for (var tab of $class("tabContent")) {
      tab.style.display = "none"

      if (tab.id !== "configuration") {
        tab.innerHTML = ""
      }
    }

    trace_bnd(false)
  }

// *********************************************************************************************************************
// Cache values from the current tab in local storage, then deactivate the tab button
const cache_values_and_deactivate =
  () => {
    const trace_bnd = trace_boundary("cache_values_and_deactivate")
    trace_bnd(true)

    for (var tablink of $class("tabButton")) {
      if (tablink.className.indexOf("active") > -1) {
        tablink.className = tablink.className.replace(" active", "")
        window.store_tab_values(tablink.id.replace("tab_button_", ""))
      }
    }

    trace_bnd(false)
  }

// *********************************************************************************************************************
// Fetch tab content from server
const fetch_tab =
  tabName => {
    const trace_bnd = trace_boundary("fetch_tab", tabName)

    trace_bnd(true)

    let req = new XMLHttpRequest()
    
    req.open('GET',`./tabs/${tabName}.html`)
    
    req.onload = () => {
      trace_boundary("onload", tabName)(true)
      trace_info("onload")(`Inserting HTML for ${tabName}`)
      $id(tabName).innerHTML = ""
      $id(tabName).insertAdjacentHTML('afterbegin', req.response)
      
      // Restore the current tab's values
      // This function is defined in main.js based on the availability of local storage.  If local storage is not
      // available, then this function evaluates to no_op
      window.restore_tab_values(tabName)
      
      update_screen(tabName)

      // All tabs contain a canvas element except for configuration
      // if (tabName !== "configuration") {
      //   $id("graph_canvas").onmousemove = e => (p => $id('mouse').innerHTML = `${p.x}, ${p.y}`)(mousePositionOnCanvas(e))
      // }

      trace_boundary("onload", tabName)(false)
    }
    
    req.send()
    trace_bnd(false)
  }

// *********************************************************************************************************************
// Fetch config values either from local storage or from the DOM
// These values must be returned as an array where the order is "air_temp" followed by "air_pressure"
const fetch_config_from_ls =
  () =>
    (config_vals => [config_vals.air_temp, config_vals.air_pressure])
    (JSON
      .parse(window.localStorage.getItem("configuration"))
      .reduce((acc, field) => setProperty(acc, field.id, field.value), {})
    )

const fetch_config_from_dom = () => [$id("air_temp").value, $id("air_pressure").value]

const fetch_config_values =
  () =>
    !!window.localStorage.getItem("configuration")
    ? fetch_config_from_ls()
    : fetch_config_from_dom()

// *********************************************************************************************************************
// This function must be called every time an input value is changed
const update_screen =
  tabName => {
    const trace_bnd = trace_boundary("update_screen", tabName)
    const trace     = trace_info("update_screen")

    trace_bnd(true)
    
    // Perform any unit conversions that might be needed then extract the input values relevant for the WASM module
    let current_field_values = tabConfig[tabName]
      .reduce((acc, field) => {
          show_and_convert_units(field)
          return field.isWasmArg ? push(field.getter(field.id), acc) : acc
        }, [])

    // For non-configuration tabs, add the configuration values since these are common to all calculations
    if (tabName !== "configuration") {
      current_field_values = current_field_values.concat(window.get_config())
    }

    // What are we sending to WASM?
    trace(`Passing [${current_field_values.join(", ")}] to WASM function ${tabName}`)

    // WASM does its magic unless the configuration tab is selected, in which case window[tabName] resolves to no_op
    let wasm_response = window[tabName].apply(null, current_field_values)
    
    // Write the WASM response to the console using either "error" or "log"
    // if (isNotNullOrUndef(wasm_response)) {
    //   // We expect to get an object back from WASM. However, if we get an array, then there is at least one error
    //   console[isArray(wasm_response) ? "error" : "log"](JSON.stringify(wasm_response, null, 2))
    // }
    
    trace_bnd(false)
}


// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  limit_max
, half
, double
, open_tab
, update_screen
, fetch_tab
, fetch_config_from_dom
, fetch_config_from_ls
, fetch_config_values
}