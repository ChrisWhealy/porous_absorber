/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Local storage utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { no_op }     from "./utils.js"
import { trace }     from "./trace.js"
import { tabConfig } from "./config.js"

// *********************************************************************************************************************
// Check if local storage is available
// Warning: This function cannot be run before the HTML page has fully initialised!
const storage_available =
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
const restore_from_local_storage =
  tabName => {
    trace(`---> restore_from_local_storage(${tabName})`)
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

    trace(`<--- restore_from_local_storage(${tabName})`)
  }

// *********************************************************************************************************************
const write_to_local_storage =
  tabName => {
    trace(`---> write_to_local_storage(${tabName})`)
    let cacheVals = tabConfig[tabName].map(
      field => ({
        "id"    : field.id
      , "value" : field.getter(field.id)
    }))

    trace(`Writing ${JSON.stringify(cacheVals)} to local storage`)
    window.localStorage.setItem(tabName, JSON.stringify(cacheVals))

    trace(`<--- write_to_local_storage(${tabName})`)
  }

// *********************************************************************************************************************
const clear_local_storage =
  () => {
    trace("---> clear_local_storage()")
    let key_count = Object.keys(tabConfig).length
    Object.keys(tabConfig).map(tab => window.localStorage.removeItem(tab))
    alert(`All cached data for ${key_count} tabs has been removed from local storage`)
    trace("<--- clear_local_storage()")
  }

// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  storage_available
, restore_from_local_storage
, write_to_local_storage
, clear_local_storage
}
