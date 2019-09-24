/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Local storage utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

// *********************************************************************************************************************
// Check if local storage is available
// This function cannot be run before the HTML page has fully initialised
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
      return e instanceof DOMException && (
        // everything except Firefox
        e.code === 22 ||
        // Firefox
        e.code === 1014 ||
        // test name field too, because code might not be present
        // everything except Firefox
        e.name === 'QuotaExceededError' ||
        // Firefox
        e.name === 'NS_ERROR_DOM_QUOTA_REACHED') &&
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

