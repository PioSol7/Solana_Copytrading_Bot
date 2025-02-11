use std::{env, sync::Arc, time::Duration};

use anyhow::Result;
use jito_json_rpc_client::jsonrpc_client::rpc_client::RpcClient as JitoRpcClient;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    signature::Keypair,
    signer::Signer,
    system_transaction,
    transaction::{Transaction, VersionedTransaction},
};
use spl_token::ui_amount_to_amount;

use std::str::FromStr;
use tokio::time::Instant;

use crate::{
    common::utils::log_message,
    services::jito::{
        self, get_tip_account, get_tip_value, init_tip_accounts, wait_for_bundle_confirmation,
    },
};

// prioritization fee = UNIT_PRICE * UNIT_LIMIT
fn get_unit_price() -> u64 {
    env::var("UNIT_PRICE")
        .ok()
        .and_then(|v| u64::from_str(&v).ok())
        .unwrap_or(1)
}

fn get_unit_limit() -> u32 {
    env::var("UNIT_LIMIT")
        .ok()
        .and_then(|v| u32::from_str(&v).ok())
        .unwrap_or(300_000)
}
pub async fn jito_confirm(
    keypair: &Keypair,
    version_tx: VersionedTransaction,
    recent_block_hash: &Hash,
) {
    // jito confirm
}

pub async fn new_signed_and_send(
    client: &RpcClient,
    keypair: &Keypair,
    mut instructions: Vec<Instruction>,
    jito_client: Arc<JitoRpcClient>,
    timestamp: Instant,
) -> Result<Vec<String>> {
    // jito confirm
}
