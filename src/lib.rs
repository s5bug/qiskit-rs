//! Qiskit bindings for Rust
//!
//! This library exposes the C API for Qiskit in Rust.
//!
//! ## Creating a Circuit
//!
//! ```
//! use qiskit_rs::QuantumCircuit;
//!
//! fn main() {
//!     // Initialize a circuit with 2 quantum registers and 2 classical registers
//!     let mut qc = QuantumCircuit::new(2, 2);
//!
//!     // Apply circuit operations
//!     qc.h(0);
//!     qc.cx(0, 1);
//!
//!     // Apply measurements
//!     qc.measure(0, 0);
//!     qc.measure(1, 1);
//! }
//! ```

#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
// #[cfg(not(doctest))]
mod qiskit_ffi;
pub mod qiskit;

pub use qiskit::{
    QuantumCircuit,
    QuantumRegister,
    ClassicalRegister,
    QiskitError,
};
