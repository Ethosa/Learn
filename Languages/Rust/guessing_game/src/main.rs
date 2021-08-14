// Author: Ethosa
mod guess;


fn main() {
    let mut myguess = guess::build_guess();

    loop {
        println!("Try to guess my number >.<");

        let mut readed: String = String::new();

        std::io::stdin().read_line(&mut readed).expect("Failed read line");

        let answer: i32 = readed.trim().parse().expect("Enter a number!");

        println!("{}", if myguess.check(answer) {"Great! You won ^^"} else {"You lose ..."});

        myguess.regen();
    }
}
