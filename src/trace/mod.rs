/***********************************************************************************************************************
 * Trace Utility - Trace crossing function boundary
 *
 * FYI: The rust-analyzer might flag the calls to the `log` function as unsafe; however, the code compiles fine
 *
 * (c) Chris Whealy 2020, 2024
 */
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

pub mod trace_flags;

/***********************************************************************************************************************
 * Interface to browser console.log() function
 */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: String);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: String);
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
pub fn make_boundary_trace_fn<'a>(is_active: bool, mod_name: &'a str, fn_name: &'a str) -> Box<dyn Fn(TraceAction) + 'a> {
    if is_active {
        Box::new(
            move |action: TraceAction| {
                log(format!("{} {}.{}()", action, mod_name, fn_name));
            })
    } else {
        Box::new(move |_| {})
    }
}

/*********************************************************************************************************************
 * Trace data during execution flow
 */
pub fn make_trace_fn<'a>(is_active: bool, mod_name: &'a str, fn_name: &'a str) -> Box<dyn Fn(String) + 'a> {
    if is_active {
        Box::new(move |info| {
            log(format!("WASM      {}.{}() {}", mod_name, fn_name, info));
        })
    } else {
        Box::new(move |_| {})
    }
}
