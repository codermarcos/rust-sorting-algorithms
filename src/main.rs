extern crate rand;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
mod generate;
fn main() {
	let mut input_algorithm = String::new();
	let mut input_length = String::new();
	let mut output = stdout();

	println!("Algorithm:");
	println!(" 0. all");
	println!(" 1. quick");
	println!(" 2. bubble");
	println!(" 3. insertion");
	println!(" 4. selection");

	print!("Selecione um algoritimo: ");
	output.flush().expect("could not flush");

	stdin()
		.read_line(&mut input_algorithm)
		.expect("tipo invalido");

	input_algorithm = str::replace(&input_algorithm, "\n", "");

	print!("Selecione quantos items: ");
	output.flush().expect("could not flush");

	stdin().read_line(&mut input_length).expect("tipo invalido");

	input_length = str::replace(&input_length, "\n", "");

	let array = generate::vec(input_length.parse::<usize>().unwrap());

}
