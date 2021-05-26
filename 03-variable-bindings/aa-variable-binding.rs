fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // compiler will warn about this unused variable binding
    let noisy_unused_variable = 3u32;

    // compiler won't warn about this unused variable binding
    // because the variable is prefixed with an underscore
    let _silent_unused_variable = 3u32;
}