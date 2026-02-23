fn main() {
    let mut the_area = area (3, 5);
    println!("Hello, world! {} ", the_area);
    let tuple = (3,5);
    the_area = area_by_tuple (tuple);
    println!("Hello, world! {} ", the_area);
}
fn area(width: u32, height: u32) -> u32{
    width * height
}
fn area_by_tuple(the_tuple : (u32, u32)) -> u32 {
    the_tuple.0 * the_tuple.1
}
