fn main() {
    println!("=== Rust Error Handling: Panic and Result ===\n");

    // PANIC EXAMPLES
    panic_examples();

    println!();

    // RESULT EXAMPLES
    result_examples();

    println!();

    // ERROR HANDLING PATTERNS
    error_handling_patterns();
}

// PANIC - Unrecoverable errors
// ==============================

fn panic_examples() {
    println!("--- PANIC (Unrecoverable Errors) ---\n");

    println!("1. What is Panic?\n");
    println!("Panic occurs when a program encounters an unrecoverable error.");
    println!("The program will stop execution and unwind the stack.\n");

    // Explicit panic with panic! macro
    println!("2. Explicit Panic with panic! macro:\n");

    // Example 1: Basic panic
    println!("Example 1: Basic panic");
    println!("Code: panic!(\"Something went wrong!\")");
    // Uncomment to see panic:
    // panic!("Something went wrong!");

    println!("(Skipped to continue execution)\n");

    // Example 2: Panic with formatted message
    println!("Example 2: Panic with formatted message");
    let _value = 42;
    println!("Code: panic!(\"Value was {{}}\", value);");
    // Uncomment to see panic:
    // panic!("Value was {}", value);

    println!("(Skipped to continue execution)\n");

    // Implicit panics
    println!("3. Implicit Panics (automatic panic conditions):\n");

    // Index out of bounds
    println!("Example 1: Index out of bounds");
    let arr = [1, 2, 3];
    println!("Array: {:?}", arr);
    println!("Accessing arr[0]: {}", arr[0]);
    println!("Code: arr[100] would panic!");
    // Uncomment to see panic:
    // println!("{}", arr[100]);

    println!("(Skipped to continue execution)\n");

    // Unwrap on None
    println!("Example 2: Unwrap on None");
    let _option: Option<i32> = None;
    println!("let option: Option<i32> = None;");
    println!("Code: option.unwrap() would panic!");
    // Uncomment to see panic:
    // option.unwrap();

    println!("(Skipped to continue execution)\n");

    // Unwrap on Err
    println!("Example 3: Unwrap on Err");
    let _result: Result<i32, String> = Err(String::from("Something failed"));
    println!("let result: Result<i32, String> = Err(...);");
    println!("Code: result.unwrap() would panic!");
    // Uncomment to see panic:
    // result.unwrap();

    println!("(Skipped to continue execution)\n");

    // expect with custom message
    println!("4. Panic with expect() and custom message:\n");

    let _file_result: Result<i32, String> = Err(String::from("File not found"));
    println!("Code: file_result.expect(\"Failed to read file\")");
    // Uncomment to see panic with message:
    // file_result.expect("Failed to read file");

    println!("(Skipped to continue execution)\n");

    // When to panic
    println!("5. When to Use Panic:\n");
    println!("✓ Use panic! when:");
    println!("  - Program hits an unrecoverable, unexcoverable state");
    println!("  - In tests");
    println!("  - In prototypes or examples");
    println!("  - When you're absolutely certain an operation won't fail\n");

    println!("✗ Don't use panic! when:");
    println!("  - Error is expected/recoverable (use Result instead)");
    println!("  - In production library code");
    println!("  - User input validation");
    println!("  - File I/O operations");
    println!("  - Network operations\n");
}

// RESULT - Recoverable errors
// =============================

fn result_examples() {
    println!("--- RESULT (Recoverable Errors) ---\n");

    println!("1. Result Type:\n");
    println!("Result<T, E> has two variants:");
    println!("  Ok(T)  - Success, contains the value");
    println!("  Err(E) - Failure, contains the error\n");

    // Creating Results
    println!("2. Creating Results:\n");

    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("Something failed"));

    println!("Success result: {:?}", success);
    println!("Failure result: {:?}\n", failure);

    // Checking if Result is Ok or Err
    println!("3. Checking Result Status:\n");

    println!("is_ok(): {}", success.is_ok());
    println!("is_err(): {}", success.is_err());
    println!("is_ok(): {}", failure.is_ok());
    println!("is_err(): {}\n", failure.is_err());

    // Extracting values
    println!("4. Extracting Values from Result:\n");

    // Method 1: match (most flexible)
    println!("Method 1: match");
    match &success {
        Ok(value) => println!("  Success: Got value {}", value),
        Err(e) => println!("  Error: {}", e),
    }

    // Method 2: if let (concise)
    println!("Method 2: if let");
    if let Ok(value) = &success {
        println!("  Got value: {}", value);
    }

    // Method 3: unwrap (panics on error)
    println!("Method 3: unwrap (dangerous!)");
    let value = success.unwrap();
    println!("  Value: {}\n", value);

    // Method 4: unwrap_or (provides default)
    println!("Method 4: unwrap_or");
    let value_or_default = failure.unwrap_or(-1);
    println!("  Value or default: {}\n", value_or_default);

    // Method 5: expect (panics with message)
    println!("Method 5: expect");
    let success2: Result<i32, String> = Ok(42);
    let value_or_panic = success2.expect("Expected success");
    println!("  Value: {}\n", value_or_panic);

    // Transforming Results
    println!("5. Transforming Results:\n");

    let num_result: Result<i32, String> = Ok(10);

    // Using map to transform the value
    let doubled = num_result.as_ref().map(|x| x * 2);
    println!("Original: {:?}", num_result);
    println!("After map(|x| x * 2): {:?}\n", doubled);

    // Using map_err to transform the error
    let error_result: Result<i32, i32> = Err(5);
    let transformed_error = error_result.map_err(|e| e * 10);
    println!("Original error: {:?}", error_result);
    println!("After map_err(|e| e * 10): {:?}\n", transformed_error);

    // Chaining Results
    println!("6. Chaining Results with and_then:\n");

    let result1: Result<i32, String> = Ok(5);
    let result2 = result1.and_then(|x| {
        if x > 0 {
            Ok(x * 2)
        } else {
            Err(String::from("Number must be positive"))
        }
    });
    println!("Chained result: {:?}\n", result2);

    // Result in functions
    println!("7. Functions Returning Result:\n");

    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}\n", e),
    }

    // Custom error types
    println!("8. Custom Error Types:\n");

    match parse_age("25") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}", e),
    }

    match parse_age("abc") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}", e),
    }

    match parse_age("-5") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Error: {}\n", e),
    }

    // Question mark operator
    println!("9. Question Mark Operator (?):\n");

    match read_config() {
        Ok(value) => println!("Config value: {}", value),
        Err(e) => println!("Config error: {}\n", e),
    }
}

