use std::{net::{TcpStream, IpAddr}, io::{BufReader, BufRead}};

/// Read a TCP message from a stream and print. Annotated with source IP.
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
