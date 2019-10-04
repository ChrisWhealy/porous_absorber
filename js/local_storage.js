/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Local storage utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { tabConfig } from "./config.js"

// *********************************************************************************************************************
// Define trace functions
import { do_trace_boundary, do_trace_info} from "./trace.js"

const MOD_NAME     = "local_storage"
const DEBUG_ACTIVE = false

const trace_boundary = do_trace_boundary(DEBUG_ACTIVE)(MOD_NAME)
const trace          = do_trace_info(DEBUG_ACTIVE)(MOD_NAME)

// *********************************************************************************************************************
// Check if local storage is available
// Warning: This function cannot be run before the HTML page has fully initialised!
const storageAvailable =
  type => {
    let storage

    try {
      storage = window[type]
      var x = '__storage_test__'
      storage.setItem(x, x)
      storage.removeItem(x)
      return true
    }
    catch(e) {
      return e instanceof DOMException &&
        // everything except Firefox
      ( e.code === 22 ||
        // Firefox
        e.code === 1014 ||
        // test name field too, because code might not be present
        // everything except Firefox
        e.name === 'QuotaExceededError' ||
        // Firefox
        e.name === 'NS_ERROR_DOM_QUOTA_REACHED'
      ) &&
      // acknowledge QuotaExceededError only if there's something already stored
      (storage && storage.length !== 0)
    }
  }

// *********************************************************************************************************************
const restoreFromLocalStorage =
  tabName => {
    const trace_bnd = trace_boundary("restoreFromLocalStorage", tabName)
    trace_bnd(true)

    let tabValueStr = window.localStorage.getItem(tabName)

    if (!!tabValueStr) {
      let thisConfig = tabConfig[tabName]

      JSON.parse(tabValueStr).map((field, idx) => {
        trace(`     ${field.id}=${field.value}`)
        thisConfig[idx].setter(field.id, field.value)
      })
    }
    else {
      trace(`No values for ${tabName} found in local storage`)
    }

    trace_bnd(false)
  }

// *********************************************************************************************************************
const writeToLocalStorage =
  tabName => {
    const trace_bnd = trace_boundary("writeToLocalStorage", tabName)
    trace_bnd(true)

    let cacheVals = tabConfig[tabName].map(
      field => ({
        "id"    : field.id
      , "value" : field.getter(field.id)
    }))

    trace(`Writing ${JSON.stringify(cacheVals)} to local storage`)
    window.localStorage.setItem(tabName, JSON.stringify(cacheVals))

    trace_bnd(false)
  }

// *********************************************************************************************************************
const clearLocalStorage =
  () => {
    const trace_bnd = trace_boundary("clearLocalStorage")
    trace_bnd(true)

    let key_count = Object.keys(tabConfig).length
    Object.keys(tabConfig).map(tab => window.localStorage.removeItem(tab))
    alert(`All cached data for ${key_count} tabs has been removed from local storage`)

    trace_bnd(false)
  }

// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  storageAvailable
, restoreFromLocalStorage
, writeToLocalStorage
, clearLocalStorage
}
