fn main() {
    println!("--- Calling a simple function ---");
    say_hello();

    println!("\n--- Functions with parameters ---");
    print_value(10);
    print_measurement(25, 'm');

    println!("\n--- Block expressions ---");
    let a = {
        let x = 3;
        x + 2 // no semicolon → block is an expression that evaluates to 5
    };
    println!("Value of a (block without semicolon): {}", a);

    let b = {
        let x = 3;
        let _ = x + 2;
    }; // semicolon → this is a statement, block returns ()
    println!("Value of b (block with semicolon): {:?}", b);

    println!("\n--- Function with implicit return ---");
    let five = give_five();
    println!("give_five() returned: {}", five);

    println!("\n--- Function with explicit early return ---");
    let bigger = maybe_double(4);
    println!("maybe_double(4) = {}", bigger);
    let bigger = maybe_double(6);
    println!("maybe_double(6) = {}", bigger);

    // Uncommenting this function will cause a compile error because of semicolon:
    // fn bad_return(x: i32) -> i32 {
    //     x + 1; // <-- semicolon here means unit `()`, not `i32`!
    // }
}

fn say_hello() {
    println!("Hello from say_hello!");
}

fn print_value(x: i32) {
    println!("The value is: {}", x);
}

fn print_measurement(value: i32, unit: char) {
    println!("Measurement: {}{}", value, unit);
}

fn give_five() -> i32 {
    5 // final expression without semicolon is returned
}

fn maybe_double(x: i32) -> i32 {
    if x > 5 {
        return x * 2; // explicit early return
    }
    x // implicit return of this expression
}
