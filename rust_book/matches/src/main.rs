fn main() {
	let sides = 4;
	match sides {
		3 => println!("Triangle"),
		4 => println!("Rectangle"),
		5 => println!("Pentagon"),
		_=>println!("Everything else"),
	}
	let selection = true ;
	let my_choice = match selection {
		true => "I agree",
		false => "I disagree",
	};
	println!("{}", my_choice);
}
