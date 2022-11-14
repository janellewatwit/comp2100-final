use std::{net::{TcpListener, TcpStream}, io::{Write, BufReader, BufRead}};

fn handle_client(mut stream: TcpStream) {
	let ip = stream.peer_addr().unwrap().ip();
	println!("{:?}", ip);

	stream.write(b"Hello World!\n").unwrap();

	let mut reader= BufReader::new(&stream);

	loop {
		let mut as_str = String::new();
		reader.read_line(&mut as_str).unwrap();
		if as_str.len() < 3 {
			break;
		}
	}
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:10000")?;

	// accept connections and process them serially
	for stream in listener.incoming() {
		handle_client(stream?);
	}
	Ok(())
}
