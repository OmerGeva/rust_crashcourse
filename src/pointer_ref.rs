// Reference Pointers -> point to a resource in memory

pub fn run(){
    // Primitive array 
    let array1 = [1, 2, 3];
    let array2 = array1;
    println!{"array1: {:?}, array2: {:?}", array1, array2};
    
    /* with non-primitive types, if you assign another variable to a piece of data the first variable will no
    longer hold that value, you'll need to use a reference pointer to the resource
    */

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!{"vec1: {:?}", (&vec1, vec2)};
}