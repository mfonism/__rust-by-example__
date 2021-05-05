// automagically derive Debug for a structure
// then implement Display for it
// print with the respective formatters and compare

use std::fmt;

// case of MinMax

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {})", self.0, self.1);
    }
}

// case of Point2D

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "x: {}, y: {}", self.x, self.y);
    }
}

// case of Complex

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} + {}i", self.real, self.imag);
    }
}


fn main() {
    let minmax = MinMax(0, 4);
    let point = Point2D { x: 3.3, y: 4.4 };
    let complex = Complex { real: 3.3, imag: 7.1 };

    println!("--------------");
    println!("Case of MinMax");
    println!("--------------");
    println!("Debug");
    println!("* regular");
    println!("{:?}", minmax);
    println!("* pretty");
    println!("{:#?}", minmax);
    println!("--------------");
    println!("Display");
    println!("{}", minmax);
    println!("--------------");

    println!("");

    println!("---------------");
    println!("Case of Point2D");
    println!("---------------");
    println!("Debug");
    println!("* regular");
    println!("{:?}", point);
    println!("* pretty");
    println!("{:#?}", point);
    println!("---------------");
    println!("Display");
    println!("{}", point);
    println!("---------------");

    println!("");

    println!("---------------");
    println!("Case of Complex");
    println!("---------------");
    println!("Debug");
    println!("* regular");
    println!("{:?}", complex);
    println!("* pretty");
    println!("{:#?}", complex);
    println!("---------------");
    println!("Display");
    println!("{}", complex);
    println!("---------------");
}
