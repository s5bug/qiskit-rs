#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod qiskit;
pub use crate::qiskit::QuantumCircuit;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghz() {
        unsafe {
            let num_qubits = 2;
            let circuit = qk_circuit_new(num_qubits, num_qubits);
            qk_circuit_gate(circuit, QkGate_QkGate_H, [0].as_ptr(), std::ptr::null());
            for i in 0..(num_qubits - 1) {
                qk_circuit_gate(circuit, QkGate_QkGate_CX, [i, i + 1].as_ptr(), std::ptr::null());
            }
            for i in 0..num_qubits {
                qk_circuit_measure(circuit, i, i);
            }
            qk_circuit_free(circuit);
        }
    }
}