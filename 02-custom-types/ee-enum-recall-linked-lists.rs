#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>),
}

impl List {
    fn empty() -> Self {
        Self::Nil
    }

    fn prepend(self, num: i32) -> Self {
        Self::Cons(num, Box::new(self))
    }

    fn head(&self) -> i32 {
        match *self {
            Self::Nil => panic!("Cannot get the head of an empty list!"),
            Self::Cons(i, _) => i,
        }
    }

    fn length(&self) -> i32 {
        match self {
            Self::Cons(_, ref tail) =>  1 + tail.length(),
            _ => 0,
        }
    }

    fn tail(&self) -> &Self {
        match self {
            Self::Cons(_, tail) => &*tail,
            nil => nil,
        }
    }

    fn sum(&self) -> i32 {
        match self {
            Self::Cons(i, ref tail) => i + tail.sum(),
            _ => 0,
        }
    }
}

fn main() {
    let empty = List::empty();
    println!("An empty list: {:?}", empty);

    let singleton = empty.prepend(0i32);
    println!("A singleton: {:?}", singleton);

    let moar = singleton.prepend(1).prepend(2).prepend(3).prepend(4);
    println!("A longer list: {:?}", moar);

    let head = moar.head();
    println!("Head of longer list: {:?}", head);

    let tail = moar.tail();
    println!("Tail of longer list: {:?}", tail);
    println!("Tail of previous tail: {:?}", tail.tail());

    println!("Length of longer list: {:?}", moar.length());

    println!("Sum of longer list: {:?}", moar.sum());
    println!("Sum of tail of longer list: {:?}", moar.tail().sum());
    println!("Sum of tail of tail of longer list: {:?}", moar.tail().tail().sum());

}