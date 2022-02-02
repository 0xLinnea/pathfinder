use pathfinder_lib::{config, rpc, sequencer, storage::Storage};

#[tokio::main]
async fn main() {
    println!("🏁 Starting node.");
    let config =
        config::Configuration::parse_cmd_line_and_cfg_file().expect("Configuration failed");

    // TODO: get database path from configuration
    let storage = Storage::migrate("database.sqlite".into()).unwrap();
    // TODO: pick the correct sequencer based on the Ethereum chain.
    let sequencer = sequencer::Client::goerli().unwrap();

    let (_handle, local_addr) = rpc::run_server(config.http_rpc_addr, storage, sequencer)
        .expect("⚠️ Failed to start HTTP-RPC server");
    println!("📡 HTTP-RPC server started on: {}", local_addr);
    let () = std::future::pending().await;
}
