fn main() {
	println!("Hello, world!");
	let mut s = String::from("Hello");
	let len = calculate_length(&s);
	println!("{} {}", s, len);
	change(&mut s);
	//double_mutable_ref_error();
	mutable_ref_diff_scope();
	combine_mut_and_nonmut_ref_overlapped();
	combine_mut_and_nonmut_ref_separate();
}
//define a function which accepts a variable and not a reference
fn calculate_length(s: &String) -> usize {	
	return s.len()	
}
fn change(myStr: &mut String) {
	myStr.push_str(" world");
}
fn double_mutable_ref_error() {
	let mut my_string = String::from("double ref");
	let r1 = &mut my_string;
	let r2 = &mut my_string;
	//If you uncomment this , it will give an error about double borrow
	//println!("{}, {}", r1, r2);
}
fn mutable_ref_diff_scope() {
	let mut my_string = String::from("double ref");
	{
		let r1 = &mut my_string;
	}
	let r2 = &mut my_string;
}
fn combine_mut_and_nonmut_ref_overlapped() {
	let mut my_string = String::from("double ref");
	let r1 = &my_string;
	let r2 = &my_string;
	let r3 = &mut my_string;
//	if uncommented it will show an error 
	//println!("{} {} {} ", r1, r2, r3);

}
fn combine_mut_and_nonmut_ref_separate() {
	let mut my_string = String::from("double ref");
	let r1 = &my_string;
	let r2 = &my_string;
	println!("{} {}  ", r1, r2);
	let r3 = &mut my_string;
	println!("{} ", r3);

}
