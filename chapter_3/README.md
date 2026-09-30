# Chapter 3: Reserved Keywords & Lexical Scoping

This chapter covers reserved identifiers and scope mechanics in the Rust language.

## 📌 Topics Covered

1. **Reserved Keywords**:
   - Strict keywords that cannot be used as variable names (e.g. `fn`, `let`, `match`, `mut`, `struct`, `trait`, `type`, `where`).
   - Weak keywords and raw identifiers (`r#type`).
2. **Lexical Block Scoping**:
   - Inner block shadows outer bindings without destroying the outer variable once the block exits.
   - Stack memory cleanup on block exit.

## 📂 Subprojects

- [`variables/`](variables/): Demonstrates keyword documentation and inner block variable shadowing.

## 🚀 Running the Project

```bash
cargo run -p reserve_keywords
```
