/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import * as LS from "./local_storage.js"
import * as TM from "./tab_manager.js"

import { no_op }         from "./utils.js"
import { $id, $class }   from "./dom_access.js"
import { updateScreen }  from "./tab_manager.js"
import { setCanvasSize } from "./canvas.js"

// JavaScript wrappers for WASM functions
import init
, { rb_porous_absorber
  , slotted_panel
  , perforated_panel
  , microperforated_panel
}
from '../pkg/porous_absorber_calculator.js'

// *********************************************************************************************************************
// Define trace functions
import { do_trace_boundary, do_trace_info} from "./trace.js"

const MOD_NAME     = "main"
const DEBUG_ACTIVE = false

const trace_boundary = do_trace_boundary(DEBUG_ACTIVE)(MOD_NAME)
const trace_info     = do_trace_info(DEBUG_ACTIVE)(MOD_NAME)

// *********************************************************************************************************************
// Define canvas sized based on current window size
window.onload = () => setCanvasSize($id("graph_canvas"))

window.resize = window.onresize = () => {
  setCanvasSize($id("graph_canvas"))

  // Redraw the active tab
  for (var tablink of $class("tabButton")) {
    if (tablink.className.search("active") > -1)
      updateScreen(tablink.id.replace("tab_button_", ""))
  }
}


// *********************************************************************************************************************
// Make the tab's various onclick and oninput functions available at the window level
window.open_tab      = TM.open_tab
window.updateScreen = TM.updateScreen
window.limit_max     = TM.limit_max
window.half          = TM.half
window.double        = TM.double

// Make the WASM wrapper functions accessible
window.rb_porous_absorber    = rb_porous_absorber
window.slotted_panel         = slotted_panel
window.perforated_panel      = perforated_panel
window.microperforated_panel = microperforated_panel
window.configuration         = no_op

start_wasm()



// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************


// *********************************************************************************************************************
// Define the use of local storage
function use_local_storage() {
  const trace_bnd = trace_boundary("use_local_storage")
  const trace     = trace_info("use_local_storage")
  trace_bnd(true)

  let can_i_haz_local_storage = LS.storage_available("localStorage")

  trace(`Local storage is ${can_i_haz_local_storage ? "" : "not"} available`)

  // Define which function is called based on the availability of local storage
  window.restore_tab_values  = can_i_haz_local_storage ? LS.restore_from_local_storage : no_op
  window.store_tab_values    = can_i_haz_local_storage ? LS.write_to_local_storage     : no_op
  window.clear_local_storage = can_i_haz_local_storage ? LS.clear_local_storage        : no_op
  window.get_config          = can_i_haz_local_storage ? TM.fetch_config_values        : TM.fetch_config_from_dom

  trace_bnd(false)
}

// *********************************************************************************************************************
// Activate configuration and default tabs
async function start_tabs() {
  const trace_bnd = trace_boundary("start_tabs")
  trace_bnd(true)
  
  // Ensure the configuration tab is always loaded
  await TM.fetch_tab("configuration")

  // Select the default tab
  for (var tablink of $class("tabButton")) {
    if (tablink.getAttribute("default") === "true") tablink.click()
  }

  trace_bnd(false)
}

// *********************************************************************************************************************
// Initialise the Web Assembly module, then start the tabs containing each absorber device type
async function start_wasm() {
  await init()
  console.log("WASM module initialisation complete...")
  
  use_local_storage()
  await start_tabs()
}

