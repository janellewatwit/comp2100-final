use std::{
	net::TcpStream,
	io::Write
};

/// Reply "Hello World!" to any incoming TCP stream.
/// Log incoming message contents.
pub fn handle_stream(mut stream: TcpStream)
{
	// Log receive connection
	let ip = stream.peer_addr().unwrap().ip();
	println!("Received TCP connection from: {:?}", ip);

	// Log request
	let request = crate::read_tcp::read_stream(&stream);
	print!("From {} |\n{}", ip, request);

	// Reply
	stream.write(b"Hello World!\n").unwrap();
}
