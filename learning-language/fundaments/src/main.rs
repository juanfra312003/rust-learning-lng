fn main() {
    // Learning the fundamentos of Rust
    // Print by console, use of mutable and unmutable vars and const.
    println!("\nFundaments of Rust:");
    fundaments();

    println!("\nData Types of Rust:");
    data_types();
}

fn fundaments() {
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

    let mut integer: i128 = 1654;
    println!("integer Value: {}", integer);

    integer = 525;
    println!("a Value: {}", integer);

    let divisor: i32 = 2;

    // Float data, if we want to divide two integers we need to cast them to float
    let division: f32 = integer as f32 / divisor as f32;
    println!("Division Value: {}", division);

    // Boolean
    let today_is_saturday: bool = false;
    println!("Today is Saturday: {}", today_is_saturday);

    // Character
    let caracter: char = 'A';
    println!("Character Value: {}", caracter);

    // DATA TYPES COMPOUNDS
    // 1. TUPLAS
    let mut tupla: (i32, i32, f64) = (1, 2, 3.0);

    // Asign the values to a set of vars
    let (mut x, y, z) = tupla;

    println!("Tupla Value: ({}, {}, {})", x, y, z);

    // For access to elements of the tupla, use the dot .
    println!("Tupla Value: ({}, {}, {})", tupla.0, tupla.1, tupla.2);

    // Change the value of the tupla
    tupla.0 = 10;
    x = tupla.0;
    println!("Tupla Value: ({}, {}, {})", x, y, z);

    // Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array Value: {:?}", array);
}
