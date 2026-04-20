#![no_std]
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_parens,
    clippy::missing_safety_doc,
    clippy::transmute_ptr_to_ptr,
    clippy::unused_unit,
    clippy::identity_op
)]
#![cfg_attr(all(not(feature = "bindgen"), Py_LIMITED_API), allow(unused_imports))]

#[cfg(feature = "bindgen")]
mod bindgen_bindings;

#[cfg(feature = "bindgen")]
pub use crate::bindgen_bindings::*;

#[cfg(not(feature = "bindgen"))]
include!("manual_bindings.rs");
