fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;    // z is an immutable reference to x

    // This will cause a compile-time error because
    // y is a mutable reference and z is an immutable reference
    // to the same variable.
    *y = 10;   
}