#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
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
        value_in_cents(Coin::Quarter(UsState::Alaska));
        coin_with_match_and_if_let(Coin::Quarter(UsState::Alabama));
        coin_with_match_and_if_let(Coin::Penny);
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
fn coin_with_match_and_if_let(coin: Coin) {
    if let Coin::Quarter(state) = coin { 
        println !("State Quarter from {state:?} ");
    } else {
        println !("Not a quarter ");
    }
}
