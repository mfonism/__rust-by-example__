#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let plus = Operations::Add;
    let minus = Operations::Subtract;
    println!("{:?}", plus);
    println!("{:?}", minus);
}