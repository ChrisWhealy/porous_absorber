/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import * as LS from "./local_storage.js"
import * as TM from "./tab_manager.js"

import { no_op }  from "./utils.js"
import { $class } from "./dom_access.js"

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

// *********************************************************************************************************************
// Make the tab's various onclick and oninput functions available at the window level
window.open_tab      = TM.open_tab
window.update_screen = TM.update_screen
window.limit_max     = TM.limit_max
window.half          = TM.half
window.double        = TM.double


// *********************************************************************************************************************
// Activate configuration tab and select default tab
async function start_tabs() {
  const trace_bnd = trace_boundary("start_tabs")
  trace_bnd(true)

  let can_i_haz_local_storage = LS.storage_available("localStorage")

  // Define which function is called based on the availability of local storage
  window.restore_tab_values  = can_i_haz_local_storage ? LS.restore_from_local_storage : no_op
  window.store_tab_values    = can_i_haz_local_storage ? LS.write_to_local_storage     : no_op
  window.clear_local_storage = can_i_haz_local_storage ? LS.clear_local_storage        : no_op
  window.get_config          = can_i_haz_local_storage ? TM.fetch_config_values        : TM.fetch_config_from_dom

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
  start_tabs()
}

// *********************************************************************************************************************
// Make the JavaScript wrapper functions accessible to coding outside this module
window.rb_porous_absorber    = rb_porous_absorber
window.slotted_panel         = slotted_panel
window.perforated_panel      = perforated_panel
window.microperforated_panel = microperforated_panel
window.configuration         = no_op

start_wasm()
