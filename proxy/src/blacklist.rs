use std::{io::Read, net::IpAddr, fs::File};

/// Check if an IpAddr is on the blacklist.
pub fn check_blacklisted_ip(ip: &IpAddr) -> bool
{
	// open file
	let mut contents = String::new();
	let mut file = File::open("./blacklist").unwrap();
	file.read_to_string(&mut contents).unwrap();

	// cast to Vec<String>
	let blacklisted_ips: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();

	// return contains
	blacklisted_ips.contains(&ip.to_string())
}
