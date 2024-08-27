fn main() {
    check_mutability();
    check_immutability();
    shadowing();
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

const MAX_POINTS: u32 = 100_000; // Constants are always immutable and the value must be a constant expression, not a function call or any other value that could only be computed at runtime.

// Variables in Rust are immutable by default. To make them mutable, you need to use the mut keyword.
fn check_mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This will compile because x is mutable.
    println!("The value of x is: {}", x);
}

// This function will not compile with out commenting because x is immutable.
fn check_immutability() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This will not compile because x is immutable.
    println!("The value of x is: {}", x);
}

// Rust allows you to shadow the previous value of guess with a new one.
fn shadowing() {
    let x = 5;
    {
        // This x is a new variable that shadows the outer x.
        let x = x + 1;
        println!("The value of x in the inner scope is: {}", x);
    }
    // This x is the original x.
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
