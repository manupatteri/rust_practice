fn main() {
    println!("Hello, world!");
    another_func();
    func_with_multiple_args(45, 'c');
}
fn another_func() {
  println!("another function");
}
fn func_with_multiple_args(intvalue : i32, charvalue : char) {
    println!("{}", intvalue + 1);
    println!("next arg is {charvalue}");
}
func scope_block_as_expression() {
   let a = {
       let b = 5;
       b + 3
   }
    println!("scope expression returned {a}");
}
