/***********************************************************************************************************************
 * Porous Absorber Calculator
 * 
 * Basic utility functions
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

// The "do nothing" function
const no_op = () => {}

// Identity function (which, according to Dave Keenan, should more appropriately be called the idiot function.
// See http://dkeenan.com/Lambda/ for details)
const idiot = val => val

// A version of Array.push that can be used in a single expression function such as Array.map()
const push = (el, arr) => (_ => arr)(arr.push(el))

// Set an object property value and returnd the updated object.  Needed for use in single expression functions
const setProperty = (obj, propName, propVal) => (_ => obj)(obj[propName] = propVal)


