use std::convert::From;


#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        match self {
            Number { value: ref item} => *item
        }
    }
}

fn main() {
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
    // my own
    // definition of Into
    let elt: i32 = Number::into(num);
    println!("Another one... {:?}", elt);
}