fn main() {
    const PI: f32 = 3.141592;

    // hardcoded precision
    println!("PI is equal to {pi:.3}", pi=PI);

    // named argument precision
    println!("PI is equal to {pi:.prec$}", pi=PI, prec=3);

    // positional argument precision
    println!("PI is equal to {0:.1$}", PI, 3);

    // `:.*`
    // precision must always come before the thing to print when using the `:.*` format
    println!("PI is equal to {:.*}", 3, PI);
}