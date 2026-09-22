fn main() {
    // Scalar Types
    // Integer
    let int_val: i32 = 42;
    println!("Integer: {}", int_val);

    // Floating point
    let float_val: f64 = 3.1415;
    println!("Float: {}", float_val);

    // Boolean
    let bool_val: bool = true;
    println!("Boolean: {}", bool_val);

    // Character
    let char_val: char = 'R';
    println!("Char: {}", char_val);

    // Compound Types
    // Tuple
    let tuple_val: (i32, f64, char) = (500, 6.4, 'z');
    println!("Tuple: ({}, {}, {})", tuple_val.0, tuple_val.1, tuple_val.2);

    // Array
    let array_val: [i32; 3] = [1, 2, 3];
    println!(
        "Array: [{}, {}, {}]",
        array_val[0], array_val[1], array_val[2]
    );

    // String slice (&str)
    let str_val: &str = "Hello, Rust!";
    println!("String slice: {}", str_val);

    // String (heap-allocated)
    let string_val: String = String::from("Owned String");
    println!("String: {}", string_val);

    // Option type (enum)
    let some_val: Option<i32> = Some(10);
    let none_val: Option<i32> = None;
    println!("Option Some: {:?}, Option None: {:?}", some_val, none_val);

    // Result type (enum)
    let ok_val: Result<i32, &str> = Ok(200);
    let err_val: Result<i32, &str> = Err("Error occurred");
    println!("Result Ok: {:?}, Result Err: {:?}", ok_val, err_val);

    // Example: Panic in debug mode (panic! macro)
    // This will cause the program to panic and print a message with backtrace in debug mode.
    // In release mode, the message is printed but backtrace is usually omitted or minimized.
    // Uncomment the next line to see panic behavior:
    // panic!("This is a panic example!");

    // Example: Integer overflow
    // In debug mode, this will panic. In release mode, it wraps around (no panic).
    // Uncomment the next lines to see the difference:
    // let x: u8 = 255;
    // let y = x + 1; // Panics in debug, wraps to 0 in release
    // println!("Overflow result: {}", y);

    // Example: Wrapping add (always wraps on overflow, no panic in any mode)
    let x: u8 = 255;
    let y = x.wrapping_add(1); // This will wrap to 0
    println!("Wrapping add result: {}", y); // Output: 0

    // Example: Overflowing add (returns tuple: (result, did_overflow))
    let x: u8 = 255;
    let (result, did_overflow) = x.overflowing_add(1);
    println!(
        "Overflowing add result: {}, overflowed: {}",
        result, did_overflow
    ); // Output: 0, true

    // Numeric operations examples
    let a: i32 = 10;
    let b: i32 = 3;
    println!("Addition: {} + {} = {}", a, b, a + b);
    println!("Subtraction: {} - {} = {}", a, b, a - b);
    println!("Multiplication: {} * {} = {}", a, b, a * b);
    println!("Division: {} / {} = {}", a, b, a / b);
    println!("Remainder: {} % {} = {}", a, b, a % b);

    // Floating point division
    let c: f64 = 10.0;
    let d: f64 = 3.0;
    println!("Floating division: {} / {} = {}", c, d, c / d);
}
