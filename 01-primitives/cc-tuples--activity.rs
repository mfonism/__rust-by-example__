struct Matrix(f32, f32, f32, f32);

impl std::fmt::Display for Matrix {
    fn fmt(self: &Self,  f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;
    Matrix(a, c, b, d)
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}
