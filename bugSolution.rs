fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    // Correct approach 1:  Use separate variables
    *y = 10;
    let z = x; // Now we can create an immutable reference (or copy) without conflict
    println!("x: {}, z: {}", x, z);

    // Correct approach 2:  Clone the data
    let mut x2 = 5;
    let y2 = &mut x2;
    let z2 = x2; // Create a copy of the data
    *y2 = 10;
    println!("x2: {}, z2: {}", x2, z2);
} 