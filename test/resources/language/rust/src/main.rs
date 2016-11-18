#![crate_type="bin"]
#![crate_id="flycheck"]

mod imported;
mod importing;
mod warnings;

pub fn main() {
    // Prevent an unused function foo() warning :)
    importing::foo();
    println!("Hello, world!");
}
