fn main() {
    // Learning the fundamentos of Rust
    // Print by console, use of mutable and unmutable vars and const.
    fundaments();


}

fn fundaments() {
    println!("My first Rust program!");
    let mut x = 10;

    println!("X Value: {}", x);

    x = 31;

    println!("X value: {}", x);

    let x = "Hi there, my name is Juan Francisco";
    println!("X Value: {}", x);
}
