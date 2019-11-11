/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Local storage utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import tabConfig from "./tabConfig.js"

import {
  setProperty
, isNull
, isNullOrUndef
} from "./utils.js"

// *********************************************************************************************************************
// Define trace functions
import defineTrace from "./appConfig.js"
const { traceFnBoundary, traceInfo } = defineTrace("localStorage")


// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                 P R I V A T E   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************


// *********************************************************************************************************************
const restoreFromLocalStorageFn =
  tabName => {
    const trace = traceInfo("restoreFromLocalStorage")

    let tabValueStr = window.localStorage.getItem(tabName)

    if (isNull(tabValueStr)) {
      trace(`No values for ${tabName} found in local storage`)
    }
    else {
      trace(`Cached values for tab "${tabName}" found in local storage`)
      let thisConfig = tabConfig[tabName]
  
      JSON.parse(tabValueStr).map((field, idx) => {
        trace(`     ${field.id}=${field.value}`)
        thisConfig[idx].setter(field.id, field.value)
      })
    }
  }

  
// *********************************************************************************************************************
const writeToLocalStorageFn =
  tabName => {
    let cacheVals = tabConfig[tabName].map(
      field =>
      isNullOrUndef(field.getter(field.id))
      ? { "id" : field.id, "value" : field.default }
      : { "id" : field.id, "value" : field.getter(field.id) }
      )
      
      traceInfo("writeToLocalStorage")(`Writing ${JSON.stringify(cacheVals)} to local storage`)
      window.localStorage.setItem(tabName, JSON.stringify(cacheVals))
    }
    
// *********************************************************************************************************************
const clearLocalStorageFn =
  () => {
    let keyCount = Object.keys(tabConfig).length
    Object.keys(tabConfig).map(tab => window.localStorage.removeItem(tab))
    alert(`All cached data for ${keyCount} tabs has been removed from local storage`)
  }



// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



// *********************************************************************************************************************
// Check if local storage is available
// Warning: This function cannot be run before the HTML page has fully initialised!
// *********************************************************************************************************************
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
// Fetch the air temperature and pressure config values from local storage
// *********************************************************************************************************************
const fetchConfigTabValues =
  () =>
    JSON
      .parse(window.localStorage.getItem("configuration"))
      // Force all field values to be strings otherwise Rust panics when tries to unwrap the results of the call to
      // function into_serde()
      .reduce((acc, field) => setProperty(acc, field.id, field.value + ""), {})


// *********************************************************************************************************************
// Wrap private API functions in boundary trace functionality then expose as public API
// *********************************************************************************************************************
const restoreFromLocalStorage = traceFnBoundary("restoreFromLocalStorage", null, restoreFromLocalStorageFn)
const writeToLocalStorage     = traceFnBoundary("writeToLocalStorage",     null, writeToLocalStorageFn)
const clearLocalStorage       = traceFnBoundary("clearLocalStorage",       null, clearLocalStorageFn)

export {
  storageAvailable
, restoreFromLocalStorage
, writeToLocalStorage
, clearLocalStorage
, fetchConfigTabValues
}
