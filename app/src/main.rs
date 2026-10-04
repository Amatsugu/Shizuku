use client::init_client;
#[cfg(feature = "server")]
use server::init_server;

fn main() {
	#[cfg(all(feature = "client", not(feature = "server")))]
	init_client();
	#[cfg(all(feature = "server", not(feature = "client")))]
	init_server();
}
