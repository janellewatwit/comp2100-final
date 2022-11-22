pub mod blacklist;
pub mod listen;
pub mod proxy_server;
pub mod read_tcp;

/// Host a proxy server on port 20000
fn main()
{
	listen::listen(20000, proxy_server::handle_proxy);
}
