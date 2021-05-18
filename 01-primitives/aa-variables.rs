fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0; // regular annotation of numbers
    let an_integer = 5i32;  // suffix annotation of numbers

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12; // the type is i64, inferred form another time
    inferred_type = 42980938784i64;

    // a mutable variable's value can be changed
    let mut mutable = 12;
    mutable = 32;

    // Error! the type of a variable can't be changed
    mutable = true;

    // variables can be overwritten with shadowing
    let mutable = true;
}