fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // this binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("inner shadow showing from inner block: {}", shadowed_binding);
    }

    println!("inner shadow doesn't exist in this scope: {}", shadowed_binding);

    // this binding *shadows* the previous one (which?)
    let shadowed_binding = false;
    println!("shadowed in outer block: {}", shadowed_binding);
}
