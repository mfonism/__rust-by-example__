fn main() {
    // can we mutate the value of a variable
    let a = 32;
    a = 42;
    println!("{}", a);
}

// Error: cannot assign twice to immutable variable
