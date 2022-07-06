// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];  // [data type; length]

    println!("{:?}", numbers);

    // Get single aval
    println!("Single value : {}", numbers[0]);

    // re-assign a value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get array length
    println!("Array Length {}", numbers.len());

    // Arrays are stack allocated
    // println!("This array occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice 
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);



}