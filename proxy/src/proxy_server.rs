use std::{
	io::{Write},
	net::TcpStream, thread::sleep,
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
	// brief pause for pretty output
	sleep(std::time::Duration::from_millis(50));

	// Forward request from Client to Origin Server
	let request = crate::read_tcp::read_stream(&client_stream);
	print!("From {} |\n{}", ip, request);
	// brief pause for pretty output
	sleep(std::time::Duration::from_millis(50));
	origin_stream.write(request.as_bytes()).unwrap();


	// brief pause for pretty output
	sleep(std::time::Duration::from_millis(50));

	// Forward response from Origin Server to Client
	let response = crate::read_tcp::read_stream(&origin_stream);
	print!("From {} |\n{}", ip, response);
	client_stream.write(response.as_bytes()).unwrap();
}
