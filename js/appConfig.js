/***********************************************************************************************************************
 * Porous Absorber Calculator App Configuration
 * 
 * All module debug flags are defined here
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { do_trace_boundary, do_trace_info } from "./trace.js"

const DEBUG_MODULE = {
  "domAccess"      : false
, "localStorage"   : false
, "main"           : false
, "tabManager"     : false
, "unitConversion" : false
}

const define_trace =
  modName => ({
    "traceBoundary" : do_trace_boundary(DEBUG_MODULE[modName])(modName)
  , "traceInfo"     : do_trace_info(DEBUG_MODULE[modName])(modName)
  })


export {
  define_trace
}