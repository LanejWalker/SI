

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

// Fix error without adding new line
fn stringy() {
    let s: str = "hello, world";
}

// Fill the blanks to make it work
fn practice() {
    assert_eq!(i8::MAX, __); 
    assert_eq!(u8::MAX, __); 

    println!("Success!");
}


fn tuplle() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}
/// Return `12` if `n` is even,
fn magic_number(n: u32) -> u32 {
    todo!()
}
