#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn apply(&self, x: i32, y:i32) -> i32 {
        match self {
            // inside the impl
            // `Self` is a generic type alias for the subject type
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    println!("{:?}", Operations::Add);
    println!("`Operations::Add.apply(5, 4)` gives: {}", Operations::Add.apply(5, 4));
    println!("{:?}", Operations::Subtract);
    println!("`Operations::Subtract.apply(5, 4)` gives: {}", Operations::Subtract.apply(5, 4));
}