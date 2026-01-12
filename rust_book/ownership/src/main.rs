fn main() {
    println!("Hello, world!");
    let mut s1 = String::from("hello");
    println!("{s1} world");
    let s2 = s1;
    //println!("{s1} world");
    println!("{s2} world");
    s1 = s2.clone();
    println!("{s1} {s2} ");
}
