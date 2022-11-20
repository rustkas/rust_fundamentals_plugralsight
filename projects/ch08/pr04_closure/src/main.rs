fn main() {
    write_message();

    let name = "Duck Airlines";
    
    let write_message = |slogan: String| {
        String::from(format!("{}. {}", name, slogan))
    };
    let slogan = String::from("We hit the ground every time.");
    let phrase = write_message(slogan);
    println!("{phrase}");
}

fn write_message() {
    let name = "Duck Airlines";
    let slogan = "We hit the ground every time.";
    println!("Welcome to {name}. {slogan}");
}