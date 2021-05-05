fn main() {
    println!("{part:b} of {whole:b} people know binary. The other half doesn't!", part=1, whole=2);

    // push `number` to the extreme right, in a field of width 5

    println!("{number:>5}", number=1);

    println!("{number:0>5}", number=1);

    println!("{number:>width$}", number=1, width=5);

    println!("{number:0>width$}", number=1, width=5);

    // push `number` to the extreme left, in a field of width 5

    println!("{number:<5}", number=1);

    println!("{number:0<5}", number=1);
}
