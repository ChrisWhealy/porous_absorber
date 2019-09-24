/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Tab management functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

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
    (modFn => $(targetEl).max = Math.min(modFn(srcEl.value), upperLimit))
    (modifierFn || idiot)


// *********************************************************************************************************************
// Tab management
// *********************************************************************************************************************
const openTab = (evt, tabName) => {
  trace("---> openTab()")

  // Cache values from current tab and deactive that tab button
  cache_values_and_deactivate()
  hide_and_empty_all_tabs()

  // Make the selected tab button active
  evt.currentTarget.className += " active"
  $(tabName).style.display = "block"
  
  fetch_tab(tabName)

  trace("<--- openTab()")
}

// *********************************************************************************************************************
// Hide tabs and remove their content except for the configuration tab
const hide_and_empty_all_tabs =
  () => {
    trace("---> hide_all_tabs()")

    for (var tab of $class("tabContent")) {
      tab.style.display = "none"

      if (tab.id !== "configuration") {
        tab.innerHTML = ""
      }
    }

    trace("<--- hide_all_tabs()")
  }

// *********************************************************************************************************************
// Cache values from the current tab in local storage, then deactivate the tab button
const cache_values_and_deactivate =
  () => {
    trace("---> cache_values_and_deactivate()")

    for (var tablink of $class("tabButton")) {
      if (tablink.className.indexOf("active") > -1) {
        tablink.className = tablink.className.replace(" active", "")
        store_tab_values(tablink.id.replace("tab_button_", ""))
      }
    }

    trace("<--- cache_values_and_deactivate()")
  }

// *********************************************************************************************************************
// Fetch tab content from server
const fetch_tab =
  tabName => {
    trace(`---> fetch_tab(${tabName})`)
    let req = new XMLHttpRequest()
    
    req.open('GET',`./tabs/${tabName}.html`)
    
    req.onload = () => {
      trace(`     Inserting HTML for ${tabName}`)
      $(tabName).innerHTML = ""
      $(tabName).insertAdjacentHTML('afterbegin', req.response)

      // If local storage contains any values for the current tab's fields, then restore these
      restore_tab_values(tabName)

      update_screen(tabName)
    }
    
    req.send()
    trace(`<--- fetch_tab(${tabName})`)
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

const fetch_config_from_dom = () => [$("air_temp").value, $("air_pressure").value]

const fetch_config_values =
  () =>
    !!window.localStorage.getItem("configuration")
    ? fetch_config_from_ls()
    : fetch_config_from_dom()

// *********************************************************************************************************************
// This function must be called every time an input value is changed
const update_screen =
  tabName => {
    trace(`---> update_screen(${tabName})`)
    
    // Perform any unit conversions that might be needed then extract the input values relevant for the WASM module
    let current_field_values = tabConfig[tabName]
      .reduce((acc, field) => {
          show_and_convert_units(field)
          return field.isWasmArg ? push(field.getter(field.id), acc) : acc
        }, [])

    // For non-configuration tabs, add the configuration values since these are common to all calculations
    if (tabName !== "configuration") {
      current_field_values = current_field_values.concat(get_config())
    }

    // What are we sending to WASM?
    trace(`Passing ${current_field_values} to WASM function ${tabName}`)

    // WASM does its magic...
    // If the configuration tab is selected, then window[tabName] resolves to no_op
    let wasm_response = window[tabName].apply(null, current_field_values) || "Ok"
    
    // Did it work?
    if (wasm_response !== "Ok") {
      console.log(JSON.stringify(wasm_response, null, 2))
    }
    
    trace(`<--- update_screen(${tabName})`)
}

