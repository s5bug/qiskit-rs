QISKIT_DIR_NAME = qiskit_c_lib
QISKIT_DIR = $(realpath -s ./$(QISKIT_DIR_NAME))
QISKIT_URL = https://github.com/Qiskit/qiskit.git

build: ffi

test:
	cargo test

check_deps:
	rustc --version
	cargo --version
	gcc --version
	cbindgen --version

ffi: $(QISKIT_DIR)/dist/c
	cargo build

$(QISKIT_DIR)/dist/c: $(QISKIT_DIR)
	cd qiskit_c_lib && make c

$(QISKIT_DIR):
	git clone --depth 1 $(QISKIT_URL) $(QISKIT_DIR_NAME)

clean:
	rm -rf $(QISKIT_DIR)

.PHONY: all