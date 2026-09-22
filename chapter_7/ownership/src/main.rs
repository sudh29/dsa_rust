fn main() {
    println!("=== Rust Ownership Concept ===\n");

    // OWNERSHIP BASICS
    // Every value in Rust has a single owner
    // When the owner goes out of scope, the value is deallocated

    println!("--- Ownership Transfer (Move) ---");

    // s1 owns the String
    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    // When we assign s1 to s2, ownership MOVES to s2
    // s1 is no longer valid (this is called a "move")
    let s2 = s1;
    println!("s2 = {}", s2);

    // This would cause an error because s1 no longer owns the data:
    // println!("s1 = {}", s1);  // ERROR: value borrowed after move

    println!("\n--- Ownership with Functions ---");

    let s3 = String::from("world");
    println!("Before function call: s3 = {}", s3);

    // When we pass s3 to a function, ownership is transferred
    takes_ownership(s3);

    // s3 is no longer valid here because ownership was moved into the function
    // println!("After function call: s3 = {}", s3);  // ERROR

    println!("\n--- Function Returns Ownership ---");

    // The function returns a new String, and ownership is transferred to s4
    let s4 = gives_ownership();
    println!("s4 = {}", s4);

    // This function returns the ownership of s5 back to the caller
    let s5 = String::from("rust");
    let s6 = takes_and_gives_back(s5);
    // s5 is no longer valid, but s6 is valid
    println!("s6 = {}", s6);
    // println!("s5 = {}", s5);  // ERROR: ownership was moved

    println!("\n--- Borrowing (References) ---");
    // Borrowing: allows you to use a value without taking ownership
    // A reference is created with & and doesn't take ownership

    let s7 = String::from("borrowing");
    let length = calculate_length(&s7);

    println!("s7 = {}, length = {}", s7, length);
    // s7 is still valid! The function only borrowed it

    println!("\n--- Mutable References ---");
    // Mutable references allow you to modify borrowed data
    // Created with &mut and requires the variable to be mutable

    let mut s8 = String::from("hello");
    println!("Before: s8 = {}", s8);

    modify_string(&mut s8);
    println!("After: s8 = {}", s8);

    println!("\n--- Borrowing Rules ---");
    // Rule 1: You can have either one mutable reference OR multiple immutable references

    let s9 = String::from("rust");

    // Multiple immutable references are allowed (read-only)
    let ref1 = &s9;
    let ref2 = &s9;
    let ref3 = &s9;
    println!("ref1 = {}, ref2 = {}, ref3 = {}", ref1, ref2, ref3);

    // After immutable references are done being used, we can create a mutable one
    let mut s10 = String::from("ownership");
    let ref_mut = &mut s10;
    modify_string(ref_mut);
    println!("Modified via mutable reference: {}", s10);

    // We can have multiple mutable references as long as they don't overlap
    // This is NOT allowed (would cause error):
    // let mut s11 = String::from("test");
    // let ref_mut1 = &mut s11;
    // let ref_mut2 = &mut s11;  // ERROR: cannot borrow as mutable more than once

    println!("\n--- Stack vs Heap ---");
    // Copy types (stored on stack): integers, floats, booleans, chars
    // Non-copy types (stored on heap): String, Vec, etc.

    // Integers are Copy - they don't move
    let x = 5;
    let y = x; // x is copied, not moved
    println!("x = {}, y = {}", x, y); // Both are still valid!

    // Strings are not Copy - they move
    let s11 = String::from("stack vs heap");
    let _s12 = s11; // ownership moved
    // println!("s11 = {}", s11);  // ERROR

    println!("\n--- Cloning (Deep Copy) ---");
    // If you want a deep copy of heap data, use .clone()

    let s13 = String::from("clone example");
    let s14 = s13.clone(); // Creates a deep copy

    println!("s13 = {}", s13); // Both are valid!
    println!("s14 = {}", s14);

    // Additional borrowing demonstrations
    borrowing_rules_demo();

    // Demonstrating borrow_immutable function
    let s_borrow = String::from("immutable borrow");
    borrow_immutable(&s_borrow);

    // Demonstrating borrow_mutable function
    let mut s_mutable = String::from("mutable borrow");
    borrow_mutable(&mut s_mutable);
    println!("After borrow_mutable: {}", s_mutable);

    // Demonstrating first_word function
    let sentence = String::from("hello rust");
    let fw = first_word(&sentence);
    println!("First word using first_word(): {}", fw);

    // Slicing demonstrations
    string_slicing_demo();
    array_slicing_demo();
    borrowing_in_loops_demo();
}

