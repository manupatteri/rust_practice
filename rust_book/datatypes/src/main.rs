use rand::Rng;
use std::mem;

fn main() {
    let guess: u32 = "234".parse().expect("not a member");
    let easy_to_read_number : u32 = 123_000;
    println!(" parsed_number {guess} ");
    println!("easy_to_read_number {easy_to_read_number}");
    //let overflow : u8 = 255 + 1;
    arithmetic();
    booleans();
    chars();
    tuples();
    arrays();
    strings(String::from("Tom"));
    integers();
    chars1();
    bools1();
    floats();
}
fn strings(name : String) {
    println!("His name is {name}");
}
fn arrays() {
  let array_without_type_count  = [1,2,3,4];
  let mut array_elem  = array_without_type_count[2];
  println!("element at a given index 2 is  {array_elem}");
  let mut rng = rand::thread_rng();

  //purely random number
  //let n1: usize = rng.gen();
  //random number within a range
  let n1: usize = rng.gen_range(0..5);
  array_elem  = array_without_type_count[n1];
  println!("got a safe index {n1} element at that index is  {array_elem}");
}
fn tuples() {
    let x : (i32, f64, u8) = (400, 3.2, 6);
    let integer_value = x.0;
    let float_value = x.1;
    let unsigned_value = x.2;
    println!("unsigned_value <- float_value <- integer_value" );
    println!("{unsigned_value} <- {float_value} <- {integer_value}" );
    
    let simple_tuple = (4.3, 4, 2.5, 6);
    let (a, b, c, d) = simple_tuple;
    println!("Printing only float {b} {d}");
}
fn arithmetic() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
fn booleans() {
    let t = true;
    let y : bool = false;
    println!("{t} {y}");
}
fn chars() {
  let x: isize = 2;
  println!("{x}");
}
fn integers() {
    let u8int : u8 = 4;
    let u16int : u16 = 45;
    let u32int : u32 = 456;
    let u64int : u64 = 4567;
    let u128int : u128 = 45678;
    let usizeint : usize = 45678;
    let i8int : i8 = -4;
    let i16int : i16 = -45;
    let i32int : i32 = -456;
    let i64int : i64 = -4567;
    let i128int : i128 = -45678;
    let isizeint : isize = -45678;
    println!("{u8int} {u16int} {u32int} {u64int} {u128int} {usizeint} {} {} {} {} {} ",
        mem::size_of_val(&u8int),
        mem::size_of_val(&u16int),
        mem::size_of_val(&u32int),
        mem::size_of_val(&u64int),
        mem::size_of_val(&u128int));
    println!("{i8int} {i16int} {i32int} {i64int} {i128int} {isizeint} {} {} {} {} {} ",
        mem::size_of_val(&i8int),
        mem::size_of_val(&i16int),
        mem::size_of_val(&i32int),
        mem::size_of_val(&i64int),
        mem::size_of_val(&i128int),
        );
}
fn chars1() {
    let mychar = 'c';
    let mychar_with_type : char = 'd';
    println!("{mychar} {} {mychar_with_type} {} ", mem::size_of_val(&mychar), mem::size_of_val(&mychar_with_type));

}
fn bools1() {
    let mybool : bool = true;
    println!("{mybool} {} ", 
        mem::size_of_val(&mybool));
}
fn floats() {
    let f32float : f32 = 3.14;
    let f64float : f64 = 3.14;
    let float_default = 3.14;
    println!("{} {} {} {} Default is f64 -> {} {}", 
        f32float,
        mem::size_of_val(&f32float),
        f64float,
        mem::size_of_val(&f64float),
        float_default,
        mem::size_of_val(&float_default),
        );
}
