/**
 * Host a service over TCP. Choose the port and a function
 * handler to manage open TcpStreams.
 */
pub fn listen(port: u16, handler: fn(std::net::TcpStream))
{
	let listener = std::net::TcpListener::bind(
		std::net::SocketAddr::from(([0,0,0,0], port))
	).unwrap();
	println!("Started listening on :{}", port);

	// accept connections and process them serially
	for stream in listener.incoming()
	{
		match stream
		{
			Ok(stream) => handler(stream),
			Err(err) => println!("{}", err)
		}
	}
}
