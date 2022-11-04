use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	let answer = rand::thread_rng().gen_range(1..=100);
	println!("I am thinking of a number. I bet you can guess it.");
	println!("(DEBUG) Answer = {answer}");
	println!("Try it! Enter a number: ");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read the input.");

	let guess : u32 = guess.trim().parse().expect("You need to guess a number silly!");
	
match guess.cmp(&answer) {
		Ordering::Less => println!("Too low!"),
		Ordering::Greater => println!("Too high!"),
		Ordering::Equal => println!("You got it!")
	}

	println!("You guessed: {guess}");
}
