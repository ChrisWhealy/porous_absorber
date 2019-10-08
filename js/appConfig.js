/***********************************************************************************************************************
 * Porous Absorber Calculator App Configuration
 * 
 * All module debug flags are defined here
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { do_trace_boundary, do_trace_info } from "./trace.js"

const DEBUG_MODULE = {
  "dom_access"      : false
, "local_storage"   : false
, "main"            : false
, "tab_manager"     : false
, "unit_conversion" : false
}

const define_trace =
  modName => ({
    "trace_boundary" : do_trace_boundary(DEBUG_MODULE[modName])(modName)
  , "trace_info"     : do_trace_info(DEBUG_MODULE[modName])(modName)
  })


export {
  define_trace
}