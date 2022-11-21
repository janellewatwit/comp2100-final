/**
 * Read a TCP message from a stream and print. Annotated with source IP.
 */
pub fn log_tcpread(stream: &std::net::TcpStream, ip: std::net::IpAddr)
{
	let mut reader= std::io::BufReader::new(stream);

	loop
	{
		let mut as_str = String::new();
		std::io::BufRead::read_line(&mut reader, &mut as_str).unwrap();
		print!("From {} | {}", ip, as_str);
		if as_str.len() < 3
		{
			break;
		}
	}
}
