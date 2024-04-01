fn main() {
    // Learning the fundamentals of Rust
    // Print by console, use of mutable and immutable vars and const.
    println!("\nFundamentals of Rust:");
    fundamentals();

    println!("\nData Types of Rust:");
    data_types();
}

fn fundamentals() {
    println!("My first Rust program!");
    let mut x = 10;
    const GRAVITY: f32 = 9.8;

    println!("x Value: {}", x);

    x = 31;

    println!("x value: {}", x);

    let x = "Hi there, my name is Juan Francisco";
    println!("x Value: {}", x);

    println!("Gravity Value: {}", GRAVITY);
}

fn data_types() {
    // DATA TYPES SIMPLES
    println!("Integer management: ");

    let mut integer: i32 = 1654;
    println!("integer Value: {}", integer);

    integer = 525;
    println!("integer Value: {}", integer);

    let divisor: i32 = 2;

    // Float data, if we want to divide two integers we need to cast them to float
    let division: f32 = integer as f32 / divisor as f32;
    println!("Division Value: {}", division);

    // Boolean
    let today_is_saturday: bool = false;
    println!("Today is Saturday: {}", today_is_saturday);

    // Character
    let character: char = 'A';
    println!("Character Value: {}", character);

    // DATA TYPES COMPOUNDS
    // 1. TUPLAS
    let mut tuple: (i32, i32, f64) = (1, 2, 3.0);

    // Assign the values to a set of vars
    let (mut x, y, z) = tuple;

    println!("Tuple Value: ({}, {}, {})", x, y, z);

    // For access to elements of the tuple, use the dot .
    println!("Tuple Value: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // Change the value of the tuple
    tuple.0 = 10;
    x = tuple.0;
    println!("Tuple Value: ({}, {}, {})", x, y, z);

    // Arrays
    println!("\nArrays: ");
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    using_functions(array);
}

fn using_functions(array: [i32; 5]) {
    // Arrays calling a function with a return value and the array as parameter

    for i in 0..5 {
        println!("Array Value: {}", array[i]);
        let cousin: bool = cousin_number(array[i]);
        if cousin {
            println!("The number {} is cousin", array[i]);
        } else {
            println!("The number {} is not cousin", array[i]);
        }
    }

    // Give a value to a var due to a function
    let age = 18;
    let j = {
        let current_age = age;
        current_age + 1
    };

    println!("My current age is: {}", j);
}

fn cousin_number(number: i32) -> bool {
    if number <= 1 {
        return false;
    }

    for i in 2..(number / 2 + 1) {
        if number % i == 0 {
            return false;
        }
    }
    true
}
