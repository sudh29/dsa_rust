use std::collections::HashMap;

fn main() {
    println!("=== Rust Collections: Vectors, Strings, and HashMaps ===\n");

    // VECTORS
    vectors_examples();

    println!();

    // STRINGS
    strings_examples();

    println!();

    // HASHMAPS
    hashmap_examples();
}

// VECTORS - Dynamic arrays that can grow or shrink
// ================================================

fn vectors_examples() {
    println!("--- VECTORS (Dynamic Arrays) ---\n");

    // Creating vectors
    println!("1. Creating Vectors:\n");

    // Method 1: Using vec! macro
    let vec1 = vec![1, 2, 3, 4, 5];
    println!("vec! macro: {:?}", vec1);

    // Method 2: Using Vec::new()
    let mut vec2: Vec<i32> = Vec::new();
    println!("Vec::new(): {:?}", vec2);

    // Method 3: Using with_capacity
    let mut vec3: Vec<String> = Vec::with_capacity(3);
    println!(
        "with_capacity(3): len={}, capacity={}\n",
        vec3.len(),
        vec3.capacity()
    );

    // Adding elements
    println!("2. Adding Elements:\n");

    vec2.push(10);
    vec2.push(20);
    vec2.push(30);
    println!("After push: {:?}", vec2);

    vec3.push(String::from("apple"));
    vec3.push(String::from("banana"));
    vec3.push(String::from("cherry"));
    println!("String vector: {:?}\n", vec3);

    // Accessing elements
    println!("3. Accessing Elements:\n");

    println!("vec1[0] = {}", vec1[0]);
    println!("vec1.get(2) = {:?}", vec1.get(2));
    println!("vec1.get(100) = {:?}", vec1.get(100)); // Returns None (safe)
    // println!("vec1[100] = {}", vec1[100]);  // Would panic!
    println!();

    // Iterating vectors
    println!("4. Iterating Vectors:\n");

    println!("Immutable iteration:");
    for (index, value) in vec1.iter().enumerate() {
        println!("  vec1[{}] = {}", index, value);
    }

    // Mutable iteration
    let mut vec4 = vec![1, 2, 3];
    println!("\nMutable iteration (doubling values):");
    for value in &mut vec4 {
        *value *= 2;
    }
    println!("After doubling: {:?}", vec4);

    // Consuming iteration
    let vec5 = vec![10, 20, 30];
    println!("\nConsuming iteration:");
    for value in vec5 {
        println!("  value = {}", value);
    }
    // vec5 is no longer valid after consuming iteration

    println!();

    // Vector methods
    println!("5. Vector Methods:\n");

    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("Original: {:?}", numbers);
    println!("Length: {}", numbers.len());
    println!("Is empty: {}", numbers.is_empty());
    println!("First element: {:?}", numbers.first());
    println!("Last element: {:?}", numbers.last());

    numbers.pop();
    println!("After pop(): {:?}", numbers);

    numbers.remove(0);
    println!("After remove(0): {:?}", numbers);

    numbers.sort();
    println!("After sort(): {:?}", numbers);

    println!("\nReverse: {:?}", {
        let mut temp = numbers.clone();
        temp.reverse();
        temp
    });

    // Collecting into vector
    println!("\n6. Creating Vectors from Iterators:\n");

    let squared: Vec<i32> = (1..6).map(|x| x * x).collect();
    println!("Squares 1-5: {:?}", squared);

    let evens: Vec<i32> = (1..11).filter(|x| x % 2 == 0).collect();
    println!("Even numbers 1-10: {:?}", evens);

    println!();
}

// STRINGS - Text data in Rust
// =============================

