use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    another_func();
    func_with_multiple_args(45, 'c');
    let num = 12;
    let result = func_increase_by_a_dozen(num);
    println!("increase {num} by dozen {result}");
    func_after_a_month('M', 5000);
    func_hash_map(1,"95101".to_string() , "San Jose".to_string()); 
    func_hash_map(2,"95101".to_string() , "San Jose".to_string()); 
    func_hash_map(3,"95101".to_string() , "San Jose".to_string()); 
    another_func1();
    another_func_with_params1(45);
    another_fun_with_multiple_params(1, "joe".to_str(), 'c');
}
fn another_func() {
  println!("another function");
}
fn func_with_multiple_args(intvalue : i32, charvalue : char) {
    println!("{}", intvalue + 1);
    println!("next arg is {charvalue}");
}
//function where a block is introduced
fn scope_block_as_expression() {
   let a = {
       let b = 5;
       b + 3
   };
   println!("scope expression returned {a}");
}
fn func_increase_by_a_dozen(n : i32) -> i32 {
    n + 12
}
fn func_after_a_month(first: char, salary: i32) {
	println!("employee starting with {first} has {salary} dollars as salary");
}
fn func_hash_map(counter: u64, key: String, value: String) -> bool {
	let mut my_map = HashMap::new();
	my_map.insert("95391", "Mountain House");
	my_map.insert("95051", "Santa Clara");
	my_map.insert(&key, &value);
	for key in my_map.keys() {
		println!("{} {} ", counter, key);
	}
	if my_map.len() > 2 {
		return true;
	} else {
		return false;
	}	
}
fn another_func1() {
    println!("another_func1: Hello world");
}
fn another_func_with_params1(num: i32) {
    println! ("another_func_with_params1: {}", num);
    println! ("another_func_with_params1: {num}");
}
fn another_fun_with_multiple_params(id: i32, name: String, initial: char) {
    println!(" {id} {name} {initial} ");
}
