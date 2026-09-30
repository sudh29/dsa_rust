//! # Rust Reserved Keywords
//!
//! The following are reserved keywords in Rust and cannot be used as identifiers:
//!
//! - `as`, `break`, `const`, `continue`, `crate`, `else`, `enum`, `extern`, `false`, `fn`, `for`
//! - `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`
//! - `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`
//! - `where`, `while`, `async`, `await`, `dyn`
//!
//! For a complete and up-to-date list, see the [Rust Reference](https://doc.rust-lang.org/reference/keywords.html).

fn main() {
    // Immutable variable example
    let x = 5;
    println!("x: {}", x);

    // Mutable variable example
    let mut y = 10;
    println!("y: {}", y);
    y += 5; // Modifying the mutable variable
    println!("Modified y: {}", y);

    // Scope example
    {
        let y = 25; // This `y` shadows the outer `y` within this block
        println!("Inner scoped y: {}", y);
    }
    println!("Outer y after inner scope: {}", y); // Original `y` is still accessible

    // Constant example
    const MAX_POINTS: usize = 100;
    println!("Max points: {}", MAX_POINTS);

    // Shadowing example
    let p: i32 = 42;
    println!("Original p: {}", p);
    let p = 100; // Shadowing the previous `p`
    println!("Shadowed p: {}", p);
}
