const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 1;
fn main() {
	let missiles : i32 = STARTING_MISSILES;
	let ready : i32= READY_AMOUNT;
	let _unused_sample_var = 4;
    	println!("Firing {} of my {} missiles", ready, missiles);
	//missiles = missiles - ready;
    	//println!("{} missiles left ", missiles);
    	println!("{} missiles left ", missiles - ready);
	//If uncommented, tries to change a const variable
	//READY_AMOUNT = 1;
}
