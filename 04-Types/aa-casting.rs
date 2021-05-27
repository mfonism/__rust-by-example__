#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // error! No implicit conversion
    // let integer: u8 = decimal;
    
    // explicit conversion
    let integer = decimal as u8;
    let character = integer as u8;

    println!("decimal is {}", decimal);
    println!("integer is {}", integer);
    println!("character is {}", character);

    // error! invalid cast
    // there are limits in conversion rules
    // a float cannot be directly converted into a character
    // let character = decimal as char;
}