// Takes ownership of a String
fn takes_ownership(s: String) {
    println!("Inside takes_ownership: {}", s);
    // s goes out of scope here, the String is deallocated
}

// Returns ownership of a new String
fn gives_ownership() -> String {
    let s = String::from("this is given");
    s // s is moved out and returned
}

// Takes ownership and returns it back
fn takes_and_gives_back(s: String) -> String {
    println!("Inside takes_and_gives_back: {}", s);
    s // Ownership is returned
}

// Borrows a reference - doesn't take ownership
// & means "immutable reference" (read-only)
fn calculate_length(s: &String) -> usize {
    println!("Inside calculate_length: {}", s);
    s.len()
    // s goes out of scope, but it doesn't own the String, so nothing happens
}

// Borrows a mutable reference - can modify the data
// &mut means "mutable reference" (can read and write)
fn modify_string(s: &mut String) {
    s.push_str(" world!");
    println!("Inside modify_string: {}", s);
}

// BORROWING EXAMPLES
// ==================

// Function that takes an immutable reference
// Receives a borrowed reference but doesn't take ownership
fn borrow_immutable(s: &String) {
    println!("Borrowed immutably: {}", s);
}

// Function that takes a mutable reference
// Can read and modify the borrowed data
fn borrow_mutable(s: &mut String) {
    s.push_str(" [modified]");
    println!("Borrowed mutably and modified: {}", s);
}

// Function that demonstrates borrowing rules
fn borrowing_rules_demo() {
    println!("\n=== Borrowing Rules Demo ===\n");

    let s = String::from("hello");

    // Multiple immutable borrows are allowed
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!(
        "Multiple immutable borrows: r1={}, r2={}, r3={}",
        r1, r2, r3
    );

    // After using immutable references, we can borrow mutably
    let mut s_mut = String::from("mutable");
    let r_mut = &mut s_mut;
    r_mut.push_str(" string");
    println!("After mutable borrow: {}", r_mut);
}

// SLICING EXAMPLES
// ================

// Function that takes a string slice
// String slices are references to a portion of a String
// Type &str represents a string slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' is the byte value of space
            return &s[0..i];
        }
    }
    &s[..] // Return entire string if no space found
}

// Function with slice parameter (more flexible)
// &str parameter accepts both String and &str
fn first_word_flexible(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Function to demonstrate array slicing
fn array_slicing_demo() {
    println!("\n=== Array Slicing Demo ===\n");

    let arr = [1, 2, 3, 4, 5];

    // Array slice: reference to a portion of an array
    let slice1 = &arr[0..3]; // Elements 0, 1, 2 (excludes 3)
    println!("arr[0..3] = {:?}", slice1);

    let slice2 = &arr[2..]; // From index 2 to end
    println!("arr[2..] = {:?}", slice2);

    let slice3 = &arr[..3]; // From start to index 3 (excludes 3)
    println!("arr[..3] = {:?}", slice3);

    let slice4 = &arr[..]; // Entire array
    println!("arr[..] = {:?}", slice4);
}

// Function to demonstrate string slicing
fn string_slicing_demo() {
    println!("\n=== String Slicing Demo ===\n");

    let s = String::from("hello world");

    // String slices
    let hello = &s[0..5]; // "hello"
    println!("s[0..5] = {}", hello);

    let world = &s[6..11]; // "world"
    println!("s[6..11] = {}", world);

    let hello_world = &s[..]; // entire string
    println!("s[..] = {}", hello_world);

    // Using first_word function with string slice
    let word = first_word_flexible(&s);
    println!("First word: {}", word);

    // String literals are slices
    let s_literal = "hello"; // Type is &str (string slice)
    println!("String literal (slice): {}", s_literal);
}

// Function demonstrating borrowing in loops
fn borrowing_in_loops_demo() {
    println!("\n=== Borrowing in Loops Demo ===\n");

    let mut vec = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", vec);

    // Immutable borrow in loop (can iterate multiple times)
    println!("Iterating with immutable borrow:");
    for &num in &vec {
        // & creates immutable reference, & in pattern borrows
        println!("  {}", num);
    }

    // Mutable borrow in loop
    println!("Iterating with mutable borrow:");
    for num in &mut vec {
        // &mut creates mutable reference
        *num *= 2; // * dereferences the mutable reference
        println!("  {}", num);
    }
    println!("Vector after mutation: {:?}", vec);
}
