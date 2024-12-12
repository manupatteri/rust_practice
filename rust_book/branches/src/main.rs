use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let first_num: usize = rng.gen_range(0..10);
    if_condition(first_num);
    let second_num: usize = rng.gen_range(0..15);
    if_else_condition(second_num);
    println!("number of digits in {second_num} is {}", if_expression(second_num));
}

fn if_condition(num : usize) {
    if num % 2 == 0 {
        println!("{num} is even ");
    } else {
        println!("{num} is odd ");
    }
} 
fn if_else_condition(num : usize) {
    if num < 5 {
        println!("{num} less than 5 ");
    } else if num > 5 && num < 10 {
        println!("{num} is between 5 and 10 ");
    } else if num > 10 {
        println!("{num} is more than 10 ");
    } else {
        println!("{num} is somewhere in between ");
    } 
}
fn if_expression(num : usize ) -> u32 {
  let result = if num > 10 { 2 } else { 1 };
  return result;
}
