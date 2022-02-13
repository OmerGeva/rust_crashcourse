// Vectors - Resizable Arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    // Reassign
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector of length {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    // Loop through values
    for i in numbers.iter() {
        println!("Number: {}", i)
    }

    println!("Numbers vec: {:?}", numbers);

    // Loop and mutate values
    for i in numbers.iter_mut() {
        *i *= 2;
    }

    println!("Numbers vec * 2: {:?}", numbers);
}