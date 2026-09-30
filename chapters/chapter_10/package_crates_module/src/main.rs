// RUST PACKAGES, CRATES, AND MODULES - MULTI-FILE EXAMPLE
// =========================================================
//
// PACKAGE: A Cargo project (contains Cargo.toml)
// - Contains one or more crates
// - Cargo.toml describes how to build those crates
// - Example: This entire folder is a package
//
// CRATE: A binary or library
// - Binary crate: An executable (has main() function) - src/main.rs
// - Library crate: Code meant to be used by other programs - src/lib.rs
// - A package can have at most one library crate but multiple binary crates
//
// MODULE: A namespace for organizing code within a crate
// - Modules create a hierarchy and control privacy
// - Use 'mod' keyword to define modules
// - Modules can be in separate files
// - Public items: accessible outside the module (pub keyword)
// - Private items: only accessible within the module or parent modules
//
// FILE STRUCTURE:
// src/
//   main.rs          ← Binary crate root (this file)
//   lib.rs           ← Library crate root (declares modules)
//   greetings.rs     ← greetings module
//   math/
//     mod.rs         ← math module root (declares submodules)
//     arithmetic.rs  ← math::arithmetic module
//     geometry.rs    ← math::geometry module
//   shapes/
//     mod.rs         ← shapes module root (declares submodules)
//     rectangle.rs   ← shapes::rectangle module
//     circle.rs      ← shapes::circle module

// Import from the library crate (lib.rs)
// This makes all public items from lib.rs available
use package_crates_module::{
    Circle, // Re-exported from lib.rs
    Rectangle,
    arithmetic,
    geometry, // Re-exported from lib.rs
    greetings,
    math,
};

fn main() {
    println!("=== Rust Packages, Crates, and Modules ===\n");
    println!("Multi-File Organization Example\n");

    // UNDERSTANDING PACKAGES AND CRATES
    println!("--- Package and Crate Structure ---\n");
    println!("📦 Package: 'package_crates_module' (defined in Cargo.toml)");
    println!("├── 📚 Binary Crate: src/main.rs (this file)");
    println!("│   └── Entry Point: fn main() ← You are here");
    println!("└── 📚 Library Crate: src/lib.rs");
    println!("    ├── 📂 Module: greetings (src/greetings.rs)");
    println!("    ├── 📂 Module: math (src/math/)");
    println!("    │   ├── arithmetic (src/math/arithmetic.rs)");
    println!("    │   └── geometry (src/math/geometry.rs)");
    println!("    └── 📂 Module: shapes (src/shapes/)");
    println!("        ├── rectangle (src/shapes/rectangle.rs)");
    println!("        └── circle (src/shapes/circle.rs)\n");

    // GREETINGS MODULE
    println!("--- Greetings Module (from src/greetings.rs) ---\n");
    greetings::public_hello();
    greetings::greet_formal();
    greetings::greet_informal();
    println!();

    // MATH MODULE - ARITHMETIC SUBMODULE
    println!("--- Math::Arithmetic Module (from src/math/arithmetic.rs) ---\n");

    let a = 20;
    let b = 5;

    println!(
        "Using full path: math::arithmetic::add({}, {}) = {}",
        a,
        b,
        math::arithmetic::add(a, b)
    );
    println!(
        "Using re-export: arithmetic::add({}, {}) = {}",
        a,
        b,
        arithmetic::add(a, b)
    );

    println!("Arithmetic operations:");
    println!("  {} + {} = {}", a, b, arithmetic::add(a, b));
    println!("  {} - {} = {}", a, b, arithmetic::subtract(a, b));
    println!("  {} * {} = {}", a, b, arithmetic::multiply(a, b));

    match arithmetic::divide(a, b) {
        Some(result) => println!("  {} / {} = {}", a, b, result),
        None => println!("  Division by zero!"),
    }

    match arithmetic::modulo(a, b) {
        Some(result) => println!("  {} % {} = {}", a, b, result),
        None => println!("  Modulo by zero!"),
    }
    println!();

    // MATH MODULE - GEOMETRY SUBMODULE
    println!("--- Math::Geometry Module (from src/math/geometry.rs) ---\n");

    let circle_radius = 5.0;
    let rect_width = 8.0;
    let rect_height = 6.0;

    println!("Circle (r={})", circle_radius);
    println!("  Area: {:.2}", geometry::circle_area(circle_radius));
    println!(
        "  Circumference: {:.2}",
        geometry::circle_circumference(circle_radius)
    );

    println!("Rectangle ({}x{})", rect_width, rect_height);
    println!(
        "  Area: {:.2}",
        geometry::rectangle_area(rect_width, rect_height)
    );
    println!(
        "  Perimeter: {:.2}",
        geometry::rectangle_perimeter(rect_width, rect_height)
    );

    println!("Triangle (base=10, height=8)");
    println!("  Area: {:.2}", geometry::triangle_area(10.0, 8.0));

    println!("Sphere (r=3)");
    println!("  Volume: {:.2}", geometry::sphere_volume(3.0));
    println!();

    // SHAPES MODULE - RECTANGLE
    println!("--- Shapes::Rectangle (from src/shapes/rectangle.rs) ---\n");

    let mut rect = Rectangle::new(12.0, 8.0);
    println!("{}", rect);
    println!("  Perimeter: {:.2}", rect.perimeter());
    println!("  Diagonal: {:.2}", rect.diagonal());
    println!("  Is square: {}", rect.is_square());

    rect.scale(0.5);
    println!("After scaling by 0.5: {}", rect);
    println!();

    // SHAPES MODULE - CIRCLE
    println!("--- Shapes::Circle (from src/shapes/circle.rs) ---\n");

    let mut circle = Circle::new(7.0);
    println!("{}", circle);
    println!("  Circumference: {:.2}", circle.circumference());
    println!("  Diameter: {:.2}", circle.diameter());

    circle.scale(1.5);
    println!("After scaling by 1.5: {}", circle);

    println!(
        "Point (5, 5) inside circle: {}",
        circle.contains_point(5.0, 5.0)
    );
    println!(
        "Point (15, 15) inside circle: {}",
        circle.contains_point(15.0, 15.0)
    );
    println!();

    // MODULE ORGANIZATION BENEFITS
    println!("--- Benefits of Multi-File Module Organization ---\n");

    println!("1. 📁 SEPARATION OF CONCERNS");
    println!("   - Each module in its own file");
    println!("   - Easy to find and maintain code");
    println!("   - Follows single responsibility principle\n");

    println!("2. 🏗️ CLEAR HIERARCHY");
    println!("   - Nested modules in folders (math/, shapes/)");
    println!("   - mod.rs files declare the module structure");
    println!("   - Parent modules organize related submodules\n");

    println!("3. 🔒 PRIVACY CONTROL");
    println!("   - Private functions only in their module");
    println!("   - pub keyword exports what should be public");
    println!("   - lib.rs re-exports commonly used items\n");

    println!("4. 🔄 RE-EXPORTS");
    println!("   - lib.rs re-exports from nested modules");
    println!("   - Makes API simpler: use::Rectangle instead of shapes::rectangle::Rectangle");
    println!("   - Users don't need to know internal structure\n");

    println!("5. 📦 SCALABILITY");
    println!("   - Easy to add new modules without changing existing files");
    println!("   - Can grow to hundreds of modules");
    println!("   - Clear import statements show dependencies\n");

    println!("--- Module Import Patterns ---\n");

    println!("Pattern 1: Full path");
    println!("  math::arithmetic::add(2, 3)\n");

    println!("Pattern 2: Using 'use' import");
    println!("  use math::arithmetic;");
    println!("  arithmetic::add(2, 3)\n");

    println!("Pattern 3: Re-exported from lib.rs");
    println!("  use package_crates_module::arithmetic;");
    println!("  arithmetic::add(2, 3)\n");

    println!("Pattern 4: Bring item into scope");
    println!("  use shapes::Rectangle;");
    println!("  let r = Rectangle::new(5.0, 10.0);\n");

    visibility_example();
}

