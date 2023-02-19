//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}

/*

	./target/release/node-template purge-chain --base-path /tmp/alice --chain local

	./target/release/node-template \
	--base-path /tmp/alice \
	--chain local \
	--alice \
	--port 30333 \
	--ws-port 9945 \
	--rpc-port 9933 \
	--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
	--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
	--validator

*/

/*

	./target/release/node-template purge-chain --base-path /tmp/bob --chain local -y

	./target/release/node-template \
	--base-path /tmp/bob \
	--chain local \
	--bob \
	--port 30334 \
	--ws-port 9946 \
	--rpc-port 9934 \
	--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
	--validator \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp

*/
