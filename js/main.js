/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import * as LS from "./local_storage.js"
import * as TM from "./tab_manager.js"

import { no_op }       from "./utils.js"
import { $id, $class } from "./dom_access.js"

import {
  setCanvasSize
, GRAPH
, GRAPH_OVERLAY
} from "./canvas.js"

// JavaScript wrappers for WASM functions
import init
, { rb_porous_absorber
  , slotted_panel
  , perforated_panel
  , microperforated_panel
} from '../pkg/porous_absorber_calculator.js'

// *********************************************************************************************************************
// Define trace functions
import { define_trace } from "./appConfig.js"
const { trace_boundary, trace_info } = define_trace("main")

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
// Make the tab's various onclick and oninput functions available at the window level
window.openTab                     = TM.openTab
window.updateScreen                = TM.updateScreen
window.updateScreenAndMouseHandler = TM.updateScreenAndMouseHandler
window.limitMax                    = TM.limitMax
window.half                        = TM.half
window.double                      = TM.double

// Make the WASM wrapper functions accessible
window.rb_porous_absorber    = rb_porous_absorber
window.slotted_panel         = slotted_panel
window.perforated_panel      = perforated_panel
window.microperforated_panel = microperforated_panel
window.configuration         = no_op

startWASM()



// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************


// *********************************************************************************************************************
// Define the use of local storage
function useLocalStorage() {
  const trace_bnd = trace_boundary("useLocalStorage")
  const trace     = trace_info("useLocalStorage")
  trace_bnd(true)

  let can_i_haz_local_storage = LS.storageAvailable("localStorage")

  trace(`Local storage is${can_i_haz_local_storage ? " " : " not "}available`)

  // If local storage is availabel, then we must check that it has been populated with the configuration tab values
  if (can_i_haz_local_storage) {
    LS.writeToLocalStorage("configuration")
  }

  // Define which function is called based on the availability of local storage
  window.restoreTabValues   = can_i_haz_local_storage ? LS.restoreFromLocalStorage : no_op
  window.store_tab_values   = can_i_haz_local_storage ? LS.writeToLocalStorage     : no_op
  window.clearLocalStorage  = can_i_haz_local_storage ? LS.clearLocalStorage       : no_op
  window.getConfigTabValues = can_i_haz_local_storage ? LS.fetchConfigTabValues    : fetchConfigFromDom

  trace_bnd(false)
}

// *********************************************************************************************************************
// Fetch config values from DOM
const fetchConfigFromDom = () => [$id("air_temp").value, $id("air_pressure").value]

// *********************************************************************************************************************
// Activate configuration and default tabs
async function startTabs() {
  const trace_bnd = trace_boundary("startTabs")
  trace_bnd(true)
  
  // Ensure the configuration tab is always loaded
  await TM.fetchTab("configuration")

  // Select the default tab
  for (var tablink of $class("tabButton")) {
    if (tablink.getAttribute("default") === "true") tablink.click()
  }

  trace_bnd(false)
}

// *********************************************************************************************************************
// Initialise the Web Assembly module, then start the tabs containing each absorber device type
async function startWASM() {
  await init()
  console.log("WASM module initialisation complete...")
  
  useLocalStorage()
  await startTabs()
}

