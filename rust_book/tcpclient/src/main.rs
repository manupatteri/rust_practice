use std::net::TcpStream;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
	for n in 1..10 {
		let mut stream = TcpStream::connect("127.0.0.1:7878")?;
		stream.write(b"key=value")?;
	}
	Ok(())

}
