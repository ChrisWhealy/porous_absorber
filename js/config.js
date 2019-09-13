/***********************************************************************************************************************
 * Porous Absorber Calculator Configuration
 * 
 * (c) Chris Whealy 2019
 **********************************************************************************************************************/

 /***********************************************************************************************************************
  * The tabConfig object contains one property per tabName
  * Each tabName property is an object that holds firstly a metadata array of the HTML elements passed to the WASM
  * module, and secondly, the name of the WASM function that calculates that particular absorption curve
  * 
  * The value of the "id" property below must match the corresponding id of the input field in the DOM
  *
  * The value of "units" property is needed primarily for metric to imperial conversion but is maintained for all
  * values for consistency and potential future use
  *
  * The "fetch" property is set to the function name needed to read the input value from the corresponding HTML element
  *
  * The WASM moduile returns either the success value "Ok" or an array of error messages
  **********************************************************************************************************************/
 const tabConfig = {
     "rb_porous_absorber" : [
       { id : "absorber_thickness_mm", units : "mm",      fetch : fetchInt      }
     , { id : "flow_resistivity",      units : "rayls/m", fetch : fetchInt      }
     , { id : "air_gap_mm",            units : "mm",      fetch : fetchInt      }
     , { id : "angle",                 units : "°",       fetch : fetchInt      }
     , { id : "start_graph_freq",      units : "Hz",      fetch : fetchFloat    }
     , { id : "smooth_curve",          units : "each",    fetch : fetchCheckbox }
     , { id : "subdivision",           units : "each",    fetch : fetchRadio    }
     , { id : "air_temp",              units : "°C",      fetch : fetchInt      }
     , { id : "air_pressure",          units : "bar",     fetch : fetchFloat    }
     ]
   , "slotted_panel" : [
   ]
   , "perforated_panel" : [
    //    { id : "panel_thickness_mm",    units : "mm",      fetch : fetchFloat }
    //  , { id : "repeat_distance_mm",    units : "mm",      fetch : fetchFloat }
    //  , { id : "hole_radius_mm",        units : "mm",      fetch : fetchFloat }
    //  , { id : "open_area",             units : "%",       fetch : fetchFloat }
    //  , { id : "cavity_depth_mm",       units : "mm",      fetch : fetchInt   }
    //  , { id : "absorber_thickness_mm", units : "mm",      fetch : fetchInt   }
    //  , { id : "air_gap_mm",            units : "mm",      fetch : fetchInt   }
    //  , { id : "flow_resistivity",      units : "rayls/m", fetch : fetchInt   }
    //  , { id : "air_temp",              units : "°C",      fetch : fetchInt   }
    //  , { id : "air_pressure",          units : "bar",     fetch : fetchFloat }
    //  , { id : "start_graph_freq",      units : "Hz",      fetch : fetchFloat }
    //  , { id : "subdivision",           units : "each",    fetch : fetchRadio }
   ]
   , "microperforated_panel" : [
   ]
   , "configuration" : [
   ]
 }
 
