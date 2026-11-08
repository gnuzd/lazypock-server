//! The lazypock binary.

mod cli;

fn main() {
    println!("lazypock {}", lazypock_core::version());
}
