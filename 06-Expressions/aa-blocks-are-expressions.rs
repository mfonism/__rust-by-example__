fn main() {
    let a = 5usize;
    let b = 4usize;

    let difference_of_squares = {
        let a_squared = a * a;
        let b_squared = b * b;
        a_squared - b_squared
    };

    println!("The difference of squares between {:?} and {:?} is {:?}", a, b, difference_of_squares);
}
