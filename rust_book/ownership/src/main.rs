fn main() {
    println!("Hello, world!");
    let mut s1 = String::from("hello");
    println!("{s1} world");
    let s2 = s1;
    //println!("{s1} world");
    println!("{s2} world");
    s1 = s2.clone();
    println!("{s1} {s2} ");
    let mut only_variable = String::from("First");
    only_variable = String::from("Second");

    //At this point, nothing is referring to the original value("First") on the heap at all. 
    //The original string thus immediately goes out of scope. Rust will run the drop function on it and its memory will be freed right away. 
    //When we print the value at the end, it will be "Second".

    println!("{only_variable}");
    let x : i32 = 5;
    let y = x;
    println! ("First Var {x} is still valid while new one {y} got initialized with same value");
    takes_ownership(only_variable);
    //error[E0382]: borrow of moved value: `only_variable`
    //  --> src/main.rs:22:31
    //   |
    //10 |     let mut only_variable = String::from("First");
    //   |         ----------------- move occurs because `only_variable` has type `String`, which does not implement the `Copy` trait
    //...
    //21 |     takes_ownership(only_variable);
    //   |                     ------------- value moved here
    //22 |     println! ("only_variable {only_variable} no more valid here ");
    //   |                               ^^^^^^^^^^^^^ value borrowed here after move
    //   |

    //println! ("only_variable {only_variable} no more valid here ");
    let num = 4;
    makes_copy(num);
    println! ("scalar can be used after being used function use: {num}");
    let new_string = gives_ownership();
    println! ("Got ownership: {new_string} ");
    let operational_string = takes_and_gives_back(String::from("input"));
    println! ("{operational_string} got ownership back ");

    let input =String::from("John Doe"); 
    let (my_str, length) = my_calculate_length(input);
    println! ("{my_str} got length of {length} ");

}
fn takes_ownership(some_string : String) {
    println!("some_string:{some_string} moved here inside Fn takes_ownership");
}//some_string goes out of scope and drop getting called.

fn makes_copy(some_integer: i32) {
    println!("Copying scalar type:{some_integer}");
}
fn gives_ownership() -> String {
    println! ("gives_ownership : Creates a string and gives ownership to caller");
    String::from("OwnershipGiven")
}
fn takes_and_gives_back(input: String) -> String {
    println! ("takes_and_gives_back : Got a string and returns ownership to caller");
    input
}
fn my_calculate_length(value:String) -> (String, usize)  {
    let length = value.len();
    (value, length)
}
