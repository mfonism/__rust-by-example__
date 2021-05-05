use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?;
        }
        return write!(f, "]");
    }
}

fn main() {
    let v = List(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{:?}", v);
    println!("{:#?}", v);
    println!("{}", v);
}