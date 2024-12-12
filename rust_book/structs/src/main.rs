struct MyUnitLikeStruct;
struct TupleStructArea(i32, i32);
struct TupleStructCoordinate(i32, i32);
struct User {
	active : bool,
	username : String,
	email : String,
	sign_in_count : u64,
}
fn build_user(new_name: String, new_email: String) -> User {
	User {
		active: true,
		username: new_name,
		email: new_email,
		sign_in_count: 2,
	}
}
//Requires named lifetime parameter
//struct NewUser {
//	active : bool,
//	city : &str,
//}
//fn reference_field() {
//	let newuser = NewUser{
//			active:true, 
//			city: &String::from("San Jose")
//		};
//}
fn main() {
	//println!("Hello, world!");
	//let mut user1 = User {
	//    	active : true,
	//    	username : String::from("somename1"),
	//    	email : String::from("some@gmail.com"),
	//    	sign_in_count : 3,
	//    };
	//println!("User name {} ", user1.username);
	//user1.email = String::from("some1@gmail.com");
	//let new_user = build_user(String::from("mnarayanan"), String::from("mn@email.com"));
	//let duplicated_user = User{
	//	username:String::from("somename2"),
	//	.. new_user};
	//	
	//println!("User name {} ", new_user.username);
	//println!("User name {} ", new_user.active);
	//println!("User name {} ", new_user.sign_in_count);
	////email is already copied over to duplicated_user
	////println!("User name {} ", new_user.email);
	//println!("Duplicated User name {} ", duplicated_user.email);
	//println!("Duplicated User name {} ", duplicated_user.username);
	//println!("Duplicated User name {} ", duplicated_user.active);
	//println!("Duplicated User name {} ", duplicated_user.sign_in_count);
	//create_tuple_structs();
	//create_unit_like_structs();
	//area_by_vars();
	//area_by_tuple();
	area_by_struct();
}
fn create_tuple_structs() {
	let area = TupleStructArea(10, 20);
	let cordinate = TupleStructArea(20, 20);
	println!("Area {}", area.0 * area.1);
}
fn create_unit_like_structs() {
	let uls = MyUnitLikeStruct;
}
fn area_by_vars() {
	let width = 110;
	let height  = 50;
	println!("Area is {} ", area_by_vars_int(width, height));
}
fn area_by_vars_int (width: u32, length: u32) -> u32 {
	width * length
}
struct RecTuple ( u32, u32 );
fn area_by_tuple() {
	let rec_tuple = RecTuple (110, 50);
	println!("AreaByTuple {} ", area_by_tuple_int(&rec_tuple));
}
fn area_by_tuple_int(rec_tuple:&RecTuple) -> u32 {
	rec_tuple.0 * rec_tuple.1	
}
#[derive(Debug)]
struct Rectangle {
	width : u32,
	length : u32,
}
fn area_by_struct() {
	let rec_structure = Rectangle{width : 110, length :50};
	println!("Area by structure {} ", area_by_struct_int(&rec_structure));
	println!("print  Rectangle {:?} ", rec_structure);
	println!("print  Rectangle {:#?} ", rec_structure);
	dbg!(rec_structure);
	
}
fn area_by_struct_int(rectangle : &Rectangle) -> u32{
	rectangle.width * rectangle.length
}
