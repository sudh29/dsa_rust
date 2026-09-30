// src/greetings.rs - Greetings module
// This module is in a separate file for better organization

// Private function (default)
fn private_hello() {
    println!("  [Private] Hello from private function");
}

// Public function
pub fn public_hello() {
    println!("  [Public] Hello from public function");
}

// Call private function within module
pub fn greet_with_private() {
    println!("  Calling private function from public function:");
    private_hello();
}

pub fn greet_formal() {
    println!("  Greetings! Welcome to the Rust learning module.");
}

pub fn greet_informal() {
    println!("  Hey there! Ready to learn some Rust?");
}
