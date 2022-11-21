use std::{
	net::TcpStream,
	io::Write
};

/// Reply "Hello World!" to any incoming TCP stream.
/// Log incoming message contents.
pub fn handle_stream(mut stream: TcpStream)
{
	let ip = stream.peer_addr().unwrap().ip();
	println!("Received TCP connection from: {:?}", ip);
	crate::read_tcp::log_tcpread(&stream, ip);
	stream.write(b"Hello World!\n").unwrap();
}
