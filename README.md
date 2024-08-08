# Arithmetic coding in Rust
A simple arithmetic coder implementation that should be easy to read. Includes a driver binary and a library. By default a binary is built.
## Build
Run `cargo build --release`.
## Run encoding routine
Run `target/release/arithmetic-coding -e <input from stdin>`. For example on Linux, just pipe your file using cat, and then direct the output to file: `cat war_and_peace.txt | target/release/arithmetic-coding -e > output.bin`
## Run decoding routine
Run `target/release/arithmetic-coding -d <input from stdin>`.