fn strings_examples() {
    println!("--- STRINGS ---\n");

    // String types
    println!("1. String Types:\n");

    // &str - String slice (immutable, fixed size, often in static memory)
    let str_literal: &str = "Hello"; // &str (string slice)
    println!("String literal (&str): {}", str_literal);

    // String - Owned string (heap-allocated, mutable, growable)
    let string_owned = String::from("Hello"); // String (owned)
    println!("String::from(): {}", string_owned);

    let string_macro = String::from("World");
    println!("String::from(): {}\n", string_macro);

    // Creating strings
    println!("2. Creating Strings:\n");

    let s1 = String::from("Hello");
    let s2 = "World".to_string();
    let s3 = format!("Hello, {}!", "Rust");

    println!("String::from(): {}", s1);
    println!(".to_string(): {}", s2);
    println!("format!: {}\n", s3);

    // String concatenation
    println!("3. String Concatenation:\n");

    let hello = String::from("Hello");
    let world = String::from("World");

    // Method 1: Using + operator (takes ownership of first string)
    let result1 = hello + " " + &world;
    println!("Using + operator: {}", result1);

    // Method 2: Using format! macro (doesn't consume strings)
    let hello2 = String::from("Hello");
    let world2 = String::from("World");
    let result2 = format!("{} {}", hello2, world2);
    println!("Using format!: {}\n", result2);

    // String length and capacity
    println!("4. String Properties:\n");

    let text = String::from("Hello, Rust!");
    println!("Text: {}", text);
    println!("Len (bytes): {}", text.len());
    println!("Capacity: {}", text.capacity());

    // Unicode example
    let unicode = String::from("Hello 🦀");
    println!("\nUnicode text: {}", unicode);
    println!(
        "Byte length: {} (emojis take multiple bytes)",
        unicode.len()
    );
    println!("Char count: {}", unicode.chars().count());
    println!();

    // String methods
    println!("5. String Methods:\n");

    let mut s = String::from("Hello Rust");

    println!("Original: {}", s);
    println!("Uppercase: {}", s.to_uppercase());
    println!("Lowercase: {}", s.to_lowercase());
    println!("Contains 'Rust': {}", s.contains("Rust"));
    println!("Starts with 'Hello': {}", s.starts_with("Hello"));
    println!("Ends with 'Rust': {}", s.ends_with("Rust"));

    // Mutation
    s.push_str(" is awesome!");
    println!("\nAfter push_str: {}", s);

    s.push('!');
    println!("After push: {}", s);

    // Slicing strings
    println!("\n6. String Slicing:\n");

    let s = String::from("Hello, World!");
    let hello = &s[0..5];
    let world = &s[7..12];

    println!("Full string: {}", s);
    println!("s[0..5]: {}", hello);
    println!("s[7..12]: {}\n", world);

    // Iterating strings
    println!("7. Iterating Strings:\n");

    let s = String::from("Hello");

    println!("Iterating bytes:");
    for byte in s.as_bytes() {
        print!("{} ", byte);
    }
    println!();

    println!("Iterating chars:");
    for ch in s.chars() {
        print!("{} ", ch);
    }
    println!("\n");

    // String parsing
    println!("8. String Parsing:\n");

    let num_str = "42";
    let num: i32 = num_str.parse().expect("Failed to parse");
    println!("Parsed '{}' as i32: {}", num_str, num);

    let float_str = "3.14";
    let float: f64 = float_str.parse().expect("Failed to parse");
    println!("Parsed '{}' as f64: {}\n", float_str, float);
}

// HASHMAPS - Key-value collections
// ==================================

