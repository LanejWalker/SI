

// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}


// Fill the blank
fn test() {
    let v: u16 = 38_u8 as __;

    println!("Success!");
}


// Fill the blanks to make it work
fn practice() {
    assert_eq!(i8::MAX, __); 
    assert_eq!(u8::MAX, __); 

    println!("Success!");
}


/// Return `12` if `n` is even,
/// `13` if `n` is divisible by `3`,
/// `17` otherwise.
fn magic_number(n: u32) -> u32 {
    todo!()
}
