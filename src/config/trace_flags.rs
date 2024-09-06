/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * (c) Chris Whealy 2020
 */
use crate::{
    chart,
    devices::{microperforated_panel, perforated_panel, porous_absorber, slotted_panel},
};

type TraceConfig<'a> = (bool, &'a str);

const TRACE_CONFIG: [TraceConfig; 11] = [
    (true, crate::MOD_NAME),
    (true, chart::render::MOD_NAME),
    (true, chart::render::draw::MOD_NAME),
    (true, microperforated_panel::MOD_NAME),
    (true, microperforated_panel::calc_engine::MOD_NAME),
    (true, perforated_panel::MOD_NAME),
    (true, perforated_panel::calc_engine::MOD_NAME),
    (true, porous_absorber::MOD_NAME),
    (true, porous_absorber::calc_engine::MOD_NAME),
    (true, slotted_panel::MOD_NAME),
    (true, slotted_panel::calc_engine::MOD_NAME),
];

pub fn trace_flag_for(mod_name: &str) -> bool {
    let mut flag: bool = false;

    for config in TRACE_CONFIG.iter() {
        if config.1 == mod_name {
            flag = config.0;
        }
    }

    flag
}
