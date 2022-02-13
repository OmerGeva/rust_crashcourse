// Loops are used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite Loop
    // loop {
    //     count += 1;
    //     println!("Number {}", count);

    //     if count > 19 {
    //         break;
    //     }
    // }

    // While loops (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizbuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    // For Range
    for i in 0..100 {
        if i % 15 == 0 {
            println!("fizbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}