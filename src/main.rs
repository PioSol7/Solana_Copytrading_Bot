use bincode::Options;
use jito_json_rpc_client::jsonrpc_client::rpc_client::RpcClient as JitoRpcClient;
use temp::common::utils::{
    create_arc_rpc_client, create_nonblocking_rpc_client, import_arc_wallet, import_env_var,
    import_wallet, log_message, AppState,
};
use temp::core::token::get_account_info;
use temp::core::tx::jito_confirm;
use temp::engine::swap::{pump_swap, raydium_swap};
// use copy_trading_bot::dex::pump::pump_sdk_swap;
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::Value;
use solana_sdk::message::VersionedMessage;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;
use spl_associated_token_account::get_associated_token_address;
use std::env;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use tokio::time::Instant;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};

#[derive(Serialize)]
struct SwapRequest {
    quoteResponse: serde_json::Value, // You may deserialize it into a specific struct if known
    userPublicKey: String,
    wrapAndUnwrapSol: bool,
    dynamicComputeUnitLimit: bool,
    prioritizationFeeLamports: u64,
}
#[tokio::main]

async fn main() {
    dotenv().ok();
    let target = env::var("TARGET_PUBKEY").expect("TARGET not set");

    let rpc_client = create_arc_rpc_client().unwrap();
    let rpc_nonblocking_client = create_nonblocking_rpc_client().await.unwrap();
    let wallet = import_arc_wallet().unwrap();

    let state = AppState {
        rpc_client,
        rpc_nonblocking_client,
        wallet,
    };
    pub static BLOCK_ENGINE_URL: LazyLock<String> =
        LazyLock::new(|| import_env_var("JITO_BLOCK_ENGINE_URL"));
    let jito_client = Arc::new(JitoRpcClient::new(format!(
        "{}/api/v1/bundles",
        *BLOCK_ENGINE_URL
    )));
    let unwanted_key = env::var("JUP_PUBKEY").expect("JUP_PUBKEY not set");
    let ws_url = env::var("RPC_WEBSOCKET_ENDPOINT").expect("RPC_WEBSOCKET_ENDPOINT not set");

    let (ws_stream, _) = connect_async(ws_url)
        .await
        .expect("Failed to connect to WebSocket server");
    let (mut write, mut read) = ws_stream.split();
    // Subscribe to logs
    let subscription_message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "transactionSubscribe",
        "params": [

            {
                "failed": false,
                "accountInclude": ["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"],
                "accountExclude": [unwanted_key],
                // Optionally specify accounts of interest
            },
            {
                "commitment": "processed",
                "encoding": "jsonParsed",
                "transactionDetails": "full",
                "maxSupportedTransactionVersion": 0
            }
        ]
    });

    write
        .send(subscription_message.to_string().into())
        .await
        .expect("Failed to send subscription message");

    let _ = log_message("---------------------   Copy-trading-bot start!!!  ------------------\n")
        .await;

    // Listen for messages
    while let Some(Ok(msg)) = read.next().await {
        if let WsMessage::Text(text) = msg {
            let json: Value = serde_json::from_str(&text).unwrap();

            let sig = json["params"]["result"]["signature"]
                .as_str()
                .unwrap_or_default();
            let timestamp = Instant::now();

            // filter tx raydium part
            tx_ray();

            // filter tx pumpfun part
            tx_pump();
        }
    }
}

pub async fn tx_ray(
    json: Value,
    target: String,
    timestamp: Instant,
    state: AppState,
    jito_client: Arc<JitoRpcClient>,
) {
    // parsing tx part

    if  {
        dirs = "buy".to_string();
        swap_to_events_on_raydium(
            mint,
            amount_in * percent / 100,
            dirs,
            pool_id,
            timestamp.clone(),
            jito_client.clone(),
            state.clone(),
        )
        .await;
    } else {
        dirs = "sell".to_string();
        swap_to_events_on_raydium(
            mint,
            amount_in * percent / 100,
            dirs,
            pool_id,
            timestamp.clone(),
            jito_client.clone(),
            state.clone(),
        )
        .await;
    }
}

pub async fn tx_pump(
    json: Value,
    target: String,
    timestamp: Instant,
    state: AppState,
    jito_client: Arc<JitoRpcClient>,
) {
    // Iterate over logs and check for unwanted_key

    if  {
        dirs = "buy".to_string();
        swap_to_events_on_pump(
            mint,
            amount_in * percent / 100,
            dirs,
            timestamp.clone(),
            jito_client.clone(),
            state.clone(),
        )
        .await;
    } else {
        dirs = "sell".to_string();

        swap_to_events_on_pump(
            mint,
            amount_in * percent / 100,
            dirs,
            timestamp.clone(),
            jito_client.clone(),
            state.clone(),
        )
        .await;
    }
}

pub async fn swap_on_jup(mint: String, dir: String, amount: u64) {
    // get tx
    jito_confirm()
}
pub async fn swap_to_events_on_pump(
    mint: String,
    amount_in: u64,
    dirs: String,
    timestamp: Instant,
    jito_client: Arc<JitoRpcClient>,
    state: AppState,
) {
    println!("2: {:#?}", timestamp.elapsed().clone());

    let slippage = 10000;
    println!("2.1: {:#?}", timestamp.elapsed());
    let res = pump_swap(
        state,
        amount_in,
        &dirs,
        slippage,
        &mint,
        jito_client,
        timestamp.clone(),
    )
    .await;
}

pub async fn swap_to_events_on_raydium(
    mint: String,
    amount_in: u64,
    dirs: String,
    pool_id: String,
    timestamp: Instant,
    jito_client: Arc<JitoRpcClient>,
    state: AppState,
) {
    println!("2: {:#?}", timestamp.elapsed().clone());

    let slippage = 10000;
    println!("2.1: {:#?}", timestamp.elapsed());
    let res = raydium_swap(
        state,
        amount_in,
        &dirs,
        pool_id,
        slippage,
        &mint,
        jito_client,
        timestamp.clone(),
    )
    .await;
}
