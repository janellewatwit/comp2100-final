use std::{
	io::{BufRead, BufReader, Write},
	net::TcpStream,
};

/// If the incoming TcpStream does not come from a blacklisted
/// ip, forwards an incoming message from the client to the
/// origin server, then forwards the response from the origin
/// server back to the client. Afterwards, terminates the connection.
pub fn handle_proxy(mut client_stream: TcpStream)
{
	// Get client's IP
	let ip = client_stream.peer_addr().unwrap().ip();
	println!("Received TCP connection from: {:?}", ip);
	// Check if IP is blacklisted
	if crate::blacklist::check_blacklisted_ip(&ip)
	{
		println!("{} is blacklisted. Closing connection", ip);
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

/// Forward a TCP message from one stream to another.
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
