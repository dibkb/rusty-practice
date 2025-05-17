use std::io;
use rand::random_range;

fn check_number(guess: &i32, secret: &i32) -> bool {
    if guess < secret {
        println!("Too low");
        false
    } else if guess > secret {
        println!("Too high");
        false
    } else {
        println!("Congratulations! You have guessed correctly!");
        true
    }
}
fn main() {


    loop{

        let number: i32 = random_range(1..=10);
        let mut guess: String = String::new();
    
        println!("Playing the guessing game");
    
        io::stdin().read_line(&mut guess).expect("Some error occured");
    
        let guess:i32 = guess.trim().parse().expect("Please enter a number between 1 and 100");


        println!("Your guessed number is {}",guess);
    
        println!("The secret number was {}", number);

        if check_number(&guess,&number){
            break;
        }
    }
}
