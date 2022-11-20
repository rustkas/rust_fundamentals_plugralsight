fn main() {
    write_message();

    let name = "Duck Airlines";
    
    let write_message = |slogan: String| {
        println!("Hey. This is the closure.");
        println!("Closure. Welcome to {name}. {slogan}");    
    };
    let slogan = String::from("We hit the ground every time.");
    write_message(slogan);
}

fn write_message() {
    let name = "Duck Airlines";
    let slogan = "We hit the ground every time.";
    println!("Welcome to {name}. {slogan}");
}