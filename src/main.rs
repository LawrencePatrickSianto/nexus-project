#![no_std]
#![no_main]

mod hello_world;
mod addition;
mod fibonacci;

#[nexus_rt::main]
fn main() {
    hello_world::print_hello();

    // addition::run_addition(5, 8, 13);
    
    // fibonacci::run_fibonacci(7, 13);
}