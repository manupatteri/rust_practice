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
	compare_match_with_if_let();	
}
fn compare_match_with_if_let() -> i32 {
	let config_max = Some(6);
	match config_max {
            Some(max) => println! ("match: Max is {max} "),
            _ => (),
        }
        if let Some(max) = config_max {
            println! ("if let: Max is {max} ");
        }
	return 4;
}
