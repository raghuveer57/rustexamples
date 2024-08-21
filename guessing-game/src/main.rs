use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess.");

        // Variables in Rust are immutable by default. To make them mutable, you need to use the mut keyword.
        let mut guess = String::new();

        // The read_line method takes the user input and appends it to the string we pass it.
        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows you to shadow the previous value of guess with a new one.
        // This feature is often used in situations in which you want to convert a value from one type to another type.
        // The trim method on a String instance will eliminate any whitespace at the beginning and end.
        // The parse method on strings parses a string into some kind of number.
        // We need to tell Rust what type of number we expect, so we add : u32 after parse.
        // The : u32 annotation is a type annotation that tells Rust what type of number we want.
        // Rust can infer this type from the context, so we could have written let guess: u32 = guess.trim().parse(); instead.
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // The cmp method compares two values and can be called on anything that can be compared.
        // It takes a reference to whatever you want to compare with.
        // It returns a variant of the Ordering enum, which is either Less, Greater, or Equal.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {guess}");
    }
    println!("The secret number is: {}", secret_number);
}
