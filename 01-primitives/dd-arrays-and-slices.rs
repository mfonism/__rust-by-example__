use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("the first element of this slice is {}", slice [0]);
    println!("this slice has {} elements", slice.len());
}

fn main() {
    // an array of length five, containing 1..5
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // an array of length 500 containing 0's
    let ys: [i32; 500] = [0; 500];

    println!("the first element of xs is {}", xs[0]);
    println!("the second element of xs is {}", xs[1]);
    println!("there are {} elements in xs", xs.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slices can point to a section of an array
    // they are of the form [starting_index .. ending_index]
    // note that ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice!");
    analyze_slice(&ys[1 .. 4]);
}