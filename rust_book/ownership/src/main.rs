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

    let ref_input = String::from("John");
    let ref_length = calculate_length(&ref_input);
    println! ("{ref_input} got length of {ref_length}. The String \"{ref_input}\" is still valid ! ");

    let mut my_string = String::from("hello ");
    println! ("{my_string} about to be changed in a function via mutable reference ");
    change_string(&mut my_string);
    println! ("{my_string} changed in a function via mutable reference ");
    get_mutable_reference_twice();
    get_unmutable_reference_twice();
    //let reference_to_nothing = dangle();
    let copy = no_dangle();
    println! ("{copy} which is  not dangling");
    let word_length_string = String::from("hello world"); 
    let word = first_word(&word_length_string);
    println! ("{word} is where {word_length_string} has first word ending");
    print_slices(&String::from("hello world"));
    let word = first_word_as_slice(&word_length_string);
    println! ("first word found via slices from {word_length_string} is |{word}| ");
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
fn calculate_length(string_ref: &String) -> usize {
    string_ref.len()
}
fn change_string(input:&mut String) {
    input.push_str(" world");
}
fn get_mutable_reference_twice() {
    let mut name = String::from("Manoj");

    let mut_name_ref_first = &mut name;
//   let mut_name_ref_second = &mut name;
//   error[E0499]: cannot borrow `name` as mutable more than once at a time
//  --> src/main.rs:86:31
//   |
//85 |     let mut_name_ref_first = &mut name;
//   |                              --------- first mutable borrow occurs here
//86 |     let mut_name_ref_second = &mut name;
//   |                               ^^^^^^^^^ second mutable borrow occurs here
//87 |     mut_name_ref_first.push_str(" N");
//   |     --------------------------------- first borrow later used here
//
//For more information about this error, try `rustc --explain E0499`.
    mut_name_ref_first.push_str(" N");

}
fn get_unmutable_reference_twice() {
    let mut name = String::from("Hello");

    let ref1 = &name;
    let ref2 = &name;

    println!("ref1: {ref1} ref2: {ref2}");

    let mut_ref1 = &mut name;
    //error[E0502]: cannot borrow `name` as mutable because it is also borrowed as immutable
//   --> src/main.rs:109:20
//    |
//105 |     let ref2 = &name;
//    |                ----- immutable borrow occurs here
//...
//109 |     let mut_ref1 = &mut name;
//    |                    ^^^^^^^^^ mutable borrow occurs here
//110 |     println!("mut_ref1: {mut_ref1} ref2: {ref2}");
//    |                                           ---- immutable borrow later used here
//
//For more information about this error, try `rustc --explain E0502`.
//    println!("mut_ref1: {mut_ref1} ref2: {ref2}");
    println!("mut_ref1: {mut_ref1} ");

}
//fn dangle() -> &String {
//       Compiling ownership v0.1.0 (/Users/manoj/Documents/Learn/rust/rust_practice/rust_book/ownership)
//error[E0106]: missing lifetime specifier
//   --> src/main.rs:128:16
//    |
//128 | fn dangle() -> &String {
//    |                ^ expected named lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//help: consider using the `'static` lifetime
//    |
//128 | fn dangle() -> &'static String {
//    |                 +++++++
//
//For more information about this error, try `rustc --explain E0106`.
//error: could not compile `ownership` due to previous error

//    let s = String::from("sample");
//    &s
//}
fn no_dangle() -> String {
    let s = String::from("sample");
    s
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn print_slices(my_string: &String) {
    let first = &my_string[0..5];
    let second = &my_string[6..11];
    println!("|{second}| : |{first}|");
    let default_start = &my_string[..5];
    let mut is_equal = first == default_start;
    println!("is_equal : |{is_equal}|");
    let default_end = &my_string[6..];
    is_equal = second == default_end;
    println!("is_equal : |{is_equal}|");
}
fn first_word_as_slice(my_word: &String) -> &str {
    let my_word_as_bytes = my_word.as_bytes();
    for (i, &item) in my_word_as_bytes.iter().enumerate() {
        if item == b' ' {
            return &my_word[0..i];
        }
    }
    return my_word;
}
