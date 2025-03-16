cargo is rust's build system and package manager

cargo --version: gives the version that is installed

cargo new <project-name> creates a new directory with the same name and it has 2 files in it:
- Cargo.toml - this file includes all the package description and it's dependencies
- src - All the code resides in this directory.

cargo build: this command compiles the code 
./target/debug/<filename>: this is the location of the executable

cargo run: this command compiles and runs the executable.
cargo check: this command compiles the code but does not generate an executable which is faster.

cargo build --release: this compiles the code and creates the executable in target/release. This is slower as compared to debug. Use debug when you have to compile and run the code frequently
