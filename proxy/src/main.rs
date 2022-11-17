pub mod blacklist;
pub mod common;
pub mod proxy_server;

fn main() {
	common::listen(20000, proxy_server::handle_proxy);
}
