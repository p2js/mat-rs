#![warn(clippy::pedantic)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_panics_doc)]
#![no_std]
pub mod dmat;
pub mod mat;

//no_std f64 abs, helper function used in both Mat and Dmat
fn f64_abs(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & (i64::MAX as u64))
}
