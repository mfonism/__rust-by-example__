// automagically deriving the Debug trait for a structure
// also lets it be pretty printed
// with the formatter {:#?}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Leidi Jack";
    let age = 27;
    let heyleidi = Person { name, age };

    println!("{:#?}", heyleidi);

}