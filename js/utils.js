/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Basic utilities module
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

// The "do nothing" function
const no_op = () => {}

// Identity function which, according to Dave Keenan, should more appropriately be called the idiot function.
// See http://dkeenan.com/Lambda/ for details
const idiot = val => val

// Set an object property value and returnd the updated object.  Needed for use in single expression functions
const setProperty = (obj, propName, propVal) => (_ => obj)(obj[propName] = propVal)

// Versions of Array.push and Array.unshift that can be used in a single expression function such as Array.map()
const push    = (el, arr) => (_ => arr)(arr.push(el))
const unshift = (el, arr) => (_ => arr)(arr.unshift(el))

// *********************************************************************************************************************
// Discover what data type the object itself thinks it has - as opposed to the data type JavaScript thinks it has
const typeOf = x => Object.prototype.toString.apply(x).slice(8).slice(0, -1)

// Partial function that creates a function to check for a specific data type
const isOfType = t => x => typeOf(x) === t

// Primitive type identifiers
const isNull      = isOfType("Null")
const isUndefined = isOfType("Undefined")
const isNumber    = isOfType("Number")
const isBigInt    = isOfType("BigInt")
const isSymbol    = isOfType("Symbol")
const isArray     = isOfType("Array")
const isMap       = isOfType("Map")
const isSet       = isOfType("Set")
const isString    = isOfType("String")
const isFn        = isOfType("Function")
const isGenFn     = isOfType("GeneratorFunction")
const isJsObject  = isOfType("Object")

// The NodeJS objects 'global' and 'process' return their own names when asked their type even though they are just
// regular objects
const isNodeJsProcess = isOfType("process")
const isNodeJsGlobal  = isOfType("global")

// Disjunctive type identifiers
const isNullOrUndef = x => isNull(x)     || isUndefined(x)
const isNumeric     = x => isNumber(x)   || isBigInt(x)
const isFunction    = x => isFn(x)       || isGenFn(x)
const isObject      = x => isJsObject(x) || isNodeJsProcess(x) || isNodeJsGlobal(x)

const isNotNullOrUndef = x => !isNullOrUndef(x)

// *********************************************************************************************************************
// Invert the structure of the series_data array returned from WASM
// The resulting object contains a property for each X axis value, which in turn, holds an array of one or more Y axis
// values
const invertPlotData =
  plotData =>
    plotData.reduce(
      (acc, seriesData) => {
        seriesData.plot_points.map(
          plotPoint =>
            ((yVals, yInfo) => setProperty(acc, plotPoint.x, yVals ? yVals.concat(yInfo) : [yInfo]))
            (acc[plotPoint.x], removeX(plotPoint))
        )
        return acc
      }
    , {})

// Create a new plot point object that does not include the "x" property
const removeX = plotPoint => ({"y" : plotPoint.y, "freq" : plotPoint.freq, "abs" : plotPoint.abs})

// Inclusive numeric range test
const isBetween = (upper, lower, val) => val >= lower && val <= upper

const isInsideRect =
  (top_left_x, top_left_y, bottom_right_x, bottom_right_y) =>
    mousePos =>
      isBetween(bottom_right_x, top_left_x, mousePos.x) &&
      isBetween(bottom_right_y, top_left_y, mousePos.y)



// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************
export {
  // Fundamental functions
  no_op
, idiot

  // Array handlers
, push
, unshift

  // Datatype identifiers
, typeOf           
, isOfType         
, isArray          
, isBigInt         
, isFunction       
, isMap            
, isNull           
, isNullOrUndef    
, isNotNullOrUndef 
, isNumber         
, isNumeric        
, isObject         
, isSet            
, isString         
, isSymbol         
, isUndefined      

  // Numeric tests
, isBetween
, isInsideRect

  // Object handlers
, setProperty
, invertPlotData
}
