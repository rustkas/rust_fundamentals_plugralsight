fn main() {
    let mut original = String::from("original value");
    println!("\nOuter original value: \t\"{}\"", original);
    // original = String::from("new value");

    
    {
        let next = &mut original;
        *next = String::from("next value");
        println!("inner scope next: \t\"{}\"", next);
        println!("inner scope original {original}");
    }
    // original = String::from("new value");
    println!("Outer original value: \t\"{}\"", original);
}