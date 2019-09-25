// *********************************************************************************************************************
// Porous Absorber Calculation Engine
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************
extern crate wasm_bindgen;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Usage
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use libm::{sin, cos, sqrt, pow, log};
use num::complex::Complex;
use std::f64::consts::PI;

use crate::trace::Trace;

use crate::air::AirConfig;
use crate::cavity::CavityConfig;
use crate::display::DisplayConfig;
use crate::sound::SoundConfig;
use crate::porous_absorber::PorousAbsorberConfig;
use crate::perforated_panel::PerforatedPanelConfig;
use crate::slotted_panel::SlottedPanelConfig;

use crate::struct_lib::{
  PorousAbsInfo
, PerforatedAbsInfo
, SlottedAbsInfo
, PlotPoint
};


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Trace functionality
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const LIB_NAME     : &str  = &"calc_engine";
const TRACE_ACTIVE : &bool = &false;


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Constants
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const PI_OVER_180    : f64 = PI / 180.0;
const ONE_80_OVER_PI : f64 = 180.0 / PI;
const AIR_VISCOSITY  : f64 = 0.0000185;


// *********************************************************************************************************************
// Public API
// *********************************************************************************************************************

// *********************************************************************************************************************
// Porous Absorber
pub fn calculate_porous_absorber(
  air    : &AirConfig
, cavity : &CavityConfig
, display: &DisplayConfig
, sound  : &SoundConfig
, porous : &PorousAbsorberConfig
) -> PorousAbsInfo {
  const FN_NAME : &str = &"calculate_porous_absorber";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let abs_info = display
    .frequencies
    .iter()
    .fold(
      PorousAbsInfo { air_gap: vec!(), no_air_gap : vec!() }
    , | mut acc, frequency | {
        let (abs_no_air_gap, abs_air_gap) = do_porous_abs_calc(*frequency, &air, &cavity, &sound, &porous);

        // Build the vectors of plot points for each absorber type
        acc.no_air_gap.push(PlotPoint { x: *frequency, y: abs_no_air_gap});
        acc.air_gap.push(PlotPoint { x: *frequency, y: abs_air_gap});

        return acc;
      }
    );
  
  trace_boundary(&Some(false));
  return abs_info;
}


// *********************************************************************************************************************
// Perforated Panel
pub fn calculate_perforated_panel(
  air    : &AirConfig
, cavity : &CavityConfig
, display: &DisplayConfig
, panel  : &PerforatedPanelConfig
, porous : &PorousAbsorberConfig
) -> PerforatedAbsInfo {
  const FN_NAME : &str = &"calculate_perforated_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Calculate apparent panel thickness
  let end_correction_delta          = 0.8 * (1.0 - 1.47 * sqrt(panel.porosity) + 0.47 * sqrt(pow(panel.porosity, 3.0)));
  let end_corrected_panel_thickness = panel.thickness + (2.0 * panel.hole_radius * end_correction_delta);

  trace(&format!("End correction delta          = {}", &end_correction_delta));
  trace(&format!("End corrected panel thickness = {}", &end_corrected_panel_thickness));

  let abs_info = display
    .frequencies
    .iter()
    .fold(
      PerforatedAbsInfo { abs_against_panel: vec!(), abs_against_backing : vec!(), no_air_gap: vec!() }
    , | mut acc, frequency | {
        let (
          abs_no_air_gap
        , abs_against_panel
        , abs_against_backing
        ) = do_perforated_panel_calc(*frequency, &air, &cavity, &panel, &porous, end_corrected_panel_thickness);

        // Build the vectors of plot points for each absorber type
        acc.abs_against_backing.push(PlotPoint { x: *frequency, y: abs_against_backing});
          acc.abs_against_panel.push(PlotPoint { x: *frequency, y: abs_against_panel});
                 acc.no_air_gap.push(PlotPoint { x: *frequency, y: abs_no_air_gap});

        return acc;
      }
    );

  trace_boundary(&Some(false));
  return abs_info;
}



