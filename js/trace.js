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

const arrow = mayBeBool => isNullOrUndef(mayBeBool) ? IN_OUT_ARROW : mayBeBool ? ENTRY_ARROW : EXIT_ARROW

const do_trace_boundary =
  isDebug =>
    modName =>
      (fnName, argVals) =>
        isEntry =>
          isDebug
          ? console.log(`${arrow(isEntry)} ${modName}.${fnName}(${isNullOrUndef(argVals) ? "" : argVals})`)
          : no_op()

const do_trace_info =
  isDebug =>
    modName =>
      fnName =>
        txt =>
          isDebug
          ? console.log(`     ${modName}.${fnName}() : ${txt}`)
          : no_op()


// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  do_trace_boundary
, do_trace_info
}