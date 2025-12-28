//unit type
//make it work, don't modify 'implicitly_ret_unit' 
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v,  implicitly_ret_unit());

    println!("success!");
}

fn implicitly_ret_unit() {
    println!("i will return a ()");
}

//don't use this one
fn explicitly_ret_unit() -> () {
    println!("i will return a ()");
}