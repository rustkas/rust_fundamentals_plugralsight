fn main() {
    let returned_ref = return_bad_ref();
    // println!("Hello, world!");
}

fn return_bad_ref() -> &i32 {
    let value = 5;
    return &value;
}