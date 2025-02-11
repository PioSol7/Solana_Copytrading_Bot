use crate::{
    core::{
        token::{get_account_info, get_mint_info},
        tx,
    },
    engine::swap::{SwapDirection, SwapInType},
};
use amm_cli::AmmSwapInfoResult;
use anyhow::{anyhow, Context, Result};
use bytemuck;
use jito_json_rpc_client::jsonrpc_client::rpc_client::RpcClient as JitoRpcClient;
use raydium_amm::state::{AmmInfo, Loadable};
use serde::Deserialize;
use serde::Serialize;
use solana_client::rpc_filter::{Memcmp, RpcFilterType};
use solana_sdk::{
    instruction::Instruction, program_pack::Pack, pubkey::Pubkey, signature::Keypair,
    signer::Signer, system_instruction,
};
use spl_associated_token_account::{
    get_associated_token_address, get_associated_token_address_with_program_id,
    instruction::create_associated_token_account_idempotent,
};
use spl_token::{amount_to_ui_amount, state::Account, ui_amount_to_amount};
use spl_token_client::token::TokenError;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::time::Instant;

pub const AMM_PROGRAM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
pub const RAYDIUM_AUTHORITY_V4: &str = "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1";

#[derive(Serialize)]
struct SwapRequest {
    quoteResponse: serde_json::Value, // You may deserialize it into a specific struct if known
    userPublicKey: String,
    wrapAndUnwrapSol: bool,
    dynamicComputeUnitLimit: bool,
    prioritizationFeeLamports: u64,
}

#[derive(Debug, Deserialize)]
pub struct PoolInfo {
    pub success: bool,
    pub data: PoolData,
}

#[derive(Debug, Deserialize)]
pub struct PoolData {
    // pub count: u32,
    pub data: Vec<Pool>,
}

impl PoolData {
    pub fn get_pool(&self) -> Option<Pool> {
        self.data.first().cloned()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pool {
    pub id: String,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "mintA")]
    pub mint_a: Mint,
    #[serde(rename = "mintB")]
    pub mint_b: Mint,
    #[serde(rename = "marketId")]
    pub market_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mint {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

pub struct Raydium {
    pub rpc_nonblocking_client: Arc<solana_client::nonblocking::rpc_client::RpcClient>,
    pub rpc_client: Option<Arc<solana_client::rpc_client::RpcClient>>,
    pub keypair: Arc<Keypair>,
    pub pool_id: Option<String>,
}

impl Raydium {
    pub fn new(
        rpc_nonblocking_client: Arc<solana_client::nonblocking::rpc_client::RpcClient>,
        rpc_client: Arc<solana_client::rpc_client::RpcClient>,
        keypair: Arc<Keypair>,
    ) -> Self {
        Self {
            rpc_nonblocking_client,
            keypair,
            rpc_client: Some(rpc_client),
            pool_id: None,
        }
    }

    pub async fn swap_by_mint(
        &self,
        mint_str: &str,
        swap_direction: SwapDirection,
        amount_in: u64,
        pool_id: String,
        slippage: u64,
        start_time: Instant,
        jito_client: Arc<JitoRpcClient>,
    ) -> Result<Vec<String>> {
        // make instructions on raydium

        tx::new_signed_and_send(
            &self.rpc_client.clone().unwrap(),
            &self.keypair,
            instructions,
            jito_client.clone(),
            start_time.clone(),
        )
        .await
    }
}
pub fn amm_swap(
    amm_program: &Pubkey,
    result: AmmSwapInfoResult,
    user_owner: &Pubkey,
    user_source: &Pubkey,
    user_destination: &Pubkey,
    amount_specified: u64,
    other_amount_threshold: u64,
    swap_base_in: bool,
) -> Result<Instruction> {
    Ok(swap_instruction)
}

pub async fn get_pool_state(
    rpc_client: Arc<solana_client::rpc_client::RpcClient>,
    pool_id: Option<&str>,
    mint: Option<&str>,
) -> Result<(Pubkey, AmmInfo)> {
    if let Some(pool_id) = pool_id {
        Ok((amm_pool_id, *pool_state))
    } else {
        Err(anyhow!("NotFoundPool: pool state not found"))
    }
}

pub async fn get_pool_state_by_mint(
    rpc_client: Arc<solana_client::rpc_client::RpcClient>,
    mint: &str,
) -> Result<(Pubkey, AmmInfo)> {
    // logger.log(format!("[FIND POOL STATE BY mint]: {}", mint));
    let pairs = vec![
        // pump pool
        (
            Some(spl_token::native_mint::ID),
            Pubkey::from_str(mint).ok(),
        ),
        // general pool
        (
            Pubkey::from_str(mint).ok(),
            Some(spl_token::native_mint::ID),
        ),
    ];

    let pool_len = core::mem::size_of::<AmmInfo>() as u64;
    let amm_program = Pubkey::from_str(AMM_PROGRAM)?;
    // Find matching AMM pool from mint pairs by filter
    let mut found_pools = None;
    for (coin_mint, pc_mint) in pairs {
        // logger.log(format!(
        //     "get_pool_state_by_mint filter: coin_mint: {:?}, pc_mint: {:?}",
        //     coin_mint, pc_mint
        // ));
        let filters = match (coin_mint, pc_mint) {
            (None, None) => Some(vec![RpcFilterType::DataSize(pool_len)]),
            (Some(coin_mint), None) => Some(vec![
                RpcFilterType::Memcmp(Memcmp::new_base58_encoded(400, &coin_mint.to_bytes())),
                RpcFilterType::DataSize(pool_len),
            ]),
            (None, Some(pc_mint)) => Some(vec![
                RpcFilterType::Memcmp(Memcmp::new_base58_encoded(432, &pc_mint.to_bytes())),
                RpcFilterType::DataSize(pool_len),
            ]),
            (Some(coin_mint), Some(pc_mint)) => Some(vec![
                RpcFilterType::Memcmp(Memcmp::new_base58_encoded(400, &coin_mint.to_bytes())),
                RpcFilterType::Memcmp(Memcmp::new_base58_encoded(432, &pc_mint.to_bytes())),
                RpcFilterType::DataSize(pool_len),
            ]),
        };
        let pools =
            common::rpc::get_program_accounts_with_filters(&rpc_client, amm_program, filters)
                .unwrap();
        if !pools.is_empty() {
            found_pools = Some(pools);
            break;
        }
    }

    match found_pools {
        Some(pools) => {
            let pool = &pools[0];
            let pool_state = AmmInfo::load_from_bytes(&pools[0].1.data)?;
            Ok((pool.0, *pool_state))
        }
        None => Err(anyhow!("NotFoundPool: pool state not found")),
    }
}

// get pool info
// https://api-v3.raydium.io/pools/info/mint?mint1=So11111111111111111111111111111111111111112&mint2=EzM2d8JVpzfhV7km3tUsR1U1S4xwkrPnWkM4QFeTpump&poolType=standard&poolSortField=default&sortType=desc&pageSize=10&page=1
pub async fn get_pool_info(mint1: &str, mint2: &str) -> Result<PoolData> {
    let client = reqwest::Client::new();

    let result = client
        .get("https://api-v3.raydium.io/pools/info/mint")
        .query(&[
            ("mint1", mint1),
            ("mint2", mint2),
            ("poolType", "standard"),
            ("poolSortField", "default"),
            ("sortType", "desc"),
            ("pageSize", "1"),
            ("page", "1"),
        ])
        .send()
        .await?
        .json::<PoolInfo>()
        .await
        .context("Failed to parse pool info JSON")?;
    Ok(result.data)
}