// ERROR HANDLING PATTERNS
// =======================

fn error_handling_patterns() {
    println!("--- Error Handling Patterns ---\n");

    // Pattern 1: Match with all cases
    println!("Pattern 1: Complete match\n");

    let result = divide(8, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    // Pattern 2: If let for single case
    println!("Pattern 2: If let (handle one case)\n");

    if let Ok(value) = divide(8, 2) {
        println!("Result: {}", value);
    }

    println!();

    // Pattern 3: Unwrap with default
    println!("Pattern 3: Unwrap with default\n");

    let value = divide(8, 0).unwrap_or(-1);
    println!("Result: {}", value);

    println!();

    // Pattern 4: Chain operations
    println!("Pattern 4: Chaining operations\n");

    let result = divide(10, 2).map(|x| x + 5).map(|x| x * 2).unwrap_or(0);
    println!("Chained result: {}", result);

    println!();

    // Pattern 5: Error propagation
    println!("Pattern 5: Error propagation with ?\n");

    match calculate_age_next_year("30") {
        Ok(age) => println!("Next year age: {}", age),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    // Pattern 6: Multiple Results
    println!("Pattern 6: Handling multiple Results\n");

    let results = vec![divide(10, 2), divide(15, 3), divide(20, 0), divide(9, 3)];

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("Result {}: {}", i, value),
            Err(e) => println!("Result {}: Error - {}", i, e),
        }
    }

    println!();

    // Pattern 7: Converting between error types
    println!("Pattern 7: Generic error handling\n");

    match generic_parse("42") {
        Ok(num) => println!("Parsed: {}", num),
        Err(e) => println!("Error: {}", e),
    }

    match generic_parse("not a number") {
        Ok(num) => println!("Parsed: {}", num),
        Err(e) => println!("Error: {}\n", e),
    }
}

// HELPER FUNCTIONS
// ================

/// Divides two numbers, returns Result
/// Returns Err if divisor is zero
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

/// Parses a string as an age, validates it
/// Returns Err if invalid
fn parse_age(s: &str) -> Result<u32, String> {
    match s.parse::<i32>() {
        Ok(num) => {
            if num >= 0 && num <= 150 {
                Ok(num as u32)
            } else {
                Err(String::from("Age must be between 0 and 150"))
            }
        }
        Err(_) => Err(String::from("Invalid age format")),
    }
}

/// Demonstrates error propagation with ? operator
fn read_config() -> Result<String, String> {
    // Simulating reading a config file
    let config_result: Result<String, String> = Ok(String::from("config_value"));

    // ? operator returns early if error
    let value = config_result?;

    Ok(format!("Loaded: {}", value))
}

/// Parses age and calculates next year's age
/// Demonstrates chaining with ?
fn calculate_age_next_year(age_str: &str) -> Result<u32, String> {
    let age = parse_age(age_str)?; // ? propagates error
    Ok(age + 1)
}

/// Generic parsing function
fn generic_parse(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let num = s.parse::<i32>()?;
    Ok(num)
}

// COMPARISON: PANIC vs RESULT
//
// PANIC:
//   - Unrecoverable errors
//   - Program terminates
//   - Use in: tests, prototypes, guaranteed safety
//   - Example: panic!("Critical error")
//
// RESULT:
//   - Recoverable errors
//   - Program continues
//   - Use in: I/O, parsing, validation, production
//   - Example: Result<T, E> with Ok(T) or Err(E)
//
// KEY METHODS:
//
// Result methods:
//   is_ok()           - Check if Ok
//   is_err()          - Check if Err
//   ok()              - Convert to Option
//   err()             - Convert to Option<E>
//   map(f)            - Transform Ok value
//   map_err(f)        - Transform Err value
//   and_then(f)       - Chain operations
//   unwrap()          - Get value or panic
//   unwrap_or(default)- Get value or default
//   expect(msg)       - Get value or panic with message
//   ?                 - Propagate error (early return)
//
// BEST PRACTICES:
//   1. Use Result for expected failures
//   2. Use panic! only for programming errors
//   3. Propagate errors with ? operator
//   4. Document what errors functions can return
//   5. Use meaningful error messages
//   6. Consider custom error types for libraries
