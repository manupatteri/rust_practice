fn main() {
    println!("Hello, world!");
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
