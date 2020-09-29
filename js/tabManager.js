/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * Tab management functions
 *
 * (c) Chris Whealy 2020
 **********************************************************************************************************************/

import {
  isArray
, isNotNullOrUndef
, invertPlotData
, idiot
, setProperty
} from "./utils.js"

import tabConfig from "./tabConfig.js"

import { $id, $class }         from "./domAccess.js"
import { showAndConvertUnits } from "./unitConversion.js"

import {
  canvasMouseOverHandler
, CANVAS_CONTAINER
, GRAPH_OVERLAY
} from "./canvas.js"

// *********************************************************************************************************************
// Define trace functions
// *********************************************************************************************************************
import defineTrace from "./appConfig.js"
const { traceFnBoundary, traceInfo } = defineTrace("tabManager")



// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                 P R I V A T E   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



// *********************************************************************************************************************
// Tab management
// *********************************************************************************************************************
const openTabFn =
  (evt, tabName) => {
    // Remove graph from screen when the configuration tab is selected and blank out graph overlay canvas
    $id(CANVAS_CONTAINER).className = tabName === "configuration" ? "fadeOut" : "fadeIn"
    $id(GRAPH_OVERLAY).width        = $id(GRAPH_OVERLAY).width

    // Cache values from current tab and deactive that tab button
    cacheValuesAndDeactivate()
    hideAndEmptyAllTabs()

    // Make the selected tab button active
    evt.currentTarget.className += " active"
    $id(tabName).style.display = "block"
    fetchTab(tabName)
  }


// *********************************************************************************************************************
// Hide tabs and remove their content except for the configuration tab
// *********************************************************************************************************************
const hideAndEmptyAllTabsFn =
  () => {
    for (var tab of $class("tabContent")) {
      tab.style.display = "none"

      if (tab.id !== "configuration") {
        tab.innerHTML = ""
      }
    }
  }

const hideAndEmptyAllTabs = traceFnBoundary("hideAndEmptyAllTabs", hideAndEmptyAllTabsFn)

// *********************************************************************************************************************
// Cache values from the current tab into local storage, then deactivate the tab button
// *********************************************************************************************************************
const cacheValuesAndDeactivateFn =
  () => {
    for (var tablink of $class("tabButton")) {
      if (tablink.className.indexOf("active") > -1) {
        tablink.className = tablink.className.replace(" active", "")
        window.store_tab_values(tablink.id.replace("tab_button_", ""))
      }
    }
  }

const cacheValuesAndDeactivate = traceFnBoundary("cacheValuesAndDeactivate", cacheValuesAndDeactivateFn)

// *********************************************************************************************************************
// Partial function that generates another function to respond to the onload event after tab HTML data is returned to
// the client
// *********************************************************************************************************************
const tabLoadedFn =
  (tabName, req) =>
    () => {
      $id(tabName).innerHTML = ""
      $id(tabName).insertAdjacentHTML('afterbegin', req.response)

      // Restore the current tab's values using the function defined in main.js that in turn, is based on the
      // availability of local storage.  If local storage is not available, then this function evaluates to no_op
      window.restoreTabValues(tabName)

      // Call WASM to update the screen and then replace the mousemove handler for the canvas overlay
      updateScreenAndMouseHandler(tabName)
    }

const tabLoaded = traceFnBoundary("tabLoaded", tabLoadedFn)


// *********************************************************************************************************************
// Cache values from the current tab into local storage
// *********************************************************************************************************************
const cacheValuesFn =
  () => {
    for (var tablink of $class("tabButton")) {
      if (tablink.className.indexOf("active") > -1) {
        window.store_tab_values(tablink.id.replace("tab_button_", ""))
      }
    }
  }

// *********************************************************************************************************************
// Fetch tab content from server
// *********************************************************************************************************************
const fetchTabFn =
  tabName =>
    (req => {
      req.open('GET',`./tabs/${tabName}.html`)
      req.onload = tabLoaded(tabName, req)
      req.send()
    })
    (new XMLHttpRequest())

