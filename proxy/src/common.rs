use std::{
	io::{BufRead, BufReader},
	net::{IpAddr, SocketAddr, TcpListener, TcpStream},
};

pub fn listen(port: u16, handler: fn(TcpStream)) {
	let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).unwrap();
	println!("Started listening on :{}", port);

	// accept connections and process them serially
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => handler(stream),
			Err(err) => println!("{}", err),
		}
	}
}

pub fn log_tcpread(stream: &TcpStream, ip: IpAddr)
{
	let mut reader= BufReader::new(stream);

	loop
	{
		let mut as_str = String::new();
		reader.read_line(&mut as_str).unwrap();
		print!("From {} | {}", ip, as_str);
		if as_str.len() < 3
		{
			break;
		}
	}
}
