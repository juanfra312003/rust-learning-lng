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

    // Conditions
    println!("\nConditions: ");
    let mut number = 9;
    conditions(number);
    number = 10;
    conditions(number);
    number = 11;
    conditions(number);

    // Conditions with return value
    println!("\nConditions with return: ");
    let number: i32 = 10;
    let number_return: i32 = conditions_odd(number);
    if number_return == 1 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    // Bucles
    println!("\nBucles: ");
    println!("\nBucles with loop");
    loop_bucle();

    println!("\nBucles with while");
    while_bucle();

    println!("\nBucles with for");
    for_while();

    println!("\nPropiety Concept: ");
    propiety_concept();

    println!("\nPropierty: ");
    let string = String::from("Hello, I'm learning Rust");
    let result_string = take_propierty(string);
    println!("String Value: {}", result_string);

    println!("\nPropierty with return: ");
    let string = String::from("Today is a great day");
    let (result_string, length) = propierty_return(string);
    println!("String Value: {}, Length: {}", result_string, length);

    // Slides
    println!("\nSlides: ");
    let string_slides = String::from("Hello friends of Rust");
    let hello = &string_slides[..5];
    let friends = &string_slides[6..];

    println!("Hello: {}", hello);
    println!("Friends: {}", friends);

    // Get the first word of a string using a function (Slides)
    let return_string = slides_first_word(&string_slides);
    println!("First Word: {}", return_string);

    // Slides of collections
    let list_a = [1, 2, 3, 4, 5];
    let list_b = &list_a[1..3];
    println!("List B: {:?}", list_b);
}

fn propierty_return(string: String) -> (String, usize) {
    let length: usize = string.len();
    return (string, length);
}

fn take_propierty(string: String) -> String {
    return string;
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
    return true;
}

fn conditions(number: i32) {
    if number > 10 {
        println!("The number {} is greater than 10", number)
    } else if number < 10 {
        println!("The number {} is less than 10", number)
    } else {
        println!("The number {} is equal to 10", number)
    }
}

fn conditions_odd(number: i32) -> i32 {
    let boolean: bool = number % 2 == 0;
    let result = if boolean { 1 } else { 0 };
    return result;
}

fn loop_bucle() {
    let mut counter = 0;
    let resultado = loop {
        println!("Hello, I'm learning Rust");
        counter += 1;
        if counter == 10 {
            break counter + 7;
        }
    };

    println!("Millos will win their {} league in this year", resultado);
}

fn while_bucle() {
    let mut counter = 10;

    while counter != 0 {
        println!("Counter Value: {}", counter);
        counter -= 1;
    }

    println!("\nWhile with Array: ");
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < array.len() {
        println!("Value: {}, {}", i, array[i]);
        i += 1;
    }
}

fn for_while() {
    let mut array = [10, 20, 30, 40, 50, 60];

    for i in 0..array.len() {
        array[i] = array[i] * 2;
    }

    for element in array.iter() {
        println!("Element Value: {}", element);
    }
}

fn propiety_concept() {
    // Stack and Heap
    let i: i32 = 7;
    let j: i32 = i;

    /* Is not possible to do this
    {
        let i = 8;
        let cadena = String::from("Hello, I'm learning Rust");
    }
     */

    let string1 = String::from("Hello, I'm learning Rust");
    let string2 = string1.clone();
    /* let string2 = string1; In this case string1 will not be pointing to anything*/

    println!("\n Int values: - Size i32");
    println!("i: {}", i);
    println!("j: {}", j);

    println!("\n String values: - Size not defined");
    println!("String2: {}", string2);
    println!("String1: {}", string1);
}

// Slides:
// Function that return the number of bits of the first word of a string
fn first_word(string: &String) -> usize {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    string.len()
}

// Function that return the slide of the first word of a string
fn slides_first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..i];
        }
    }
    &string[..]
}
