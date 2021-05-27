fn main() {
    // annotated, so has the annotated type
    let elem = 5u8;

    // an vector is a growable array
    let mut vec = Vec::new();
    // at this point the compiler doesn't know what the exact type of `vec` is
    // it just knows that it's a vector of something (`Vec<_>`)

    // insert `elem` into the vector
    vec.push(elem);
    // and now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    println!("{:?}", vec);
}