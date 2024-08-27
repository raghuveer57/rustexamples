fn main() {
    let mut value:u8 = 255;
    println!("The value of value is: {}", value);

    // value = value + 1; // This will cause an overflow error. causes panic in debug mode and wraps around in release mode.

    //saturating_add
    value = value.saturating_add(1); // This will saturate at the maximum value.
    println!("The value after saturating add is: {}", value);

    //checked_add
    //value = value.checked_add(1).expect("Overflow error"); // This will cause an overflow error.

    //overflowing_add
    match value.overflowing_add(1){
        (result, true) => println!("The value after overflowing add is: {}. Overflowed", result),
        (result, false) => println!("The value after overflowing add is: {}. Did not overflow", result),
    } 

    // Wrapping
    value = value.wrapping_add(1); // This will wrap around to 0.
    println!("The value after wrapping add is: {}", value);

    println!("The value of value is: {}", value);

}
