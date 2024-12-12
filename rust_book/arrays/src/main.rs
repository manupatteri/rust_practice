fn main() {
	println!("Hello, world!");
	let simple_array_with_full_init = [1, 3, 5, 7, 9];
	println!("{}", simple_array_with_full_init[3]);
	let array_init_to_same_value = [1; 15];
	for index in 0..array_init_to_same_value.len() {
		println!("{}", array_init_to_same_value[index]);
	}
	let ref_to_an_element: &i32 = &simple_array_with_full_init[2];
        println!("{}", ref_to_an_element);
	
}
