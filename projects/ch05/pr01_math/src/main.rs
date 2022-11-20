#![allow(unused)]
fn main() {
    let squared = i32::pow(8, 2);

    let float_integer = f32::powi(6.5,3);
    let float_float = f32::powf(6.5,3.14);
    
    println!("{float_integer}");
    println!("{float_float}");
    

    let order_ops = 8+4*2-(12/3+7)+4;
    println!("{order_ops}");

    let are_equal_is_true = 1 == 1;
    let are_equal_is_false = 1 == 2;
    println!("This result is true - {are_equal_is_true}");
    println!("This result is false - {are_equal_is_false}");

    let are_not_equal = 1 != 2;
    let is_true = true;
    let is_false = !is_true;


    let have_boarding_pass = true;
    let have_id = true;

    let can_board = have_boarding_pass && have_id;

    let have_id = !true;
    let can_board = have_boarding_pass && have_id;
    
    let have_driver_licence = false;
    let have_password = true;
    let have_proof = have_driver_licence || have_password;

}
