//solve it in two ways
//don't let 'println!' work
fn main() {
    never_return();

    println!("failed!");
}

fn never_return() -> ! {
    //implement this function, don't modify the fn signatures
    panic!()
}