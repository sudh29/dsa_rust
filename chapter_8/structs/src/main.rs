#![allow(clippy::approx_constant)]

// STRUCT DEFINITION
// A struct is a custom data type that lets you package together related data
// Structs have named fields, unlike tuples

// Basic struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Struct with derived Debug trait
// #[derive(Debug)] automatically implements Debug trait for printing
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Struct with tuple-like syntax (tuple struct)
// Fields don't have names, just types
struct Color(i32, i32, i32);
struct Point(f64, f64);

// Unit struct (no fields)
struct AlwaysEqual;

// IMPLEMENTATION BLOCKS
// impl blocks define methods and associated functions for structs

impl Rectangle {
    // Associated function: doesn't take &self, called with :: operator
    // Used for constructors
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method: takes &self (immutable reference to self)
    // Allows reading data without taking ownership
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Method that takes &mut self (mutable reference)
    // Allows modifying the struct
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // Method that takes self (takes ownership)
    // The struct cannot be used after calling this method
    fn into_square(self) -> Rectangle {
        let side = self.width.min(self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }
}

// Separate impl block for Rectangle (allowed in Rust)
impl Rectangle {
    // Another method in a different impl block
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    println!("=== Rust Structs, Methods, and Debug Macro ===\n");

    // STRUCT BASICS
    println!("--- Basic Struct Usage ---\n");

    // Creating an instance of User
    let mut user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("User: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign in count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);

    // Modifying struct fields (requires mut)
    user1.email = String::from("alice_new@example.com");
    println!("Updated email: {}\n", user1.email);

    // Using a function to create structs
    let user2 = build_user(String::from("bob@example.com"), String::from("bob"));
    println!("Created user: {}\n", user2.username);

    // Struct update syntax (copy fields from another struct)
    let user3 = User {
        email: String::from("charlie@example.com"),
        username: String::from("charlie"),
        ..user2 // Use remaining fields from user2
    };
    println!("User with update syntax: {}\n", user3.username);

    // TUPLE STRUCTS
    println!("--- Tuple Structs ---\n");

    let color = Color(255, 0, 0);
    println!("Color: ({}, {}, {})", color.0, color.1, color.2);

    let point = Point(3.14, 2.71);
    println!("Point: ({}, {})\n", point.0, point.1);

    // UNIT STRUCT
    println!("--- Unit Struct ---\n");
    let _subject = AlwaysEqual;
    println!("Unit struct instance created\n");

    // DEBUG MACRO
    println!("--- Debug Macro (Derived) ---\n");

    let rect1 = Rectangle::new(30, 50);

    // {:?} uses Debug trait (single line format)
    println!("Rectangle using {{:?}}: {:?}", rect1);

    // {:#?} uses Debug trait (pretty-printed format)
    println!("Rectangle using {{:#?}}:\n{:#?}\n", rect1);

    // METHODS AND ASSOCIATED FUNCTIONS
    println!("--- Methods and Associated Functions ---\n");

    // Using associated function (new)
    let rect2 = Rectangle::new(20, 40);
    println!("Rectangle created with new: {:?}", rect2);

    // Using methods (note the . operator)
    let area = rect2.area();
    println!("Area: {} square pixels", area);

    let perimeter = rect2.perimeter();
    println!("Perimeter: {} pixels\n", perimeter);

    // METHOD WITH PARAMETERS
    println!("--- Methods with Parameters ---\n");

    let rect3 = Rectangle::new(25, 35);
    let rect4 = Rectangle::new(10, 20);

    println!("rect3: {:?}", rect3);
    println!("rect4: {:?}", rect4);
    println!("Can rect3 hold rect4? {}\n", rect3.can_hold(&rect4));

    // MUTABLE METHOD
    println!("--- Mutable Method (&mut self) ---\n");

    let mut rect5 = Rectangle::new(15, 25);
    println!("Before scale: {:?}", rect5);
    rect5.scale(2);
    println!("After scale(2): {:?}\n", rect5);

    // METHOD TAKING OWNERSHIP
    println!("--- Method Taking Ownership (self) ---\n");

    let rect6 = Rectangle::new(10, 10);
    println!("Original: {:?}", rect6);

    // into_square takes ownership
    let square = rect6.into_square();
    println!("After into_square: {:?}", square);
    // rect6 is no longer valid here!
    // println!("{:?}", rect6);  // ERROR: value used after move

    println!("\n--- Multiple Structs with Debug ---\n");

    // Debug macro useful for displaying complex data structures
    let users = vec![
        User {
            username: String::from("alice"),
            email: String::from("alice@example.com"),
            sign_in_count: 5,
            active: true,
        },
        User {
            username: String::from("bob"),
            email: String::from("bob@example.com"),
            sign_in_count: 3,
            active: false,
        },
    ];

    // Can't print User directly without Debug trait, but we can print with debug macro
    println!("Users (using index access):");
    for user in &users {
        let status = if user.active { "active" } else { "inactive" };
        println!("  - {} ({}) [{}]", user.username, user.email, status);
    }

    println!("\n--- Struct Comparison ---\n");

    let rect7 = Rectangle::new(40, 50);
    let rect8 = Rectangle::new(40, 50);

    println!("rect7: {:?}", rect7);
    println!("rect8: {:?}", rect8);
    println!("Area rect7: {}", rect7.area());
    println!("Area rect8: {}", rect8.area());
}

// Helper function to create User structs
fn build_user(email: String, username: String) -> User {
    User {
        email,    // field init shorthand (same as email: email)
        username, // field init shorthand
        active: true,
        sign_in_count: 1,
    }
}
