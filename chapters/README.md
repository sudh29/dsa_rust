# 📖 Rust Learning Chapters Curriculum

This directory contains the hands-on tutorial crates adapted from *The Rust Programming Language* book, organized systematically as workspace members.

---

## Chapter Index

| Chapter | Sub-crate(s) | Topics & Concepts | Run Command |
|:---|:---|:---|:---|
| **[Chapter 1](chapter_1/README.md)** | `section_2` | Hello World, Cargo package structure, build vs run | `cargo run -p section_2` |
| **[Chapter 2](chapter_2/README.md)** | `guess_game`, `variables` | Guessing game CLI, `io::stdin`, `match`, mutability | `cargo run -p guess_game` |
| **[Chapter 3](chapter_3/README.md)** | `variables` | Constants, shadowing, immutable vs mutable bindings | `cargo run -p reserve_keywords` |
| **[Chapter 4](chapter_4/)** | `data_type` | Scalar types (integers, floats, bool, char) & compound types (tuples, arrays) | `cargo run -p data_type` |
| **[Chapter 5](chapter_5/)** | `functions` | Function definitions, parameter types, statements vs expressions, return values | `cargo run -p functions` |
| **[Chapter 6](chapter_6/)** | `control_flow` | `if`/`else` branching, `loop`, `while`, `for` iteration, loop labels | `cargo run -p control_flow` |
| **[Chapter 7](chapter_7/)** | `ownership` | Ownership rules, stack vs heap, move semantics, clone, references, borrowing, slices | `cargo run -p ownership` |
| **[Chapter 8](chapter_8/)** | `structs` | Defining structs, field init shorthand, tuple structs, `impl` blocks, methods, associated functions | `cargo run -p structs` |
| **[Chapter 9](chapter_9/)** | `enum_match` | Defining enums, Option<T>, match control flow, pattern extraction, `if let` | `cargo run -p enum_match` |
| **[Chapter 10](chapter_10/)** | `package_crates_module` | Packages, binary vs library crates, module hierarchy, `pub` visibility, `use` paths | `cargo run -p package_crates_module` |
| **[Chapter 11](chapter_11/)** | `data_types` | Standard collections: `Vec<T>`, UTF-8 `String`, `HashMap<K, V>` | `cargo run -p data_types` |
| **[Chapter 12](chapter_12/)** | `panic_results` | Error handling: unrecoverable `panic!`, recoverable `Result<T, E>`, `?` operator | `cargo run -p panic_results` |
| **[Chapter 13](chapter_13/)** | `generic_type_traits_lifetimes` | Generic data types and functions, Traits as interfaces, Trait bounds, Lifetime annotations | `cargo run -p generic_type_traits_lifetimes` |

---

## Workspace Execution

Run any individual chapter binary using `cargo run -p <CRATE_NAME>`:

```bash
# Example: Run Chapter 8 Structs demo
cargo run -p structs

# Example: Run Chapter 13 Generics & Lifetimes
cargo run -p generic_type_traits_lifetimes
```