fn hashmap_examples() {
    println!("--- HASHMAPS (Key-Value Collections) ---\n");

    // Creating hashmaps
    println!("1. Creating HashMaps:\n");

    // Method 1: Using HashMap::new()
    let mut map1: HashMap<String, i32> = HashMap::new();
    println!("HashMap::new(): {:?}", map1);

    // Method 2: Using collect from tuples
    let pairs = vec![("one", 1), ("two", 2), ("three", 3)];
    let map2: HashMap<&str, i32> = pairs.iter().cloned().collect();
    println!("From tuples: {:?}\n", map2);

    // Inserting values
    println!("2. Inserting Values:\n");

    map1.insert(String::from("Alice"), 85);
    map1.insert(String::from("Bob"), 92);
    map1.insert(String::from("Charlie"), 78);
    map1.insert(String::from("Diana"), 95);

    println!("After inserts: {:?}\n", map1);

    // Accessing values
    println!("3. Accessing Values:\n");

    if let Some(score) = map1.get("Alice") {
        println!("Alice's score: {}", score);
    }

    println!("Bob's score: {:?}", map1.get("Bob"));
    println!("Eve's score: {:?}", map1.get("Eve")); // Returns None

    // Check if key exists
    println!("Contains 'Charlie': {}", map1.contains_key("Charlie"));
    println!("Contains 'Frank': {}\n", map1.contains_key("Frank"));

    // Updating values
    println!("4. Updating Values:\n");

    // Insert (overwrites if exists)
    map1.insert(String::from("Alice"), 90);
    println!("After updating Alice to 90: {:?}", map1.get("Alice"));

    // Entry API (more efficient for conditional inserts)
    map1.entry(String::from("Eve")).or_insert(88);
    map1.entry(String::from("Alice")).or_insert(999); // Won't insert (exists)

    println!("After entry().or_insert():");
    println!("  Eve: {:?}", map1.get("Eve"));
    println!("  Alice: {:?}\n", map1.get("Alice"));

    // Iterating hashmaps
    println!("5. Iterating HashMaps:\n");

    println!("All entries:");
    for (name, score) in &map1 {
        println!("  {}: {}", name, score);
    }

    println!();

    // HashMap methods
    println!("6. HashMap Methods:\n");

    println!("Number of entries: {}", map1.len());
    println!("Is empty: {}", map1.is_empty());

    // Collecting keys and values
    let keys: Vec<&String> = map1.keys().collect();
    let values: Vec<&i32> = map1.values().collect();

    println!("Keys: {:?}", keys);
    println!("Values: {:?}", values);

    println!();

    // Removing values
    println!("7. Removing Values:\n");

    println!("Before remove: {}", map1.len());
    map1.remove("Bob");
    println!("After removing Bob: {}", map1.len());

    println!("Bob's score: {:?}\n", map1.get("Bob"));

    // Real-world example: Word frequency counter
    println!("8. Real-World Example: Word Frequency\n");

    let text = "the quick brown fox jumps over the lazy dog the fox jumps";
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in words {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Text: {}", text);
    println!("Word frequencies:");
    let mut sorted_words: Vec<_> = word_count.iter().collect();
    sorted_words.sort_by_key(|&(_, count)| count);
    sorted_words.reverse();

    for (word, count) in sorted_words {
        println!("  '{}': {}", word, count);
    }

    println!();

    // Another example: Student grades lookup
    println!("9. Student Grades Lookup\n");

    let mut grades: HashMap<String, Vec<i32>> = HashMap::new();

    grades.insert(String::from("Math"), vec![90, 85, 88]);
    grades.insert(String::from("Science"), vec![92, 88, 91]);
    grades.insert(String::from("English"), vec![88, 92, 89]);

    for (subject, scores) in &grades {
        let avg = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
        println!("{}: {:?} (avg: {:.1})", subject, scores, avg);
    }

    println!();

    // Comparison of Collections
    println!("--- Collections Comparison ---\n");

    println!("VECTOR:");
    println!("  ✓ Ordered sequence");
    println!("  ✓ Fast indexed access");
    println!("  ✓ Easy to iterate");
    println!("  ✗ No key lookups\n");

    println!("STRING:");
    println!("  ✓ Text data");
    println!("  ✓ UTF-8 encoded");
    println!("  ✓ Growable");
    println!("  ✗ Can't index by char (variable-width encoding)\n");

    println!("HASHMAP:");
    println!("  ✓ Fast key lookups");
    println!("  ✓ Key-value associations");
    println!("  ✗ Unordered");
    println!("  ✗ More memory overhead");
}
