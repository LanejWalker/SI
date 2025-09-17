

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
    assert_eq!(i8::MIN, __); 
    assert_eq!(u8::MIN, __); 

    println!("Success!");
}

/// Return `12` if `n` is even,
/// hint num % 2 == 0
