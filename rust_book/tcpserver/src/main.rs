use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;

fn old_main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Hello, world!");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
	handle_connection(stream);
    }
}
fn old_handle_connection(mut stream: TcpStream) {
	let peer_addr = stream.peer_addr().unwrap();
	println!("Connection: {peer_addr} args");
	let buf_reader = BufReader::new(&mut stream);
	
	let http_request: Vec<_> = buf_reader
			.lines()
			.map(|result| result.unwrap())
			.take_while(|line| !line.is_empty())
			.collect();
			
	let status_line = "HTTP/1.1 200 OK\r\n\r\n";
	let contents = fs::read_to_string("hello.html").unwrap();
	let length = contents.len();


    	let response =
        	format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");


	stream.write_all(response.as_bytes()).unwrap();

	//println!("Request : {http_request:#?}");
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Start server ");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
	handle_connection(stream);
    }

}
fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&mut stream);
 	let http_requests: Vec<_> = buf_reader
        	.lines()
        	.map(|result| result.unwrap())
        	.take_while(|line| !line.is_empty())
        	.collect();

	#println!("Connection: {peer_addr} args");
	for request in http_requests {
		println!("{request}"
	}
        let peer_addr = stream.peer_addr().unwrap();
	let a = 5;
	if a > 3 {
        	println!("Connection established {} ", peer_addr);
	}
}
