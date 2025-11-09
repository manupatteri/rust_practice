fn main() {
    println!("Hello, world!");
    println!("{}", r_first_word(&String::from("asdfg rty")));
    println!("{}", first_word(&String::from("asd")));
    println!("{}", get_first_word(&String::from("asd")));
}
fn get_first_word(my_string: &String) -> usize {
	let string_bytes = my_string.as_bytes();
	for (i, &my_char) in string_bytes.iter().enumerate() {
       		if my_char == b' ' {
        	    return i;
        	}
	}
	my_string.len()
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn r_first_word(full_string: &String) -> usize {
	let word_bytes = full_string.as_bytes();
	for (i, &item) in word_bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	word_bytes.len()
	
}
