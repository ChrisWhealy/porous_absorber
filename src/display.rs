use std::error::Error;
use std::fmt;
use libm::*;

/***********************************************************************************************************************
 * Graph start frequency and octave subdivision range check values
 */
const START_FREQ   : f64  = 20.0;
const DEFAULT_FREQ : f64  = 62.5;
const END_FREQ     : f64  = 200.0;

const UNITS_FREQ : &str = "Hz";

const SUBDIVISIONS        : [u32; 4] = [1,2,3,6];
const DEFAULT_SUBDIVISION : u32 = 3;

const DISPLAY_OCTAVES : u32 = 8;

/***********************************************************************************************************************
 * Possible errors when creating display struct
 */
enum ErrType {
  Graph
, Subdivision
}

#[derive(Debug)]
pub struct DisplayError {
  msg : String
}

impl DisplayError {
  fn new(err_type: ErrType, err_val: f64) -> DisplayError {
    return match err_type {
      ErrType::Graph => 
        DisplayError {
          msg : format!(
                  "Graph start frequency must be a value in {} between {:?} and {:?}, not '{:?}'"
                , UNITS_FREQ
                , START_FREQ
                , END_FREQ
                , err_val
                )
        }
    , ErrType::Subdivision => 
        DisplayError {
          msg : format!("Octave subdivisions argument must be either 1, 2, 3 or 6, not '{}'", err_val.round())
        }
    };
  }
}

impl fmt::Display for DisplayError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for DisplayError {
  fn description(&self) -> &str {
    &self.msg
  }
}

/***********************************************************************************************************************
 * Generate the required frequencies from the number of display octaves and the subdivisions
 * The range upper bound must be included in order not to omit the 16KHz value
 */
fn gen_frequencies(graph_start_freq : &f64, subdivisions : &u32) -> Vec<f64> {
  let intervals: Vec<u32> = (0..=(DISPLAY_OCTAVES * subdivisions)).collect();

  intervals
    .iter()
    .fold(
      vec!()
    , | mut acc, interval_no | {
        acc.push(
          if interval_no == &0 {
            *graph_start_freq
          }
          else {
            pow(2.0,log2(*graph_start_freq) + (*interval_no as f64)/(*subdivisions as f64))
          }
        );

        acc
      } 
    )
}

/***********************************************************************************************************************
 * Display configuration
 */
pub struct DisplayConfig {
  pub graph_start_freq : f64
, pub subdivisions     : u32
, pub frequencies      : Vec<f64>
}


impl DisplayConfig {
  pub fn default() -> DisplayConfig {
    DisplayConfig::new(DEFAULT_FREQ, DEFAULT_SUBDIVISION).unwrap()
  }

  pub fn new(start_freq_arg: f64, subdivisions_arg: u32) -> Result<DisplayConfig, DisplayError> {
    if start_freq_arg < START_FREQ ||
       start_freq_arg > END_FREQ {
      return Err(DisplayError::new(ErrType::Graph, start_freq_arg))
    }

    if !SUBDIVISIONS.contains(&subdivisions_arg) {
      return Err(DisplayError::new(ErrType::Subdivision, subdivisions_arg as f64))
    }

    return
      Ok(DisplayConfig {
          graph_start_freq : start_freq_arg
        , subdivisions     : subdivisions_arg
        , frequencies      : gen_frequencies(&start_freq_arg, &subdivisions_arg)
        })
  }
}

