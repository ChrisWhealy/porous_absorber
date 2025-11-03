/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * (c) Chris Whealy 2019, 2025
 **********************************************************************************************************************/

import * as LS from "./localStorage.js"
import * as TM from "./tabManager.js"

import { no_op } from "./utils.js"
import { $id, $class, fetchConfigFromDom } from "./domAccess.js"
import { setCanvasSize, GRAPH, GRAPH_OVERLAY } from "./canvas.js"

// *********************************************************************************************************************
// JavaScript wrapper functions for the underlying WASM functions
//
// Other than the default function name "init", the other function names are the names of the Rust functions exposed
// using the #[wasm_bindgen] directive in lib.rs.  These names, in turn, must match the tab names listed in the
// tabConfig object in tabConfig.js
import init, {
  rb_porous_absorber, slotted_panel, perforated_panel, microperforated_panel
} from '../pkg/porous_absorber_calculator.js'

import defineTrace from "./appConfig.js"
const { traceFnBoundary, traceInfo } = defineTrace("main")

// *********************************************************************************************************************
// Define canvas size based on current window size
window.onload = () => [GRAPH, GRAPH_OVERLAY].map(elName => setCanvasSize($id(elName)))

window.onresize = () => {
  [GRAPH, GRAPH_OVERLAY].map(elName => setCanvasSize($id(elName)))

  // Cache the current parameter values
  TM.cacheValues()

  // Rebuild the active tab
  for (var tablink of $class("tabButton")) {
    if (tablink.className.search("active") > -1)
      TM.updateScreenAndMouseHandler(tablink.id.replace("tab_button_", ""))
  }
}

// *********************************************************************************************************************
// Define the use of local storage
const useLocalStorageFn = () => {
  let can_i_haz_local_storage = LS.storageAvailable("localStorage")

  traceInfo("useLocalStorage")(`Local storage is${can_i_haz_local_storage ? " " : " not "}available`)

  // If local storage is available, then we must check that it has been populated with the configuration tab values
  if (can_i_haz_local_storage) {
    LS.writeToLocalStorage("configuration")
  }

  // Define which function is called based on the availability of local storage
  window.restoreTabValues = can_i_haz_local_storage ? LS.restoreFromLocalStorage : no_op
  window.store_tab_values = can_i_haz_local_storage ? LS.writeToLocalStorage : no_op
  window.clearLocalStorage = can_i_haz_local_storage ? LS.clearLocalStorage : no_op
  window.getConfigTabValues = can_i_haz_local_storage ? LS.fetchConfigTabValues : fetchConfigFromDom
}

const useLocalStorage = traceFnBoundary("useLocalStorage", useLocalStorageFn)

// *********************************************************************************************************************
// Activate configuration and default tabs
const startTabsFn = async () => {
  // Ensure the configuration tab is always loaded
  await TM.fetchTab("configuration")

  // Select the default tab
  for (var tablink of $class("tabButton")) {
    if (tablink.getAttribute("default") === "true") tablink.click()
  }
}

const startTabs = traceFnBoundary("startTabs", startTabsFn)

// *********************************************************************************************************************
// Initialise the Web Assembly module, then start the tabs containing each absorber device type
const startWASM = async () => {
  await init()
  console.log("WASM module initialisation complete...")

  useLocalStorage()
  await startTabs()
}

// *********************************************************************************************************************
// Make the tab's various onclick and oninput functions available at the window level
window.openTab = TM.openTab
window.updateScreen = TM.updateScreen
window.updateScreenAndMouseHandler = TM.updateScreenAndMouseHandler
window.limitMax = TM.limitMax
window.half = TM.half
window.double = TM.double
window.toggleLockAbsDims = TM.toggleLockAbsDims
window.lockAbsDims = {
  'rb_porous_absorber': null,
  'slotted_panel': null,
  'perforated_panel': null,
}

// Make the WASM wrapper functions globally accessible
window.rb_porous_absorber = rb_porous_absorber
window.slotted_panel = slotted_panel
window.perforated_panel = perforated_panel
window.microperforated_panel = microperforated_panel
window.configuration = no_op

startWASM()
