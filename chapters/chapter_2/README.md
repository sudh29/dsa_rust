# Chapter 2: Variables, Mutability & The Guessing Game

This chapter covers variable bindings, immutability by default, shadowing, and interactive CLI I/O in Rust.

## 📌 Topics Covered

1. **Variables and Mutability**:
   - `let` declares immutable bindings by default.
   - `let mut` enables mutability.
   - Constants (`const`) require type annotations and must be evaluatable at compile time.
2. **Variable Shadowing**:
   - Re-binding a variable name using `let` allows transforming values and changing types within a scope.
3. **Interactive Programming (The Guessing Game)**:
   - Reading standard input (`std::io::stdin()`).
   - Parsing strings into integers with `.trim().parse()`.
   - Pattern matching using `std::cmp::Ordering` with `match`.
   - Generating random numbers via the `rand` crate.

## 📂 Subprojects

- [`variables/`](variables/): Demonstration of immutable vs mutable variables, constants, and shadowing.
- [`guess_game/`](guess_game/): Complete interactive number guessing game.

## 🚀 Running the Projects

```bash
# Run the variables demo
cargo run -p variables

# Run the interactive guessing game
cargo run -p guess_game
```
