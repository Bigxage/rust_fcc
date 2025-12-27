//make it work
use std::mem::size_of_val;
fn main() {
    let c1: char = 'a'; // 4 bytes
    assert_eq!(size_of_val(&c1),4);

    println!("{}", size_of_val(&c1));

    let c2: char = ' ';
    assert_eq!(size_of_val(&c2),3);

    println!("success!");
}