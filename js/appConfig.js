/***********************************************************************************************************************
 * Porous Absorber Calculator App Configuration
 * 
 * Provide a single location for all application configuration:
 * 
 * 1) All module debug flags
 * 2) Minimum canvas width in pixels
 * 
 * This module exposes the traceFnBoundary and traceInfo functions as properties of the defineTrace object.
 * If any of the trace flags are switched on for modules listed below, traceFnBoundary and traceInfo will write trace
 * information to the browser console, otherwise they evaluate to a call to no_op()
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

import { doTraceFnBoundary, doTraceInfo } from "./trace.js"

const JS_MODULE_TRACE_FLAGS = {
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
// Function defineTrace acts as the API to the trace functionality that is [en|dis]abled using the trace flags defined
// above in JS_MODULE_TRACE_FLAGS
//
// It is not necessary to import trace.js anywhere else in the code.  All other modules generate their own versions of
// functions traceFnBoundary and traceInfo by simply calling defineTrace passing in their own name
//
// Usage
//
// 1) In any given module first:
//
//    import defineTrace from "./appConfig.js"
//
// 2) Call defineTrace() passing in the module name.  E.G. for module "main":
//
//    const { traceFnBoundary, traceInfo } = defineTrace("main")
//    
//    This call returns an object containing two properties called traceFnBoundary and traceInfo
//    These properties are functions whose implementation varies depending on the Boolean trace values listed in the
//    JS_MODULE_TRACE_FLAGS object above
//
//    2.1) If the trace flag for module "main" is true, then traceFnBoundary will become the function
//
//         (fnName, fn) =>
//           (...args) => {
//             writeTraceText(arrow(true), modName, fnName)
//             let retVal = fn.apply(null, args)
//             writeTraceText(arrow(false), modName, fnName)
//             return retVal
//           }
//
//         Where functions "writeTraceText" and "arrow" are defined internally within module "trace.js"
//     
//         Otherwise, traceFnBoundary becomes the function
//     
//         (_, fn) => fn.apply(null, args)
//     
//    2.2) The two arguments to function traceBoundary() are:
//
//         * A string holding the name of the function within module "main" whose boundary crossings are to be logged
//         * The function being wrapped
//
//    2.3) Similarly, if the trace flag for module "main" is true, then traceInfo will become the function
//
//         fnName => (...args) => writeTraceText("    ", modName, fnName, args.join(", "))
//
//         Otherwise, traceInfo becomes a wrapper function around the no_op() function defined in the util.js module
//
//         () => () => no_op()
//
// 3) For any function within module "main" whose boundary crossings (entry and exit points) need to logged:
//
//    3.1) Write that function as you normally would but add some arbitrary label to the end of the function name.
//         My convention is to add the suffix "Fn" to the application function name:
//
//         const doSomethingFn =
//           (arg1, arg2) => {
//             // snip...
//           }
//
//    3.2) Call traceFnBoundary() passing in the two arguments described above.  This will give you back a function
//         containing your application logic plus the additional trace functionality
//
//         const doSomething = traceFnBoundary("doSomething", doSomethingFn)
//
//    3.3) When the trace flag is switched on, calling function doSomething() will then write the following entries to
//         the browser console:
//
//         ---> main.doSomething()
//         <--- main.doSomething()
//
// 4) Additionally, within the application function doSomething(), you might want to log runtime data
//
//    4.1) Within the coding of function doSomethingFn, create a function called trace by calling function traceInfo()
//         passing in the name of the current function.  Function doSomethingFn() would now look like this:
//
//         const doSomethingFn =
//           (arg1, arg2) => {
//             const trace = traceInfo("doSomething")
//        
//             trace(`arg1 = "${arg1}", arg2 = "${arg2}"`)
//            
//             // snip...
//           }
//
//          const doSomethingFn = traceFnBoundary("doSomething", doSomethingFn)
//
// 6) Now, a call to doSomething() will produce the following in the browser console
//
//   doSomething("First arg value", "Second arg value")
//
//    ---> main.doSomething()
//         main.doSomething() : arg1 = "First arg value", arg2 = "Second arg value"
//    <--- main.doSomething()
//
// *********************************************************************************************************************
const defineTrace =
  modName => ({
    "traceFnBoundary" : doTraceFnBoundary(JS_MODULE_TRACE_FLAGS[modName], modName)
  , "traceInfo"       :       doTraceInfo(JS_MODULE_TRACE_FLAGS[modName], modName)
  })

export {
  defineTrace as default
, MIN_CANVAS_WIDTH
}
