/***********************************************************************************************************************
 * Trace Utility - Trace crossing function boundary
 *
 * FYI: The rust-analyzer might flag the calls to the `log` function as unsafe; however, the code compiles fine
 *
 * (c) Chris Whealy 2020
 */
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

/***********************************************************************************************************************
 * Interface to browser console.log() function
 */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

pub enum TraceAction {
    Enter,
    Exit,
    EnterExit,
}

impl std::fmt::Display for TraceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match &self {
            TraceAction::Enter => "WASM --->",
            TraceAction::Exit => "WASM <---",
            TraceAction::EnterExit => "WASM <-->",
        };

        write!(f, "{}", label)
    }
}

/*********************************************************************************************************************
 * Trace execution flow at function boundaries
 */
pub fn make_boundary_trace_fn<'a>(is_active: bool, mod_name: &'a str, fn_name: &'a str) -> impl Fn(TraceAction) + 'a {
    move |action: TraceAction| {
        if is_active {
            log(format!("{} {}.{}()", action, mod_name, fn_name));
        }
    }
}

/*********************************************************************************************************************
 * Trace data during execution flow
 */
pub fn make_trace_fn<'a>(is_active: bool, mod_name: &'a str, fn_name: &'a str) -> impl Fn(String) + 'a {
    move |info| {
        if is_active {
            log(format!("WASM      {}.{}() {}", mod_name, fn_name, info));
        }
    }
}
