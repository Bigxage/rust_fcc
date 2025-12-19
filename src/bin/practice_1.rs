//modify "assert_eq!" to make it work
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("success!");
}

//get the type of given variable, return a string representation of the type, e.g 
fn type_of<T>(_: &T) -> String {
    format! ("{}", std::any::type_name::<T>())
}