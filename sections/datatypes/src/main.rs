use rand::Rng;

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
}
fn arrays() {
  let array_without_type_count  = [1,2,3,4];
  let mut array_elem  = array_without_type_count[2];
  println!("element at a given index 2 is  {array_elem}");
  let mut rng = rand::thread_rng();

  //purely random number
  //let n1: usize = rng.gen();
  //random number within a range
  let n1: usize = rng.gen_range(0..10);
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
