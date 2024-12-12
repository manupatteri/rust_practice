fn main() {
	let _my_vec:Vec<u32> = Vec::new();
	let identified_vec = vec![3,4,5];
	for referenced_value in &identified_vec {
		println!("{}", referenced_value);
	}
	let mut editable_vec = Vec::new();
	editable_vec.push(1.1);
	editable_vec.push(2.1);
	editable_vec.push(3.1);
	for referenced_value in &editable_vec {
		println!("{}", referenced_value);
	}
    	println!("Hello, world!");
	//let access_vec = vec![i32, 5, 10];
	let access_vec = vec![1, 2, 3];
	println!("Initial vector: {:?}", access_vec);
	let second = access_vec[1];
	let third = access_vec.get(2);
	match third {
        	Some(third) => println!("The third element is {third}"),
        	None => println!("There is no third element."),
    	}
}
