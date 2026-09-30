fn main() {
    println!("=== Rust Control Flow Examples ===\n");

    // CONDITIONAL STATEMENTS (if/else)
    // Used to execute code blocks based on conditions
    let number = 6;

    // Simple if statement - executes if condition is true
    if number < 5 {
        println!("number is less than 5");
    } else if number == 5 {
        println!("number is exactly 5");
    } else {
        // This block executes since number (6) > 5
        println!("number is greater than 5: {}", number);
    }

    println!("\n--- if as Expression ---");
    // In Rust, if is an expression and returns a value
    // This allows assigning the result to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Value from if expression: {}", number);

    println!("\n--- LOOPS ---");

    // LOOP keyword: infinite loop (must break manually)
    // Used when you need to repeat code indefinitely
    let mut count = 0;
    loop {
        count += 1;
        println!("Loop iteration {}", count);

        // break: exits the loop immediately
        if count == 3 {
            break;
        }
    }

    println!("\n--- WHILE Loop ---");
    // While loop: repeats while condition is true
    // Checks condition BEFORE each iteration
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    println!("\n--- FOR Loop with Range ---");
    // For loop: iterates over a collection/range
    // Most efficient and safe way to loop
    // (0..3) creates a range: 0, 1, 2 (excludes 3)
    for i in (0..3).rev() {
        println!("Countdown: {}", i);
    }
    println!("Blast off!\n");

    // FOR loop with explicit range
    for number in 1..=3 {
        println!("Number: {}", number);
    }

    println!("\n--- FOR Loop with Array ---");
    // For loop over array elements
    let array = [10, 20, 30, 40, 50];
    for value in array {
        println!("Value from array: {}", value);
    }

    println!("\n--- Loop with Return Values ---");
    // Loop can return a value using break
    // This is useful for breaking out with a final value
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2; // break with a value
        }
    };
    println!("Result from loop with break value: {}", result);

    println!("\n--- Loop Labels ---");
    // Loop labels allow you to break or continue a specific loop
    // when you have nested loops. Labels start with a single quote (')
    let mut x = 0;

    // Outer loop label
    'outer: loop {
        x += 1;
        println!("Outer loop, x = {}", x);

        // Inner loop
        for y in 0..3 {
            println!("  Inner loop, y = {}", y);

            // break with label: breaks the outer loop, not just inner
            if x == 2 && y == 1 {
                println!("  Breaking outer loop!");
                break 'outer;
            }
        }
    }
    println!("Exited both loops via outer label\n");

    // Example with continue and labels
    let mut a = 0;
    'counting: loop {
        a += 1;

        if a == 3 {
            println!("Skipping iteration a = {}", a);
            continue 'counting; // continue with the labeled loop
        }

        println!("Current a = {}", a);

        if a == 5 {
            break 'counting; // break the labeled loop
        }
    }
}
