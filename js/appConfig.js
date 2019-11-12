/***********************************************************************************************************************
 * Porous Absorber Calculator App Configuration
 * 
 * Provide a single location for all application configuration:
 * 
 * 1) All module debug flags
 * 2) Minimum canvas width in pixels
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { doTraceFnBoundary, doTraceInfo } from "./trace.js"

const JS_MODULE_DEBUG_FLAGS = {
  "domAccess"      : false
, "localStorage"   : false
, "main"           : false
, "tabManager"     : false
, "unitConversion" : false
}

const MIN_CANVAS_WIDTH = 1000



// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



// *********************************************************************************************************************
// Function defineTrace acts as the API to the trace functionality
//
// It is not necessary to import trace.js anywhere else in the code.  All other modules simply call defineTrace passing
// in their own module name and the traceFnBoundary and traceInfo functions will be generated based on the value of the
// Boolean found in the above JS_MODULE_DEBUG_FLAGS
// *********************************************************************************************************************
const defineTrace =
  modName => ({
    "traceFnBoundary" : doTraceFnBoundary(JS_MODULE_DEBUG_FLAGS[modName], modName)
  , "traceInfo"       :       doTraceInfo(JS_MODULE_DEBUG_FLAGS[modName], modName)
  })

export {
  defineTrace as default
, MIN_CANVAS_WIDTH
}
