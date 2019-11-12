/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Trace program execution flow
 * This module only needs to be consumed my module appConfig
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { no_op, isNullOrUndef } from "./utils.js"

const ENTRY_ARROW  = "--->"
const EXIT_ARROW   = "<---"
const IN_OUT_ARROW = "<-->"

// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                 P R I V A T E   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************

const writeTraceText =
  (prefix, modName, fnName, txt) =>
    console.log(`${prefix} ${modName}.${fnName}()${isNullOrUndef(txt) ? "" : ` : ${txt}`}`)

const arrow = mayBeBool => isNullOrUndef(mayBeBool) ? IN_OUT_ARROW : mayBeBool ? ENTRY_ARROW : EXIT_ARROW



// *********************************************************************************************************************
// *********************************************************************************************************************
//
//                                                  P U B L I C   A P I
//
// *********************************************************************************************************************
// *********************************************************************************************************************



// *********************************************************************************************************************
// Partial function that, given a trace flag and module name, returns a function that either write the subsequent
// values to the browser console, or does nothing
// *********************************************************************************************************************
const doTraceInfo =
  (traceActive, modName) =>
    traceActive
    ? fnName => (...args) => writeTraceText("    ", modName, fnName, args.join(","))
    : ()     => ()        => no_op()

const doTraceFnBoundary =
  (traceActive, modName) =>
    traceActive
    ? (fnName, fn) =>
        (...args) => {
          writeTraceText(arrow(true), modName, fnName)
          let retVal = fn.apply(null, args)
          writeTraceText(arrow(false), modName, fnName)
          return retVal
        }
    : (_, fn) => (...args) => fn.apply(null, args)



export {
  doTraceFnBoundary
, doTraceInfo
}