// *********************************************************************************************************************
// Slotted Panel
pub fn calculate_slotted_panel(
  air    : &AirConfig
, cavity : &CavityConfig
, display: &DisplayConfig
, panel  : &SlottedPanelConfig
, porous : &PorousAbsorberConfig
) -> SlottedAbsInfo {
  const FN_NAME : &str = &"calculate_slotted_panel";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Calculate apparent panel thickness
  let end_correction_delta          = -log(sin(PI * panel.porosity / 2.0)) / PI;
  let end_corrected_panel_thickness = panel.thickness + (2.0 * panel.slot_width * end_correction_delta);

  trace(&format!("End correction delta          = {}", &end_correction_delta));
  trace(&format!("End corrected panel thickness = {}", &end_corrected_panel_thickness));

  // Calculate resistance terms
  let resistance_at_backing = porous.sigma as f64 * porous.thickness;
  let resistance_at_panel   = resistance_at_backing * panel.porosity;
  let mass_term_for_air     = end_corrected_panel_thickness * air.density / panel.porosity;

  trace(&format!("End correction delta          = {}", &end_correction_delta));
  trace(&format!("End corrected panel thickness = {}", &end_corrected_panel_thickness));

  let abs_info = display
    .frequencies
    .iter()
    .fold(
      SlottedAbsInfo { abs_against_panel: vec!(), abs_against_backing : vec!(), no_air_gap: vec!() }
    , | mut acc, frequency | {
        let (
          abs_no_air_gap
        , abs_against_panel
        , abs_against_backing
        ) = do_slotted_panel_calc(
              *frequency
            , &air
            , &cavity
            , &porous
            , end_corrected_panel_thickness
            , resistance_at_panel
            , resistance_at_backing
            , mass_term_for_air
            );

        // Build the vectors of plot points for each absorber type
        acc.abs_against_backing.push(PlotPoint { x: *frequency, y: abs_against_backing});
          acc.abs_against_panel.push(PlotPoint { x: *frequency, y: abs_against_panel});
                 acc.no_air_gap.push(PlotPoint { x: *frequency, y: abs_no_air_gap});

        return acc;
      }
    );

  trace_boundary(&Some(false));
  return abs_info;
}



// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reducer function to calculate the absorption of a porous absorber at a specific frequency
fn do_porous_abs_calc(
  frequency  : f64
, air_cfg    : &AirConfig
, cavity_cfg : &CavityConfig
, sound_cfg  : &SoundConfig
, porous_cfg : &PorousAbsorberConfig
) -> (f64, f64) {
  // Frequently used intermediate values
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

  let sin_phi: f64 = sin(sound_cfg.angle as f64 * PI_OVER_180);
  let cos_phi: f64 = cos(sound_cfg.angle as f64 * PI_OVER_180);

  // Wave number in air
  let k_air = air_cfg.two_pi_over_c * frequency;

  // Delaney and Bazley's term X
  let d_and_b_term_x = (air_cfg.density * frequency) / porous_cfg.sigma as f64;

  // Characteristic absorber impedance
  let z_abs = air_cfg.impedance * Complex::new(1.0 + 0.0571 * pow(d_and_b_term_x, -0.754), -0.087 * pow(d_and_b_term_x, -0.732));

  // Complex wave number within the porous absorber layer with its Y and X component values
  let wave_no_abs = air_cfg.two_pi_over_c
     * frequency
     * Complex::new(1.0 + 0.0978 * pow(d_and_b_term_x, -0.7), -0.189 * pow(d_and_b_term_x, -0.595));
        
  let wave_no_abs_y_comp = k_air * sin_phi;
  let wave_no_abs_x_comp = ((wave_no_abs * wave_no_abs) - (wave_no_abs_y_comp * wave_no_abs_y_comp)).sqrt();

  // Angle of propagation within porous layer
  let beta_porous = sin(abs(wave_no_abs_y_comp / wave_no_abs)) * ONE_80_OVER_PI;

  // Intermediate term for porous impedance calculation
  let porous_wave_no     = wave_no_abs * porous_cfg.thickness;
  let cot_porous_wave_no = porous_wave_no.cos() / porous_wave_no.sin();

  // Impedance at absorber surface
  let z_abs_surface = minus_i * z_abs * (wave_no_abs / wave_no_abs_x_comp) * cot_porous_wave_no;

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Calculate absorption coefficient for porous absorber with no air gap
  let abs_refl  = difference_over_sum((z_abs_surface / air_cfg.impedance) * cos_phi, 1.0);
  let abs_alpha = reflectivity_as_alpha(abs_refl);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Impedance values (with air gap)
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

  // X and Y components of the wave number in the air gap
  let wave_no_air_y_comp = wave_no_abs * sin(beta_porous * PI_OVER_180);
  let wave_no_air_x_comp = ((k_air * k_air) - (wave_no_air_y_comp * wave_no_air_y_comp)).sqrt();

  // Impedance at top of air gap (after passing through porous absorber)
  let temp_imp = k_air * cavity_cfg.air_gap;
  let air_gap_z = minus_i * air_cfg.impedance * (k_air / wave_no_air_x_comp) * (cos(temp_imp) / sin(temp_imp));

  // Impedance at top of porous absorber after passing through air gap
  let intermediate3 = minus_i * z_abs * cot_porous_wave_no;
  let abs_air_z     = ((air_gap_z * intermediate3) + (z_abs * z_abs)) / (air_gap_z + intermediate3);

  // Absorption coefficient for porous absorber with air gap
  let abs_air_refl  = difference_over_sum((abs_air_z / air_cfg.impedance) * cos_phi, 1.0);
  let abs_air_alpha = reflectivity_as_alpha(abs_air_refl);

  return (abs_alpha, abs_air_alpha);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reducer function to calculate the absorption of a perforated panel absorber
fn do_perforated_panel_calc(
  frequency          : f64
, air_cfg            : &AirConfig
, cavity_cfg         : &CavityConfig
, panel_cfg          : &PerforatedPanelConfig
, porous_cfg         : &PorousAbsorberConfig
, ec_panel_thickness : f64
) -> (f64, f64, f64) {
  const FN_NAME : &str = &"do_perforated_panel_calc";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Frequently used intermediate values
  let i      : Complex<f64> = Complex::new(0.0, 1.0);
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

  // Angular frequency and wave number in air
  let omega = 2.0 * PI * frequency;
  let k_air = air_cfg.two_pi_over_c * frequency;

  trace(&format!("Angular frequency = {}", omega));
  trace(&format!("Wave number = {}", k_air));

  // Delaney and Bazley's term X
  let d_and_b_term_x = (air_cfg.density * frequency) / porous_cfg.sigma as f64;
  trace(&format!("Delaney and Bazley's term X = {}", d_and_b_term_x));

  // Characteristic absorber impedance
  let z_abs = air_cfg.impedance * Complex::new(1.0 + 0.0571 * pow(d_and_b_term_x, -0.754), -0.087 * pow(d_and_b_term_x, -0.732));
  trace(&format!("Characteristic impedance = {}", z_abs));

  // Complex wave number within the porous absorber layer
  let wave_no_abs = k_air * Complex::new(1.0 + 0.0978 * pow(d_and_b_term_x, -0.7), -0.189 * pow(d_and_b_term_x, -0.595));

  trace(&format!("Complex wave number = {}", wave_no_abs));

  // Intermediate terms
  let inter1     = k_air * cavity_cfg.air_gap;
  let cot_inter1 = inter1.cos() / inter1.sin();
  let inter2     = wave_no_abs * porous_cfg.thickness;
  let cot_inter2 = inter2.cos() / inter2.sin();

  trace(&format!("k air * t air      = {}", inter1));
  trace(&format!("cot(k air * t air) = {}", cot_inter1));
  trace(&format!("k abs * t abs      = {}", inter2));
  trace(&format!("cot(k abs * t abs) = {}", cot_inter2));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against panel
  let abs_against_panel_z1 = minus_i * air_cfg.impedance * cot_inter1;
  let abs_against_panel_z2 = ((minus_i * abs_against_panel_z1 * z_abs * cot_inter2) + (z_abs * z_abs)) / 
                             (abs_against_panel_z1 - (i * z_abs * cot_inter2));
  let surface_resistence   = (air_cfg.density / panel_cfg.porosity) *
                             sqrt(8.0 * AIR_VISCOSITY * omega) *
                             (1.0 + ec_panel_thickness / (2.0 * panel_cfg.hole_radius));
  let abs_against_panel_z3 = ((i / panel_cfg.porosity) * ec_panel_thickness * omega * air_cfg.density) +
                             abs_against_panel_z2 +
                             surface_resistence;
  
  let abs_against_panel_refl  = difference_over_sum(abs_against_panel_z3, air_cfg.impedance);
  let abs_against_panel_alpha = reflectivity_as_alpha(abs_against_panel_refl);

  trace(&format!("Absorber against panel z1 = {}", abs_against_panel_z1));
  trace(&format!("Absorber against panel z2 = {}", abs_against_panel_z2));
  trace(&format!("Surface resistance        = {}", surface_resistence));
  trace(&format!("Overall impedence         = {}", abs_against_panel_z3));
  
  trace(&format!("Absorber against panel reflection = {}", abs_against_panel_refl));
  trace(&format!("Absorber against panel absorption = {}", abs_against_panel_alpha));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against backing
  let abs_against_backing_z1 = minus_i * z_abs * cot_inter2;
  let abs_against_backing_z2 = ((minus_i * abs_against_backing_z1 * air_cfg.impedance * cot_inter1) +
                                (air_cfg.impedance * air_cfg.impedance)) /
                               (abs_against_backing_z1 - (i * air_cfg.impedance * cot_inter1));

  let abs_against_backing_z3 = ((air_cfg.density / panel_cfg.porosity) *
                                sqrt(8.0 * AIR_VISCOSITY * omega) *
                                ((panel_cfg.thickness / 2.0 * panel_cfg.hole_radius) + 1.0)) +
                               ((ec_panel_thickness / panel_cfg.porosity) * i * omega * air_cfg.density) +
                               abs_against_backing_z2;

  let abs_against_backing_refl  = difference_over_sum(abs_against_backing_z3, air_cfg.impedance);
  let abs_against_backing_alpha = reflectivity_as_alpha(abs_against_backing_refl);

  trace(&format!("Absorber against backing z1 = {}", abs_against_backing_z1));
  trace(&format!("Absorber against backing z2 = {}", abs_against_backing_z2));
  trace(&format!("Absorber against backing z3 = {}", abs_against_backing_z3));

  trace(&format!("Absorber against backing reflection = {}", abs_against_backing_refl));
  trace(&format!("Absorber against backing absorption = {}", abs_against_backing_alpha));


  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber with no air gap
  let inter3     = wave_no_abs * (porous_cfg.thickness + cavity_cfg.air_gap);
  let cot_inter3 = inter3.cos() / inter3.sin();

  let no_air_gap_z1 = minus_i * z_abs * cot_inter3;
  let no_air_gap_z2 = (i * omega * air_cfg.density * (ec_panel_thickness / panel_cfg.porosity)) +
                          no_air_gap_z1;

  let no_air_gap_refl  = difference_over_sum(no_air_gap_z2, air_cfg.impedance);
  let no_air_gap_alpha = reflectivity_as_alpha(no_air_gap_refl);

  trace(&format!("cot(complex wave no * cavity depth) = {}", cot_inter3));
  trace(&format!("No air gap z1 = {}", no_air_gap_z1));
  trace(&format!("No air gap z2 = {}", no_air_gap_z2));

  trace(&format!("No air gap reflection = {}", no_air_gap_refl));
  trace(&format!("No air gap absorption = {}", no_air_gap_alpha));


  trace_boundary(&Some(false));
  return (no_air_gap_alpha, abs_against_panel_alpha, abs_against_backing_alpha);
}



// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Reducer function to calculate the absorption of a slotted panel absorber
fn do_slotted_panel_calc(
  frequency             : f64
, air_cfg               : &AirConfig
, cavity_cfg            : &CavityConfig
, porous_cfg            : &PorousAbsorberConfig
, ec_panel_thickness    : f64
, resistance_at_panel   : f64
, resistance_at_backing : f64
, mass_term_for_air     : f64
) -> (f64, f64, f64) {
  const FN_NAME : &str = &"do_slotted_panel_calc";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Frequently used intermediate values
  let i      : Complex<f64> = Complex::new(0.0, 1.0);
  let minus_i: Complex<f64> = Complex::new(0.0, -1.0);

  // Angular frequency and wave number in air
  let omega = 2.0 * PI * frequency;
  let k_air = air_cfg.two_pi_over_c * frequency;

  trace(&format!("Angular frequency = {}", omega));
  trace(&format!("Wave number = {}", k_air));

  // Delaney and Bazley's term X
  let d_and_b_term_x = (air_cfg.density * frequency) / porous_cfg.sigma as f64;
  trace(&format!("Delaney and Bazley's term X = {}", d_and_b_term_x));

  // Characteristic absorber impedance
  let z_abs = air_cfg.impedance * Complex::new(1.0 + 0.0571 * pow(d_and_b_term_x, -0.754), -0.087 * pow(d_and_b_term_x, -0.732));
  trace(&format!("Characteristic impedance = {}", z_abs));

  // Complex wave number within the porous absorber layer
  let wave_no_abs = k_air * Complex::new(1.0 + 0.0978 * pow(d_and_b_term_x, -0.7), -0.189 * pow(d_and_b_term_x, -0.595));

  trace(&format!("Complex wave number = {}", wave_no_abs));

  // Intermediate terms
  let inter1     = k_air * ec_panel_thickness;
  let cot_inter1 = inter1.cos() / inter1.sin();
  trace(&format!("cot(k air * t panel) = {}", cot_inter1));

  let inter2     = k_air * cavity_cfg.air_gap;
  let cot_inter2 = inter2.cos() / inter2.sin();
  trace(&format!("cot(k abs * t air) = {}", cot_inter2));

  let inter3     = wave_no_abs * porous_cfg.thickness;
  let cot_inter3 = inter3.cos() / inter3.sin();
  trace(&format!("cot(complex_wave_no * t abs) = {}", cot_inter3));

  let inter4     = wave_no_abs * (cavity_cfg.air_gap + porous_cfg.thickness);
  let cot_inter4 = inter4.cos() / inter4.sin();
  trace(&format!("cot(complex_wave_no * total depth) = {}", cot_inter4));

  let mass_term_for_slotted_panel = i * ((omega * mass_term_for_air) - (air_cfg.impedance * cot_inter1));
  trace(&format!("Mass term for air in slotted panel = {}", mass_term_for_slotted_panel));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against panel
  let abs_against_panel_z1 = minus_i * air_cfg.impedance * cot_inter2;
  let abs_against_panel_z2 = ((minus_i * abs_against_panel_z1 * z_abs * cot_inter3) + (z_abs * z_abs)) / 
                             (abs_against_panel_z1 - (i * z_abs * cot_inter3));
  let abs_against_panel_z3 = resistance_at_panel + mass_term_for_slotted_panel + abs_against_panel_z2;
  
  let abs_against_panel_refl  = difference_over_sum(abs_against_panel_z3, air_cfg.impedance);
  let abs_against_panel_alpha = reflectivity_as_alpha(abs_against_panel_refl);

  trace(&format!("Absorber against panel z1 = {}", abs_against_panel_z1));
  trace(&format!("Absorber against panel z2 = {}", abs_against_panel_z2));
  trace(&format!("Overall impedence         = {}", abs_against_panel_z3));
  
  trace(&format!("Absorber against panel reflection = {}", abs_against_panel_refl));
  trace(&format!("Absorber against panel absorption = {}", abs_against_panel_alpha));

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber against backing
  let abs_against_backing_z1 = minus_i * z_abs * cot_inter3;
  let abs_against_backing_z2 = ((minus_i * abs_against_backing_z1 * air_cfg.impedance * cot_inter2) +
                                (air_cfg.impedance * air_cfg.impedance)) /
                               (abs_against_backing_z1 - (i * air_cfg.impedance * cot_inter2));
  let abs_against_backing_z3 = resistance_at_backing + mass_term_for_slotted_panel + abs_against_backing_z2;

  let abs_against_backing_refl  = difference_over_sum(abs_against_backing_z3, air_cfg.impedance);
  let abs_against_backing_alpha = reflectivity_as_alpha(abs_against_backing_refl);

  trace(&format!("Absorber against backing z1 = {}", abs_against_backing_z1));
  trace(&format!("Absorber against backing z2 = {}", abs_against_backing_z2));
  trace(&format!("Absorber against backing z3 = {}", abs_against_backing_z3));

  trace(&format!("Absorber against backing reflection = {}", abs_against_backing_refl));
  trace(&format!("Absorber against backing absorption = {}", abs_against_backing_alpha));


  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Absorber with no air gap
  let no_air_gap_z1 = minus_i * z_abs * cot_inter4;
  let no_air_gap_z2 = resistance_at_panel + mass_term_for_slotted_panel + no_air_gap_z1;

  let no_air_gap_refl  = difference_over_sum(no_air_gap_z2, air_cfg.impedance);
  let no_air_gap_alpha = reflectivity_as_alpha(no_air_gap_refl);

  trace(&format!("No air gap z1 = {}", no_air_gap_z1));
  trace(&format!("No air gap z2 = {}", no_air_gap_z2));

  trace(&format!("No air gap reflection = {}", no_air_gap_refl));
  trace(&format!("No air gap absorption = {}", no_air_gap_alpha));

  trace_boundary(&Some(false));
  return (no_air_gap_alpha, abs_against_panel_alpha, abs_against_backing_alpha);
}



// *********************************************************************************************************************
// The num::complex::Complex module does not contain a function for returning the absolute value of a complex number
// However, this can be calculated by taking the square root of the normal square
fn abs(cplx: Complex<f64>) -> f64 {
  sqrt(cplx.norm_sqr())
}

// *********************************************************************************************************************
// General purpose differnce over sum calculation
fn difference_over_sum(a: Complex<f64>, b: f64) -> Complex<f64> {
  (a - b ) / (a + b)
}

// *********************************************************************************************************************
// Convert reflectivity to absoprtion and round to two decimal places
// If the value is less than zero, then return 0.0
fn reflectivity_as_alpha(refl: Complex<f64>) -> f64 {
   let alpha = 1.0 - pow(abs(refl), 2.0);

  // Ignore alpha values less than zero, else round to 2dp
  if alpha < 0.0 {
    0.0
  }
  else {
    (alpha * 100.0).round() / 100.0
  }
}


