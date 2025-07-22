use qiskit_rs::{QuantumCircuit,QiskitError};


#[test]
fn test_dcx() {
    let mut qc = QuantumCircuit::new(2, 0);
    let ret = qc.dcx(0, 1);
    assert_eq!(ret, QiskitError::Success)
}

#[test]
fn test_ecr() {
    let mut qc = QuantumCircuit::new(2, 0);
    qc.ecr(0, 1);
}
