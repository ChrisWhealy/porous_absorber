/***********************************************************************************************************************
 * Porous Absorber Calculator
 *
 * (c) Chris Whealy 2020
 */
use crate::{devices, calc_engine, chart};

type TraceConfig<'a> = (bool, &'a str);

const TRACE_CONFIG: [TraceConfig; 11] = [
    (false, crate::MOD_NAME),
    (false, calc_engine::microperforated_panel::MOD_NAME),
    (false, calc_engine::perforated_panel::MOD_NAME),
    (false, calc_engine::porous_absorber::MOD_NAME),
    (false, calc_engine::slotted_panel::MOD_NAME),
    (false, chart::render::MOD_NAME),
    (false, chart::render::draw::MOD_NAME),
    (false, devices::microperforated_panel::MOD_NAME),
    (false, devices::perforated_panel::MOD_NAME),
    (false, devices::porous_absorber::MOD_NAME),
    (false, devices::slotted_panel::MOD_NAME),
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
