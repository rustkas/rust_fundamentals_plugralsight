fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            continue;
        }
        println!("{counter}");
        if counter != 5 {
            print!("{counter}");
        }
        if counter == 10 {
            break;
        }
    }

    let mut counter = 1;
    while counter <= 10 {
        println!("{counter}");
        counter += 1;
    }
}
