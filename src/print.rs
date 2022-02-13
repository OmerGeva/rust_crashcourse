pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // named arguments
    println!("{name} gefällt {activity} machen", name = "Julius", activity = "sport");

    //  placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}