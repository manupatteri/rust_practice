fn main() {
    const DAYS_IN_WEEK : u32 = 7;
    const months_in_a_year : u32 = 12;
    let mut x = 5;
    println!("Value of x is {x}");
    x = 5;
    println!("Value of x is {x} in all {DAYS_IN_WEEK} days");
    let crazy_shadow_example = "hello";
    println!("Value of crazy_shadow_example initially is {crazy_shadow_example} ");
    let crazy_shadow_example = crazy_shadow_example.len();
    println!("Value of crazy_shadow_example later is {crazy_shadow_example} ");
    {
        let crazy_shadow_example = 500;
        println!("Value of crazy_shadow_example inside different scope  is {crazy_shadow_example} ");
    }
    println!("Value of crazy_shadow_example outside different scope  is still {crazy_shadow_example} ");

}
