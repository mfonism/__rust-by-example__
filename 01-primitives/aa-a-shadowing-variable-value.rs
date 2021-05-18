fn main() {
    // can we shadow the value of a variable
    let a = 32;
    let a = 42;
    println!("{}", a);
}

// Warning: unused variable: `a`
// Compiles
