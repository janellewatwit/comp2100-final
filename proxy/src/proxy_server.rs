use std::{
	io::{BufRead, BufReader, Write},
	net::TcpStream,
};

pub fn handle_proxy(mut client_stream: TcpStream)
{
	// Get client's IP
	let ip = client_stream.peer_addr().unwrap();
	println!("Received TCP connection from: {:?}", ip.ip());
	// Check if IP is blacklisted
	if crate::blacklist::check_blacklisted_ip(&ip.ip())
	{
		println!("{} is blacklisted. Closing connection", ip.ip());
		client_stream.write(b"Your IP address is blacklisted.\n").unwrap();
		return;
	}

	// Log incoming request from client
	// crate::common::log_tcpread(&client_stream, ip.ip());

	// Open a connection to the Origin Server
	let mut origin_stream = match TcpStream::connect("origin:10000")
	{
		Ok(stream) => stream,
		Err(e) =>
		{
			println!("Failed to connect to origin server!");
			println!("{}", e);
			return;
		}
	};

	// Forward request from Client to Origin Server
	forward_tcp(&client_stream, &mut origin_stream);

	// Forward response from Origin Server to Client
	forward_tcp(&origin_stream, &mut client_stream);
}

fn forward_tcp(from: &TcpStream, to: &mut TcpStream)
{
	let mut reader = BufReader::new(from);
	loop
	{
		let mut as_str = String::new();
		reader.read_line(&mut as_str).unwrap();
		print!("From {} | {}", from.peer_addr().unwrap().ip(), as_str);
		to.write(as_str.as_bytes()).unwrap();
		if as_str.len() < 3
		{
			break;
		}
	}
}
