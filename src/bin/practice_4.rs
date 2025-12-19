//modify "assert!" to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("The value of v is: {}", v);
    assert!(v == 1597);

    println!("success!")
}