use std::io;
use std::io::prelude::*;
use std::sync::Arc;

use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::{Builder, Config, LogLevel};

use ldk_node::bitcoin::Network;

fn main() {
	let mut config = Config::default();
	config.network = Network::Signet;
	//config.trusted_peers_0conf = vec![lsp_node_id.clone()];

	let mut builder = Builder::from_config(config);
	builder.set_storage_dir_path("/tmp/ldk_node_poc/".to_string());
	builder.set_log_level(LogLevel::Gossip);
	builder.set_esplora_server("https://mutinynet.com/api/".to_string());

	let lsp_node_id: PublicKey = "..".parse().unwrap();
	let lsp_address = "..".parse().unwrap();
	let lsp_token = Some("..".to_string());
	builder.set_liquidity_source_lsps2(lsp_address, lsp_node_id, lsp_token);

	let node = Arc::new(builder.build().unwrap());
	node.start().unwrap();

	let event_node = Arc::clone(&node);
	std::thread::spawn(move || loop {
		let event = event_node.wait_next_event();
		println!("GOT NEW EVENT: {:?}", event);
		println!("Channels: {:?}", event_node.list_channels());
		println!("Payments: {:?}", event_node.list_payments());
		event_node.event_handled();
	});

	println!("Channels: {:?}", node.list_channels());
	println!("Payments: {:?}", node.list_payments());
	let invoice = node.receive_payment_via_jit_channel(1_000_000_000, "asdf", 3600, None).unwrap();
	println!("INVOICE: {}", invoice);
	pause();

	node.stop().unwrap();
}

fn pause() {
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	// We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
	write!(stdout, "Press any key to continue...").unwrap();
	stdout.flush().unwrap();

	// Read a single byte and discard
	let _ = stdin.read(&mut [0u8]).unwrap();
}
