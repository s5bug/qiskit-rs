use super::*;

pub struct QuantumCircuit {
    circuit: *mut QkCircuit,
    pub num_qubits: u32,
    pub num_clbits: u32,
}

impl QuantumCircuit {
    pub fn new(num_qubits: u32, num_clbits: u32) -> QuantumCircuit {
        let qc: *mut QkCircuit = unsafe { qk_circuit_new(num_qubits, num_clbits) };
        QuantumCircuit{
            circuit: qc,
            num_qubits: num_qubits,
            num_clbits: num_clbits,
        }
    }
    pub fn h(&mut self, qubit: u32) {
        unsafe { qk_circuit_gate(self.circuit, QkGate_QkGate_H, [qubit].as_ptr(), std::ptr::null()) };
    }
    pub fn cx(&mut self, control_qubit: u32, target_qubit: u32) -> u32 {
        unsafe { qk_circuit_gate(self.circuit, QkGate_QkGate_CX, [control_qubit, target_qubit].as_ptr(), std::ptr::null()) }
    }
    pub fn measure(&mut self, qubit: u32, clbit: u32) {
        unsafe { qk_circuit_measure(self.circuit, qubit, clbit) };
    }
    fn drop(&mut self) {
        println!("Freeing quantum circuit!");
        unsafe { qk_circuit_free(self.circuit) };
    }
}