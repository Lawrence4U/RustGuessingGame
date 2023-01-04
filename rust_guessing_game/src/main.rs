use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process;


fn main() {
    println!("Guess the number!");
    println!("------------------");
    println!("Please select the difficulty");
    println!("\t1. Easy (10 tries)");
    println!("\t2. Medium (7 tries)");
    println!("\t3. Hard (5 tries)");


    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty).expect("Failed to read line");
    let difficulty: u32 = match difficulty.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please select a number");
            process::exit(1);
        }
    };

    let max_tries :u32 = match difficulty {
        1 => 10,
        2 => 7,
        3 => 5, 
        0_u32 | 4_u32..=u32::MAX => process::exit(1),
    };

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");
    let mut tries :u32 = 0;
    loop {
        println!("Please input the guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        tries +=1;

        if tries >= max_tries{
            println!("Sorry, you failed 😂, the correct answer was {secret_number}");
            break;
        }
    }

}
