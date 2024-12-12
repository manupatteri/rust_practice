fn main() {
    println!("Hello, world! {} ", get_value_from_tuple());
}
fn get_value_from_tuple() -> isize {
	let new_tup = (4, "manoj", 5.5);
	return new_tup.0;
}
