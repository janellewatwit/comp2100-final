pub mod origin_server;
pub mod read_tcp;
pub mod listen;

/// Run the origin_server
fn main()
{
	listen::listen(10000, origin_server::handle_stream);
}
