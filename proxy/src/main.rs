use std::{net::{TcpListener, TcpStream}, io::{Write, Read, BufReader, BufRead}};

fn check_blacklisted_ip(ip: &std::net::IpAddr) -> bool {
	let mut file = std::fs::File::open("./blacklist").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	let blacklisted_ips: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();
	
	blacklisted_ips.contains(&ip.to_string())
}

fn handle_client(mut incoming_stream: TcpStream) {
	let ip = incoming_stream.peer_addr().unwrap();
	println!("{:?}", ip);
	if check_blacklisted_ip(&ip.ip()) {
		println!("Bad ip");
		incoming_stream.write(b"Bad ip\n").unwrap();
		return;
	}

	let mut outgoing_stream = match TcpStream::connect("origin:10000") {
		Ok(stream) => stream,
		Err(e) => {
			println!("Failed to connect to origin server!");
			println!("{}", e);
			return;
		}
	};

	outgoing_stream.write(b"howdy\n").unwrap();
	outgoing_stream.write(b"\n").unwrap();

	let mut reader = BufReader::new(&outgoing_stream);
	loop {
		let mut as_str = String::new();
		reader.read_line(&mut as_str).unwrap();
		print!("{}", as_str);
		incoming_stream.write(as_str.as_bytes()).unwrap();
		if as_str.len() < 3 {
			break;
		}
	}
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("0.0.0.0:20000")?;
	println!("Started listening on :20000");

	// accept connections and process them serially
	for stream in listener.incoming() {
		handle_client(stream?);
	}
	Ok(())
}
