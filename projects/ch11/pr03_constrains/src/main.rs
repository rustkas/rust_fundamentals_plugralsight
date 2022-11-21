#![allow(unused)]

use std::ops::Add;

fn main() {
    let sum1 = add(256, 262);
    let sum2 = add2(256, 262);
    
    println!("{}", sum1);
    println!("{}", sum2);
}

fn add<T: Add<Output = T>>(operand1: T, operand2: T) -> T {
    operand1 + operand2
}

fn add2<T>(operand1: T, operand2: T) -> T
where T: Add<Output = T> {
    operand1 + operand2
}