use anyhow::Result;
use chrono::Local;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};
use std::process;
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub rpc_client: Arc<solana_client::rpc_client::RpcClient>,
    pub rpc_nonblocking_client: Arc<solana_client::nonblocking::rpc_client::RpcClient>,
    pub wallet: Arc<Keypair>,
}

pub struct ParseTx {
    pub type_tx: String,
    pub direction: Option<String>,
    pub amount_in: f64,
    pub amount_out: f64,
    pub mint: String,
}

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::time::SystemTime;

pub async fn log_message(message: &str) -> io::Result<()> {
    // Open the file in append mode. If the file doesn't exist, it will be created.
    let mut file = OpenOptions::new()
        .append(true) // Ensure the log is appended, not overwritten
        .create(true) // Create the file if it doesn't exist
        .open("./src/log.txt")?;

    // Get the current timestamp
    let now = Local::now();

    // Format the time as "HH:MM:SS"
    now.format("%H:%M:%S").to_string();

    // Write the log message with a timestamp
    writeln!(file, "[{:#?}] {}", now.clone(), message)?;

    Ok(())
}

pub fn read_log() -> io::Result<String> {
    // Read the entire content of the log file
    let mut file = File::open("log.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn import_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Environment variable {} is not set", key))
}

pub fn create_rpc_client() -> Result<solana_client::rpc_client::RpcClient> {
    let rpc_https = import_env_var("RPC_ENDPOINT");
    let rpc_client = solana_client::rpc_client::RpcClient::new_with_commitment(
        rpc_https,
        CommitmentConfig::processed(),
    );
    Ok(rpc_client)
}
pub fn create_arc_rpc_client() -> Result<Arc<solana_client::rpc_client::RpcClient>> {
    let rpc_https = import_env_var("RPC_ENDPOINT");
    let rpc_client = solana_client::rpc_client::RpcClient::new_with_commitment(
        rpc_https,
        CommitmentConfig::processed(),
    );
    Ok(Arc::new(rpc_client))
}

pub async fn create_nonblocking_rpc_client(
) -> Result<Arc<solana_client::nonblocking::rpc_client::RpcClient>> {
    let rpc_https = import_env_var("RPC_ENDPOINT");
    let rpc_client = solana_client::nonblocking::rpc_client::RpcClient::new_with_commitment(
        rpc_https,
        CommitmentConfig::processed(),
    );
    Ok(Arc::new(rpc_client))
}

pub fn import_wallet() -> Result<Keypair> {
    let mut file = File::open("./key.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents == "" {
        println!("Not set Private Key");
    }
    let wallet: Keypair = Keypair::from_base58_string(&contents);

    Ok(wallet)
}
pub fn import_arc_wallet() -> Result<Arc<Keypair>> {
    let mut file = File::open("./key.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let wallet: Keypair = Keypair::from_base58_string(contents.as_str());

    Ok(Arc::new(wallet))
}
