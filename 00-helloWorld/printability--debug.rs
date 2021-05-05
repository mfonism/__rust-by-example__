// we can automagically derive the Debug trait for structs
// so that they can be printed using the `{:?}` formatter
// note that this formatter is different from the default formatter {}
// although they work in the same spirit

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("`Structure(3)` prints as {:?}", Structure(3));
    println!("`Deep(Structure(7))` prints as {:?}", Deep(Structure(7)));
}

