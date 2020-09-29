/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * Local storage utility functions
 *
 * (c) Chris Whealy 2020
 **********************************************************************************************************************/

import tabConfig from "./tabConfig.js"

import {
  setProperty
, isNull
, isNullOrUndef
} from "./utils.js"

// *********************************************************************************************************************
// Define trace functions
// *********************************************************************************************************************
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
      let thisConfig = tabConfig[tabName]

      JSON.parse(tabValueStr).map((field, idx) => {
        trace(`${field.id} = ${field.value}`)
        thisConfig[idx].setter(field.id, field.value)
      })
    }
  }


// *********************************************************************************************************************
const writeToLocalStorageFn =
  tabName => {
    let cacheVals = tabConfig[tabName].map(field => ({
        "id"    : field.id
      , "value" : isNullOrUndef(field.getter(field.id)) ? field.default : field.getter(field.id)
      })
    )

      traceInfo("writeToLocalStorage")(`Writing ${JSON.stringify(cacheVals)} to local storage`)
      window.localStorage.setItem(tabName, JSON.stringify(cacheVals))
    }

// *********************************************************************************************************************
const clearLocalStorageFn =
  () =>
    (keyCount => {
      Object.keys(tabConfig).map(tab => window.localStorage.removeItem(tab))
      alert(`Cached data for ${keyCount} tabs has been removed from local storage`)
    })
    (Object.keys(tabConfig).length)



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
      ( // Firefox returns error code 1014, everyone else returns 22
        e.code === 1014 || e.code === 22 ||
        // The "code" property can sometimes be is missing, so also test "name"
        // Firefox use 'NS_ERROR_DOM_QUOTA_REACHED', everyone else uses 'QuotaExceededError'
        e.name === 'NS_ERROR_DOM_QUOTA_REACHED' || e.name === 'QuotaExceededError'
      ) &&
      // Acknowledge QuotaExceededError only if there's something already stored
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
      // Force all field values to be strings otherwise Rust panics when it tries to unwrap the results of the call to
      // function into_serde()
      .reduce((acc, field) => setProperty(acc, field.id, field.value + ""), {})


// *********************************************************************************************************************
// Wrap private API functions in boundary trace functionality then expose as public API
// *********************************************************************************************************************
const restoreFromLocalStorage = traceFnBoundary("restoreFromLocalStorage", restoreFromLocalStorageFn)
const writeToLocalStorage     = traceFnBoundary("writeToLocalStorage",     writeToLocalStorageFn)
const clearLocalStorage       = traceFnBoundary("clearLocalStorage",       clearLocalStorageFn)

export {
  storageAvailable
, restoreFromLocalStorage
, writeToLocalStorage
, clearLocalStorage
, fetchConfigTabValues
}
