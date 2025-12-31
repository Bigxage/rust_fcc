fn main() {
    println!("success!")
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            //todo
        }
        _ => {
            //todo
        }
    };

    //rather than returning a None, we use a diverging function instead
    never_return_fn()
}

//implement this function in three ways
fn never_return_fn() -> ! {
    panic!() //or unimplemented!() or todo!()
}