#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>),
}

impl List {
    fn empty() -> Self {
        Self::Nil
    }

    fn cons(self, i: i32) -> Self {
        Self::Cons(i, Box::new(self))
    }

    fn head(&self) -> i32 {
        match *self {
            Self::Nil => panic!("Cannot get the head of an empty list!"),
            Self::Cons(i, _) => i
        }
    }

    fn tail(&self) -> &Self {
        match *self {
            Self::Nil => panic!("Cannot get the tail of an empty list!"),
            Self::Cons(_, ref tail) => tail, 
        }
    }

    fn map<F>(&self, f: F) -> Self
        where F: Fn(i32) -> i32 {
        match *self {
            Self::Nil => Self::Nil,
            Self::Cons(i, ref tail) => Self::Cons(f(i), Box::new(tail.map(f))),
        }
    }
}

fn display_divider() {
    println!("\n*****\n");
}

fn main() {
    let empty = List::empty();
    let singleton = List::empty().cons(0);
    let house = List::empty().cons(0).cons(1).cons(2).cons(3).cons(4);

    display_divider();

    println!("an empty list: {:?}", empty);
    println!("a singleton list: {:?}", singleton);
    println!("a house: {:?}", house);

    display_divider();

    // println!("head of empty list: {}", empty.head());
    println!("head of singleton list: {}", singleton.head());
    println!("head of house, LOL: {}", house.head());

    display_divider();

    // println!("tail of empty list: {:?}", empty.tail());
    println!("tail of singleton list: {:?}", singleton.tail());
    println!("tail of house (rat): {:?}", house.tail());

    display_divider();

    println!("double every element in the empty list: {:?}", empty.map(|x| x * 2));
    println!("double every element in the singleton list: {:?}", singleton.map(|x| x * 2));
    println!("double every element in the house: {:?}", house.map(|x| x * 2));
}