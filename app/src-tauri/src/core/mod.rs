//! VietFlux Core Engine
//!
//! Original Vietnamese IME engine for VietFlux.
//! Implements Telex and VNI input methods with validation.

#![allow(dead_code)]

pub mod data;
pub mod processor;
pub mod buffer;
pub mod transform;
pub mod phonology;
pub mod validator;

// Re-export main types for convenience
pub use processor::Processor;
pub use buffer::{CompositionBuffer, InputUnit};
