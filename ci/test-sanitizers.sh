set -euo pipefail

export CXXFLAGS="${CXXFLAGS:-} -fsanitize=address,undefined -fno-omit-frame-pointer"
export RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-lasan -C link-arg=-lubsan"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER="env LD_PRELOAD=$(gcc -print-file-name=libasan.so)"
export ASAN_OPTIONS=detect_leaks=1
export UBSAN_OPTIONS=halt_on_error=1:print_stacktrace=1
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target/sanitizers}"

cargo test -p rdkit --locked --test test_safety --test test_graphmol --test test_scaffold_network --test test_substruct --test test_mol_ops --test test_file_parsers
cargo test -p rdkit-sys --locked --tests
