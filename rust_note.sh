rustc main.rs # rustc is the compiler
cargo new rust_note

# after update Cargo.toml
cargo build
cargo build --verbose
cargo run
cargo run --verbose
cargo test
cargo check     # is much faster than cargo build because it skips the step of producing an executable.
cargo build --release   # compile with optimization -> final executable
cargo update    # When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file.
