fn main() {
    println!("Hello, world!");
    if_else(-5);
    if_else(0);
    if_else(9);
    return_from_if_else(10);
}
fn if_else(n: i32) {
	if n < 0 {
		println!("{} is negative",n);
	} else if n == 0 {
		println!("{} is 0", n);
	} else {
		println!("{} is positive", n);
	}
}
fn return_from_if_else (n: i32) {
	let big_n = 
		if n < 0 && n > -10 {
			n * 10
//***** Error when both ended with semicolon
//error[E0277]: `()` doesn't implement `std::fmt::Display`
//  --> src/main.rs:24:31
//   |
//24 |     println! ( " result is {} ", big_n);
//   |                                  ^^^^^ `()` cannot be formatted with the default formatter
//   |
//   = help: the trait `std::fmt::Display` is not implemented for `()`
//   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
//
//For more information about this error, try `rustc --explain E0277`.
		} else {
			n/2
//********** Error when one section ended with semicolon
//error[E0308]: `if` and `else` have incompatible types
//  --> src/main.rs:34:4
//   |
//19 | /         if n < 0 && n > -10 {
//20 | |             n * 10
//   | |             ------ expected because of this
//21 | | //***** Error when both ended with semicolon
//22 | | //error[E0277]: `()` doesn't implement `std::fmt::Display`
//...  |
//34 | |             n/2;
//   | |             ^^^-
//   | |             |  |
//   | |             |  help: consider removing this semicolon
//   | |             expected `i32`, found `()`
//35 | |         };
//   | |_________- `if` and `else` have incompatible types
//
//For more information about this error, try `rustc --explain E0308`.
		};
	println! ( " result is {} ", big_n);
}
