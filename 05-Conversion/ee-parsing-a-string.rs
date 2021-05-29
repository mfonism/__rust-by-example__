fn main() {
    // target type is not specified, and will be infered according to usage
    let parsed: i32 = "5".parse().unwrap();
    
    // target type is specified by using the 'turbofish' syntax
    let turbo_passed = "10".parse::<i32>().unwrap();

    println!("{}", parsed);
    let sum =  turbo_passed;
    println!("Sum: {:?}", sum);
}