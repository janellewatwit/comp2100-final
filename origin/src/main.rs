pub mod origin_server;
pub mod common;

fn main()
{
	common::listen(10000, origin_server::handle_stream);
}
