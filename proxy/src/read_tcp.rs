use std::{net::{TcpStream}, io::{BufReader, BufRead}};

pub fn read_stream(stream: &TcpStream) -> String
{
	let mut reader = BufReader::new(stream);
	let mut out = String::new();

	loop
	{
		let mut as_str = String::new();
		reader.read_line(&mut as_str).unwrap();
		out.push_str(&as_str);
		if as_str.len() < 3
		{
			break;
		}
	}

	out
}
