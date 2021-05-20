#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // explicitly `use` each name so they are available without
    // manual scoping
    use crate::Status::{ Poor, Rich };
    // automatically `use` each name inside `Work`
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor are going to eat them!"),
    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldiers fight!"),
    }
}