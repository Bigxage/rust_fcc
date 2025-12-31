fn main() {
    //fill in the blank
    let b = false;

    let _v = match b{
        true => 1,
        //diverging functions can also be used in match expression to replace a value
        false => {
            println!("success!");
            panic!("we have no value for 'false', but we can panic");
        }
    };

    println!("exercise failed if printing out this line!")
}