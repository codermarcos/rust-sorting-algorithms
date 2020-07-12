extern crate rand;

use std::env;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod generate;
mod algorithms;

fn main() {
	let args: Vec<String> = env::args().collect();
	let show = args.contains(&"show".to_string());
	
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

	if show && args.contains(&"initial".to_string()) {
		println!("initial {:?}", array);
	}

	let mut result = algorithms::handle_algorithm(input_algorithm.as_ref(), array);

	result.sort_by(|a, b| b.duration.cmp(&a.duration));

	for r in result {
		println!("  {} levou:\n\t 0,0{}ms", r.algorithm.as_str(), r.duration);
		if show && args.contains(&"result".to_string()) {
			println!("result {:?}", r.vector);
		}
	}
}
