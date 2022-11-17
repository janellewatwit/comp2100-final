use std::io::Read;

/**
 * Check if an IpAddr is on the blacklist
 */
pub fn check_blacklisted_ip(ip: &std::net::IpAddr) -> bool {
	let mut file = std::fs::File::open("./blacklist").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	let blacklisted_ips: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();

	blacklisted_ips.contains(&ip.to_string())
}
