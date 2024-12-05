use std::io;

use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("guess!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("right number is {}", secret_number);

    loop {
        println!("guess one number!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("cant read raw");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };

        print!("guess what? {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" Too small!"),
           
            Ordering::Greater => println!(" Toobig!"),

            Ordering::Equal => {
                println!(" You win!");
                break;
            }
        }
    }
}