// *********************************************************************************************************************
// This function must be called every time an input value is changed
// *********************************************************************************************************************
const updateScreenFn =
  tabName => {
    // Perform any unit conversions that might be needed for the UI, then extract the input values relevant for the
    // current WASM function
    let wasmArgObj = tabConfig[tabName]
      .reduce((acc, field) => {
          showAndConvertUnits(field)
          // Force all field values to be strings otherwise Rust panics when unwrapping the results of the call to
          // function into_serde()
          return field.isWasmArg ? setProperty(acc, field.id, field.getter(field.id) + "") : acc
        }, {})

    // The air pressure and temperature values on the configuration tab are common to all calculations and must
    // therefore always be merged into the argument object passed to WASM
    if (tabName !== "configuration") {
      wasmArgObj = {...wasmArgObj, ...(window.getConfigTabValues()) }
    }

    // What are we sending to WASM?
    traceInfo("updateScreenFn")(`Passing ${JSON.stringify(wasmArgObj)} to WASM function ${tabName}`)

    // WASM does its magic unless the configuration tab is selected, in which case window[tabName] resolves to calling
    // function no_op()
    return window[tabName](wasmArgObj)
}

// *********************************************************************************************************************
// Update the graph by calling the required WASM function.
// This function is called either:
//  1) When a tab is selected, or
//  2) The user changes the octave subdivisions
//  3) The screen width is resized, or
//  4) The device diagram is hidden/revealed
//
//  In all three cases, the graph is redrawn; but in the first two cases, it is due to the fact that the number of plot
//  points on the graph has changed.  This also requires the mousemove handler for the canvas overlay to be replaced
//
//  In the last case, the graph must be redrawn because the canvas size has changed
// *********************************************************************************************************************
const updateScreenAndMouseHandlerFn =
  tabName =>
    (wasm_response => {
      // If the WASM function returns an array, then there has been a validation error with one or more of the arguments
      if (isArray(wasm_response)) {
        console.error(JSON.stringify(wasm_response, null, 2))
      }
      // If the non-null wasm_response is an object containing the property "series_data", then a graph has been plotted
      // and we are getting the chart data back
      else if (isNotNullOrUndef(wasm_response) && wasm_response.series_data) {
        // For all tabs except configuration, invert the structure of the wasm_response.series_data array and pass the
        // result to the canvas overlay mousemove handler
        // The chart_box property defines the bounding box within which the cross hairs should appear
        if (tabName !== "configuration") {
          $id(GRAPH_OVERLAY).onmousemove = canvasMouseOverHandler(
            $id(GRAPH_OVERLAY)
          , wasm_response.chart_box
          , invertPlotData(wasm_response.series_data)
          )
        }
      }
      else {
        if (isNotNullOrUndef(wasm_response))
          console.warn(`That's weird - got the unexpected value "${wasm_response}" back from WASM`)
      }
    })
    // Call WASM to update the graph
    (updateScreen(tabName))



// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



// *********************************************************************************************************************
// UI slider range limitation
// *********************************************************************************************************************

// Modifier functions for use with the limitMax() function
const half   = val => val / 2.0
const double = val => val * 2.0

// Restrict the maximum value of the target UI element to the current value of the source element after applying some
// modifier function to it.  Defaults to the idiot function if no modifier function is supplied
const limitMax =
  (srcEl, targetEl, upperLimit, modifierFn) =>
    (modFn => $id(targetEl).max = Math.min(modFn(srcEl.value), upperLimit))
    (modifierFn || idiot)


// *********************************************************************************************************************
// Wrap private API functions in boundary trace functionality then expose as public API
// *********************************************************************************************************************
const openTab                     = traceFnBoundary("openTab",                     openTabFn)
const cacheValues                 = traceFnBoundary("cacheValues",                 cacheValuesFn)
const fetchTab                    = traceFnBoundary("fetchTab",                    fetchTabFn)
const updateScreen                = traceFnBoundary("updateScreen",                updateScreenFn)
const updateScreenAndMouseHandler = traceFnBoundary("updateScreenAndMouseHandler", updateScreenAndMouseHandlerFn)

export {
  limitMax
, half
, double
, openTab
, cacheValues
, fetchTab
, updateScreen
, updateScreenAndMouseHandler
}
