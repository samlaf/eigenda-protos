default:
    @just --list

# run src/bin/gen.rs to generate the protos
generate-protos:
    cargo run --bin gen
