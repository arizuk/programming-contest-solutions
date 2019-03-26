use std::io;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess");

  let secret_number = 10;
  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
      .expect("Failed to readline");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please input a number!");
        continue;
      },
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}