// ENUM DEFINITION
// An enum allows you to define a type by enumerating its possible variants
// Each variant can optionally hold data

// Basic enum without associated data
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with associated data (each variant can hold different types/amounts of data)
#[derive(Debug)]
enum Message {
    Quit,                       // variant with no data
    Move { x: i32, y: i32 },    // variant with named fields
    Write(String),              // variant with a single value
    ChangeColor(i32, i32, i32), // variant with multiple values
}

// Enum representing a value that can be present (Some) or absent (None)
// This is similar to Rust's built-in Option<T> enum
#[derive(Debug)]
enum OptionCustom<T> {
    Some(T), // Variant that holds a value
    None,    // Variant representing no value
}

// Enum for Result handling
#[derive(Debug)]
enum ResultCustom<T, E> {
    Ok(T),  // Success variant with a value
    Err(E), // Error variant with an error value
}

// Coin enum for match examples
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("=== Rust Enums and Pattern Matching ===\n");

    // BASIC ENUM USAGE
    println!("--- Basic Enum Usage ---\n");

    let direction = Direction::North;
    println!("Direction: {:?}", direction);

    // Using all Direction variants
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    for dir in &directions {
        println!("  {:?}", dir);
    }

    let message = Message::Write(String::from("Hello, Rust!"));
    println!("Message: {:?}\n", message);

    // MATCH EXPRESSION - THE CORE OF PATTERN MATCHING
    println!("--- Match Expression (Match Type) ---\n");

    // Match is an expression that compares a value against patterns
    // It's exhaustive - you must handle all possible cases
    let coins = vec![Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];

    for coin in coins {
        // Basic match expression
        let value = match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => {
                println!("A nickel!");
                5
            }
            Coin::Dime => {
                println!("A dime!");
                10
            }
            Coin::Quarter => {
                println!("A quarter!");
                25
            }
        };
        println!("Coin value: {} cents", value);
    }
    println!();

    // MATCH WITH ENUMS HOLDING DATA
    println!("--- Match with Associated Data ---\n");

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello!")),
        Message::ChangeColor(128, 255, 90),
    ];

    for msg in messages {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to unpack.");
            }
            Message::Move { x, y } => {
                println!("Move to coordinates: x={}, y={}", x, y);
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB: ({}, {}, {})", r, g, b);
            }
        }
    }
    println!();

    // MATCH CONTROL FLOW
    println!("\n--- Match Control Flow ---\n");

    let numbers = vec![1, 2, 3, 4, 5];

    for num in &numbers {
        match num {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            4 => println!("Four"),
            5 => println!("Five"),
            _ => println!("Other number: {}", num), // _ matches anything (catch-all)
        }
    }

    println!();

    // MATCH WITH RANGES
    println!("--- Match with Ranges ---\n");

    let score = 85;

    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("Score: {}, Grade: {}\n", score, grade);

    // MATCH WITH GUARDS
    println!("--- Match with Guards (extra conditions) ---\n");

    let age = 25;

    match age {
        n if n < 13 => println!("Child"),
        n if n < 18 => println!("Teenager"),
        n if n < 65 => println!("Adult"),
        _ => println!("Senior"),
    }

    println!();

    // DESTRUCTURING IN MATCH
    println!("--- Destructuring with Match ---\n");

    let point = (3, 5);

    match point {
        (0, 0) => println!("At the origin"),
        (x, 0) => println!("On the x-axis at x={}", x),
        (0, y) => println!("On the y-axis at y={}", y),
        (x, y) => println!("At point ({}, {})", x, y),
    }

    println!();

    // CUSTOM ENUM EXAMPLES
    println!("--- Using Custom OptionCustom Enum ---\n");

    let custom_some: OptionCustom<i32> = OptionCustom::Some(42);
    let custom_none: OptionCustom<i32> = OptionCustom::None;

    match &custom_some {
        OptionCustom::Some(value) => println!("Custom Some: {}", value),
        OptionCustom::None => println!("Custom None"),
    }

    match &custom_none {
        OptionCustom::Some(value) => println!("Custom Some: {}", value),
        OptionCustom::None => println!("Custom None"),
    }

    println!();

    // CUSTOM RESULT ENUM EXAMPLES
    println!("--- Using Custom ResultCustom Enum ---\n");

    let custom_ok: ResultCustom<String, String> = ResultCustom::Ok(String::from("Success!"));
    let custom_err: ResultCustom<String, String> = ResultCustom::Err(String::from("Failed!"));

    match &custom_ok {
        ResultCustom::Ok(msg) => println!("Custom Ok: {}", msg),
        ResultCustom::Err(e) => println!("Custom Err: {}", e),
    }

    match &custom_err {
        ResultCustom::Ok(msg) => println!("Custom Ok: {}", msg),
        ResultCustom::Err(e) => println!("Custom Err: {}", e),
    }

    println!();
    println!("--- If Let Control Flow ---\n");

    // if let is syntactic sugar for a match with only one pattern you care about
    // Useful when you only care about one specific variant

    let config_value = Some(3u8);

    // Instead of:
    // match config_value {
    //     Some(value) => println!("Config value: {}", value),
    //     None => {},
    // }

    // You can use if let:
    if let Some(value) = config_value {
        println!("Config value is: {}", value);
    }

    println!();

    // IF LET WITH ENUM
    println!("--- If Let with Custom Enum ---\n");

    let message1 = Message::Write(String::from("Hello"));
    let message2 = Message::Quit;

    // Only execute if message matches Write variant
    if let Message::Write(text) = message1 {
        println!("Got a write message: {}", text);
    }

    // This won't match, so nothing prints
    if let Message::Write(text) = message2 {
        println!("Got a write message: {}", text);
    } else {
        println!("message2 is not a Write variant");
    }

    println!();

    // IF LET VS MATCH
    println!("--- If Let vs Match ---\n");

    let number = Some(5);

    // Using match (more verbose but handles all cases)
    match number {
        Some(n) => println!("Using match - Number: {}", n),
        None => {}
    }

    // Using if let (concise when you only care about one pattern)
    if let Some(n) = number {
        println!("Using if let - Number: {}", n);
    }

    println!();

    // PRACTICAL EXAMPLE: RESULT HANDLING
    println!("--- Result Enum Usage ---\n");

    let result_ok: Result<i32, String> = Ok(42);
    let result_err: Result<i32, String> = Err(String::from("Failed to compute"));

    // Match on Result (using references to avoid moving values)
    match &result_ok {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match &result_err {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    // IF LET WITH RESULT
    println!("--- If Let with Result ---\n");

    if let Ok(value) = &result_ok {
        println!("Result is OK with value: {}", value);
    }

    if let Err(e) = &result_err {
        println!("Result is error: {}", e);
    }

    println!();

    // OPTION ENUM (Rust's built-in)
    println!("--- Option Enum Pattern Matching ---\n");

    let some_string = Some(String::from("Hello"));
    let absent_value: Option<String> = None;

    // Match on Option
    match some_string {
        Some(s) => println!("Got string: {}", s),
        None => println!("Got nothing"),
    }

    // If let with Option
    if let Some(s) = absent_value {
        println!("Got string: {}", s);
    } else {
        println!("Value is None");
    }

    println!();

    // MULTIPLE PATTERNS IN MATCH
    println!("--- Multiple Patterns in Match ---\n");

    let day = 3;

    match day {
        1 | 7 => println!("Weekend"), // | means OR
        2..=6 => println!("Weekday"),
        _ => println!("Invalid day"),
    }

    println!();

    // IGNORING PARTS OF A PATTERN
    println!("--- Ignoring Pattern Parts ---\n");

    let origin = (0, 0, 0);

    match origin {
        (0, y, z) => println!("On yz-plane at y={}, z={}", y, z),
        _ => println!("Not on yz-plane"),
    }

    // Using _ to ignore specific values
    let point3d = (1, 2, 3);
    match point3d {
        (x, _, z) => println!("x={}, z={} (ignored y)", x, z),
    }

    println!();

    // WHILE LET - LOOP WITH PATTERN MATCHING
    println!("--- While Let Control Flow ---\n");

    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
