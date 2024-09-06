/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * (c) Chris Whealy 2020
 **********************************************************************************************************************/

import * as LOC_STOR from "./localStorage.js"
import * as TAB_MAN from "./tabManager.js"
import { no_op } from "./utils.js"
import { $id, $class, fetchConfigFromDom } from "./domAccess.js"

import {setCanvasSize, GRAPH, GRAPH_OVERLAY } from "./canvas.js"

// *********************************************************************************************************************
// JavaScript wrapper functions for the underlying WASM functions
//
// Other than the default function name "init", the other function names are the names of the Rust functions exposed
// using the #[wasm_bindgen] directive in lib.rs.  These names, in turn, must match the tab names listed in the
// tabConfig object in tabConfig.js
import init, {
  porous_absorber,
  slotted_panel,
  perforated_panel,
  microperforated_panel
} from '../pkg/porous_absorber_calculator.js'

import defineTrace from "./appConfig.js"

const { traceFnBoundary, traceInfo } = defineTrace("main")

// *********************************************************************************************************************
// Define canvas size based on current window size
window.onload = () => [GRAPH, GRAPH_OVERLAY].map(elName => setCanvasSize($id(elName)))

window.onresize = () => {
  [GRAPH, GRAPH_OVERLAY].map(elName => setCanvasSize($id(elName)))

  // Cache the current parameter values
  TAB_MAN.cacheValues()

  // Rebuild the active tab
  for (let tablink of $class("tabButton")) {
    if (tablink.className.search("active") > -1)
      TAB_MAN.updateScreenAndMouseHandler(tablink.id.replace("tab_button_", ""))
  }
}

// *********************************************************************************************************************
// Make the tab's various onclick and oninput functions available at the window level
window.openTab                     = TAB_MAN.openTab
window.updateScreen                = TAB_MAN.updateScreen
window.updateScreenAndMouseHandler = TAB_MAN.updateScreenAndMouseHandler
window.limitMax                    = TAB_MAN.limitMax
window.half                        = TAB_MAN.half
window.double                      = TAB_MAN.double

// Make the WASM wrapper functions globally accessible
window.porous_absorber       = porous_absorber
window.slotted_panel         = slotted_panel
window.perforated_panel      = perforated_panel
window.microperforated_panel = microperforated_panel
window.configuration         = no_op

// Need to ignore the promise returned here otherwise an attempt will be made to access local storage before the page
// has initialised
startWASM()

// *********************************************************************************************************************
// Define the use of local storage
function useLocalStorageFn() {
  let loc_stor_available = LOC_STOR.storageAvailable("localStorage")

  traceInfo("useLocalStorage")(`Local storage is${loc_stor_available ? " " : " not "}available`)

  // If local storage is available, then we must check that it has been populated with the configuration tab values
  if (loc_stor_available) {
    LOC_STOR.writeToLocalStorage("configuration")
  }

  // Define which function is called based on the availability of local storage
  window.restoreTabValues   = loc_stor_available ? LOC_STOR.restoreFromLocalStorage : no_op
  window.store_tab_values   = loc_stor_available ? LOC_STOR.writeToLocalStorage     : no_op
  window.clearLocalStorage  = loc_stor_available ? LOC_STOR.clearLocalStorage       : no_op
  window.getConfigTabValues = loc_stor_available ? LOC_STOR.fetchConfigTabValues    : fetchConfigFromDom
}

const useLocalStorage = traceFnBoundary("useLocalStorage", useLocalStorageFn)

// *********************************************************************************************************************
// Activate configuration and default tabs
async function startTabsFn() {
  // Ensure the configuration tab is always loaded
  await TAB_MAN.fetchTab("configuration")

  // Select the default tab
  for (let tablink of $class("tabButton")) {
    if (tablink.getAttribute("default") === "true") tablink.click()
  }
}

const startTabs = traceFnBoundary("startTabs", startTabsFn)

// *********************************************************************************************************************
// Initialise the Web Assembly module, then start the tabs containing each absorber device type
async function startWASM() {
  await init()
  console.log("WASM module initialisation complete...")

  useLocalStorage()
  await startTabs()
}
