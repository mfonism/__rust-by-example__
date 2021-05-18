fn main() {
    // can we shadow the value of a variable
    let a = 32;
    println!("{}", a);
    let a = 42;
    println!("{}", a);
}

// unused variable: `a`
