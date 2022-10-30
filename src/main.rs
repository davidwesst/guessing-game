use std::io;

fn main() {
	println!("I am thinking of a number. I bet you can guess it.");
	println!("Try it! Enter a number: ");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read the input.");

	println!("You guessed: {guess}");
}
