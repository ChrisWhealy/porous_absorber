/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * Trace program execution flow
 * This module only needs to be consumed my module appConfig
 *
 * (c) Chris Whealy 2020
 **********************************************************************************************************************/

import {isNullOrUndef, no_op} from "./utils.js"

const ENTRY_ARROW = "--->"
const EXIT_ARROW = "<---"
const IN_OUT_ARROW = "<-->"

// *********************************************************************************************************************
//                                                 P R I V A T E   A P I
// *********************************************************************************************************************

const writeTraceText =
    (prefix, modName, fnName, txt) =>
        console.log(`${prefix} ${modName}.${fnName}()${isNullOrUndef(txt) ? "" : ` : ${txt}`}`)

const arrow = mayBeBool => isNullOrUndef(mayBeBool) ? IN_OUT_ARROW : mayBeBool ? ENTRY_ARROW : EXIT_ARROW

// *********************************************************************************************************************
//                                                  P U B L I C   A P I
// *********************************************************************************************************************

// *********************************************************************************************************************
// Partial function that, given a trace flag and a module name, returns a partial function that when called with a
// function name, will do one of two things:
//
// * traceActive === true
//   Writes the supplied information to the browser console labelled with module name, function name and trace data
//
// * traceActive === false
//   Returns a function that invokes the unit function (I.E. does absolutely nothing)
// *********************************************************************************************************************
const doTraceInfo =
    (traceActive, modName) =>
        traceActive
            ? fnName => (...args) => writeTraceText("    ", modName, fnName, args.join(", "))
            : () => no_op

// *********************************************************************************************************************
// Partial function that, given a trace flag and a module name, returns a partial function that when called with a
// function name, will do one of two things:
//
// * traceActive === true
//   1) Logs crossing a function boundary on entry labelled with the module and function names
//   2) Calls the function supplied as argument fn
//   3) Logs crossing a function boundary on exit labelled with the module and function names
//
// * traceActive === false
//   Simply calls the function supplied in argument fn
//
// See the comments in appConfig.js for a full description of how these generated trace functions are used
// *********************************************************************************************************************
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
    doTraceFnBoundary,
    doTraceInfo
}
