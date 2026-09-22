#![allow(clippy::approx_constant)]

// ============ GENERIC TYPES ============

// Generic function: works with any type T
fn print_it<T: std::fmt::Display>(val: T) {
    println!("Value: {}", val);
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic struct with multiple types
struct Pair<T, U> {
    first: T,
    second: U,
}

// Generic with constraints (bounds)
#[allow(dead_code)]
struct Container<T: std::fmt::Display> {
    value: T,
}

// Generic impl block
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn get_x(&self) -> &T {
        &self.x
    }
}

// Impl only for specific types
impl Point<i32> {
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

// ============ TRAITS ============

// Define a trait
trait Drawable {
    fn draw(&self);
}

trait Resizable {
    fn resize(&mut self, width: f64, height: f64);
    fn area(&self) -> f64;
}

// Struct implementing a trait
struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle: {}x{}", self.width, self.height);
    }
}

impl Resizable for Rectangle {
    fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Another struct implementing the same traits
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius: {}", self.radius);
    }
}

impl Resizable for Circle {
    fn resize(&mut self, width: f64, _height: f64) {
        self.radius = width / 2.0;
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Trait with default implementation
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    #[allow(dead_code)]
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Trait bounds in generic functions
fn print_resizable<T: Resizable>(obj: &T) {
    println!("Area: {}", obj.area());
}

// Multiple trait bounds
fn draw_and_resize<T: Drawable + Resizable>(obj: &mut T) {
    obj.draw();
    obj.resize(10.0, 10.0);
}

// ============ LIFETIMES ============

// Function with lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct with lifetime annotation
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

// Lifetime in impl block
impl<'a> Book<'a> {
    fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }

    fn get_info(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Multiple lifetimes
struct BlogPost<'a, 'b> {
    title: &'a str,
    content: &'b str,
}

// Lifetime with trait bounds
#[allow(dead_code)]
fn print_summary<'a, T: Summary + 'a>(item: &'a T) {
    println!("{}", item.summarize());
}

// ============ COMBINING GENERICS, TRAITS, AND LIFETIMES ============

// Generic function with trait bounds and lifetimes
#[allow(dead_code)]
fn compare_lengths<'a, T: std::fmt::Display>(x: &'a T, _y: &'a T) -> &'a T {
    // This is just an example; actual comparison would depend on the type
    x
}

// Generic struct with lifetime and trait bound
struct Cache<'a, T: std::fmt::Display> {
    data: &'a T,
}

impl<'a, T: std::fmt::Display> Cache<'a, T> {
    fn display(&self) {
        println!("Cached: {}", self.data);
    }
}

fn main() {
    println!("========== GENERIC TYPES ==========\n");

    // Generic function example
    print_it(42);
    print_it("Hello");
    print_it(3.14);

    // Generic struct example
    let int_point = Point::new(5, 10);
    println!("Int Point X: {}", int_point.get_x());

    let float_point = Point::new(1.5, 2.5);
    println!("Float Point X: {}", float_point.get_x());

    // Generic with specific impl
    println!("Distance from origin: {}", int_point.distance_from_origin());

    // Pair with different types
    let pair = Pair {
        first: 5,
        second: "hello",
    };
    println!("Pair: {} and {}\n", pair.first, pair.second);

    println!("========== TRAITS ==========\n");

    // Using traits
    let mut rect = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    rect.draw();
    println!("Rectangle area: {}", rect.area());

    let circle = Circle { radius: 5.0 };
    circle.draw();
    println!("Circle area: {}", circle.area());

    // Trait with bounds in function
    print_resizable(&rect);

    // Default trait implementation
    let article = NewsArticle {
        headline: String::from("Rust 1.70 Released"),
        location: String::from("Online"),
        author: String::from("Rust Team"),
        content: String::from("New features..."),
    };
    println!("Summary: {}", article.summarize());

    // Multiple trait bounds
    draw_and_resize(&mut rect);
    println!("New rectangle area: {}\n", rect.area());

    println!("========== LIFETIMES ==========\n");

    // Lifetime example
    let str1 = String::from("long string");
    let str2 = "short";
    let result = longest(&str1, str2);
    println!("Longest string: {}", result);

    // Struct with lifetime
    let book = Book::new("The Rust Book", "Steve Klabnik and Carol Nichols");
    println!("Book info: {}", book.get_info());

    // Multiple lifetimes
    let title = String::from("My Blog Post");
    let content = String::from("This is interesting...");
    let post = BlogPost {
        title: &title,
        content: &content,
    };
    println!("Post title: {}, Content: {}\n", post.title, post.content);

    println!("========== COMBINED EXAMPLE ==========\n");

    // Cache with generics, lifetimes, and traits
    let value = 42;
    let cache = Cache { data: &value };
    cache.display();
}
