/***********************************************************************************************************************
 * Porous Absorber Calculator App Configuration
 * 
 * All module debug flags are defined here
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { doTraceBoundary, doTraceInfo } from "./trace.js"

const JS_MODULES = {
  "domAccess"      : false
, "localStorage"   : false
, "main"           : false
, "tabManager"     : false
, "unitConversion" : false
}

const define_trace =
  modName => ({
    "traceBoundary" : doTraceBoundary(JS_MODULES[modName])(modName)
  , "traceInfo"     : doTraceInfo(JS_MODULES[modName])(modName)
  })


export {
  define_trace
}