// Helper function to demonstrate visibility
fn visibility_example() {
    println!("--- Visibility and Module Access ---\n");

    println!("From main.rs (binary crate), we can:");
    println!("  ✓ Access greetings::public_hello() - it's pub in lib.rs");
    println!("  ✓ Access math::arithmetic::add - nested pub module");
    println!("  ✓ Use Rectangle::new - re-exported from lib.rs");
    println!("  ✗ Cannot access private functions in modules - they're hidden");
    println!("  ✓ Can use full path: math::arithmetic::add(2,3)");
    println!("  ✓ Can use re-export: arithmetic::add(2,3)");
    println!();
    println!("PRIVACY RULE: Private by default!");
    println!("  - Functions without 'pub' are private");
    println!("  - Modules without 'pub' are private");
    println!("  - Must explicitly mark items with 'pub' to export them");
}

// DIRECTORY STRUCTURE EXPLANATION:
//
// After running this example, your project looks like:
//
// package_crates_module/
// ├── Cargo.toml                    ← Package definition
// └── src/
//     ├── main.rs                   ← Binary crate root (executable)
//     ├── lib.rs                    ← Library crate root
//     ├── greetings.rs              ← greetings module
//     ├── math/
//     │   ├── mod.rs                ← math module root (declares submodules)
//     │   ├── arithmetic.rs         ← math::arithmetic submodule
//     │   └── geometry.rs           ← math::geometry submodule
//     └── shapes/
//         ├── mod.rs                ← shapes module root (declares submodules)
//         ├── rectangle.rs          ← shapes::rectangle submodule
//         └── circle.rs             ← shapes::circle submodule
//
// HOW MODULES CONNECT:
//
// 1. lib.rs declares public modules:
//    pub mod math;      ← Loads src/math/mod.rs
//    pub mod shapes;    ← Loads src/shapes/mod.rs
//
// 2. math/mod.rs declares submodules:
//    pub mod arithmetic; ← Loads src/math/arithmetic.rs
//    pub mod geometry;   ← Loads src/math/geometry.rs
//
// 3. main.rs imports from lib crate:
//    use package_crates_module::{math, shapes, ...};
//
// KEY CONCEPTS:
//
// PACKAGE: The whole project (Cargo.toml)
//
// CRATE: A compilation unit
//   - src/lib.rs  → Library crate (can be imported by other projects)
//   - src/main.rs → Binary crate (produces an executable)
//
// MODULE: Organizes code within a crate
//   - mod.rs files are module roots in folders
//   - .rs files are modules (except main.rs and lib.rs)
//   - Use 'pub' to make modules/functions/structs public
//   - Private by default (most restrictive)
//
// PATH RESOLUTION:
//   - Absolute: math::arithmetic::add(2, 3)
//   - With re-export: arithmetic::add(2, 3)
//   - From different crate: crate_name::module::item
//
// FILE NAMING:
//   - src/module_name.rs       → Single file module
//   - src/module_name/mod.rs   → Folder-based module (has submodules)
