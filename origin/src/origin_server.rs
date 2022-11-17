use std::{
	net::TcpStream,
	io::Write
};

/**
 * Reply "Hello World!" to any incoming TCP stream
 */
pub fn handle_stream(mut stream: TcpStream)
{
	let ip = stream.peer_addr().unwrap();
	println!("Received TCP connection from: {:?}", ip.ip());
	crate::common::log_tcpread(&stream, ip.ip());
	stream.write(b"Hello World!\n").unwrap();
}
