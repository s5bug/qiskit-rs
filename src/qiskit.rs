use crate::qiskit_ffi;

pub struct QuantumCircuit {
    circuit: *mut qiskit_ffi::QkCircuit,
}

impl QuantumCircuit {
    pub fn new(num_qubits: u32, num_clbits: u32) -> QuantumCircuit {
        let qc: *mut qiskit_ffi::QkCircuit = unsafe { qiskit_ffi::qk_circuit_new(num_qubits, num_clbits) };
        QuantumCircuit{
            circuit: qc,
        }
    }
    pub fn num_qubits(&mut self) -> u32 {
        unsafe { qiskit_ffi::qk_circuit_num_qubits(self.circuit) }
    }
    pub fn num_clbits(&mut self) -> u32 {
        unsafe { qiskit_ffi::qk_circuit_num_clbits(self.circuit) }
    }
    pub fn h(&mut self, qubit: u32) {
        unsafe { qiskit_ffi::qk_circuit_gate(self.circuit, qiskit_ffi::QkGate_QkGate_H, [qubit].as_ptr(), std::ptr::null()) };
    }
    pub fn cx(&mut self, control_qubit: u32, target_qubit: u32) -> u32 {
        unsafe { qiskit_ffi::qk_circuit_gate(self.circuit, qiskit_ffi::QkGate_QkGate_CX, [control_qubit, target_qubit].as_ptr(), std::ptr::null()) }
    }
    pub fn measure(&mut self, qubit: u32, clbit: u32) {
        unsafe { qiskit_ffi::qk_circuit_measure(self.circuit, qubit, clbit) };
    }
    pub fn copy(&mut self) -> QuantumCircuit {
        QuantumCircuit{
            circuit: unsafe { qiskit_ffi::qk_circuit_copy(self.circuit) },
        }
    }
}

impl Drop for QuantumCircuit {
    fn drop(&mut self) {
        println!("Freeing quantum circuit!");
        unsafe { qiskit_ffi::qk_circuit_free(self.circuit) };
    }
}
