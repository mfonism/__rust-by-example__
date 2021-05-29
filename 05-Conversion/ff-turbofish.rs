use std::convert::Into;

#[derive(Debug)]
struct Single (isize);

#[derive(Debug)]
struct Double (isize, isize);

#[derive(Debug)]
struct Triple (isize, isize, isize);

impl Into<Single> for isize {
    fn into(self) -> Single {
        Single (self)
    }
}

impl Into<Double> for isize {
    fn into(self) -> Double {
        Double (self, self)
    }
}

impl Into<Triple> for isize {
    fn into(self) -> Triple {
        Triple (self, self, self)
    }
}

fn main() {
    // doesn't work as I'd thought
    // raises error
    // error[E0107]: this associated function takes 0 type arguments but 1 type argument was supplied
    let single = 5.into::<Single>();

    println!("single is {}", single);
}
