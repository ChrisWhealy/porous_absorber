/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Trace program execution flow
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { no_op, isNullOrUndef } from "./utils.js"

const ENTRY_ARROW  = "--->"
const EXIT_ARROW   = "<---"
const IN_OUT_ARROW = "<-->"

// If arrow() is called with no arguments, then the in/out "<--->" arrow is displayed to indicate that the function
// being called is either just a single expression or contains very simple functionality
const arrow = mayBeBool => isNullOrUndef(mayBeBool) ? IN_OUT_ARROW : mayBeBool ? ENTRY_ARROW : EXIT_ARROW

const doTraceInfo =
  (isDebug, modName) =>
    fnName =>
      txt => isDebug ? writeTraceText("     ", modName, fnName, null, txt)
                      : no_op()

const doTraceFnBoundary =
  (traceActive, modName) =>
    (fnName, argVals, fn) =>
      traceActive
      ? (...args) => {
          writeTraceText(arrow(true), modName, fnName, argVals)
          let retVal = fn.apply(null, args)
          writeTraceText(arrow(false), modName, fnName, argVals)
          return retVal
        }
      : (...args) => fn.apply(null, args)

// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  doTraceFnBoundary
, doTraceInfo
}



// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************

const writeTraceText =
  (prefix, modName, fnName, argVals, txt) =>
    ((args, text) =>
      console.log(`${prefix} ${modName}.${fnName}(${args})${text}`)
    )
    ( isNullOrUndef(argVals) ? "" : argVals
    , isNullOrUndef(txt)     ? "" : ` : ${txt}`
    )

