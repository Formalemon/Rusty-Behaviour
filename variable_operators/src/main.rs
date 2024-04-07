// a function with no return value
fn print_hello() {
    println!("Hello, world! from a functiion.");
}

// Define a function with a return value
fn add(a: i32, b: i32) -> i32 {
    a + b // In Rust, the last expression in a function is implicitly returned
}

fn main() {
    // Data types
    let a: i32 = 5; // integer
    let b: f64 = 3.14; // floating point
    let c: bool = true; // boolean
    let d: char = 'x'; // character
    let e: &str = "hello"; // string slice

    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    // Operators
    println!("a + b: {}", (a as f64) + b); // cast a to f64 to add
    println!("a - b: {}", (a as f64) - b); // cast a to f64 to subtract
    println!("a * b: {}", (a as f64) * b); // cast a to f64 to multiply
    println!("a / b: {}", (a as f64) / b); // cast a to f64 to divide
    println!("int(a) / int(b): {}", a / (b as i32)); // cast b to i32 to divide
    println!("a % 2: {}", a % 2); // modulo

    // Compound data types
    let f: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let g: [i32; 5] = [1, 2, 3, 4, 5]; // array

    println!("f: ({}, {}, {})", f.0, f.1, f.2);
    println!("g: {:?}", g);

    // Accessing tuple elements
    let (x, y, z) = f;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Accessing array elements
    println!("g[0]: {}, g[1]: {}, g[2]: {}", g[0], g[1], g[2]);
    
    // Accessing array elements using for loop
    for val in &g {
        println!("Value using for loop: {}", val);
    }
    
    // Loops
    // for loop
    for i in 0..5 {
        println!("for loop, iteration: {}", i);
    }

    // while loop
    let mut i = 0;
    while i < 5 {
        println!("while loop, iteration: {}", i);
        i += 1;
    }

    // loop
    let mut i = 0;
    loop {
        if i >= 5 {
            break;
        }
        println!("loop, iteration: {}", i);
        i += 1;
    }

    // Nested loop
    for i in 0..3 {
        for j in 0..3 {
            println!("Nested loop using 'for', iteration: ({}, {})", i, j);
        }
    }

    // Nested loop with loop
    // Loops can return a value using the break statement
    // The result of the loop is assigned to the variable result
    let mut i = 0;
    let mut j;
    let result = 'outer: loop {
        j = 0;
        'inner: loop {
            if i == 5 {
                break 'outer 5;
            }
            if j == 3 {
                break 'inner;
            }
            println!("Nested loop using 'loop', iteration: ({}, {})", i, j);
            j += 1;
        }
        i += 1;
    };

    // Function calls
    println!("Result from nested loop: {}", result);
    print_hello();
    let sum = add(a, 5);
    println!("Sum using a function: {}", sum);
}