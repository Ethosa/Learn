fn main() {
    let hw = String::from("Hello, world!");

    println!("{}", hw);

    {
        let mut counter = 0;

        loop {
            if counter == hw.len() {
                print!("\n");
                break;
            }

            print!("{}", hw.chars().nth(counter).unwrap());
            counter += 1;
         } 
    }
}
