use std::sync::Arc;

use crate::common::utils::AppState;
use crate::dex::pump::Pump;
use crate::dex::raydium::Raydium;
use anyhow::Result;
use clap::ValueEnum;
use jito_json_rpc_client::jsonrpc_client::rpc_client::RpcClient as JitoRpcClient;
use raydium_amm::state::AmmInfo;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use tokio::time::Instant;

#[derive(ValueEnum, Debug, Clone, Deserialize)]
pub enum SwapDirection {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl From<SwapDirection> for u8 {
    fn from(value: SwapDirection) -> Self {
        match value {
            SwapDirection::Buy => 0,
            SwapDirection::Sell => 1,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Deserialize)]
pub enum SwapInType {
    /// Quantity
    #[serde(rename = "qty")]
    Qty,
    /// Percentage
    #[serde(rename = "pct")]
    Pct,
}

pub async fn pump_swap(
    state: AppState,
    amount_in: u64,
    swap_direction: &str,
    slippage: u64,
    mint: &str,
    jito_client: Arc<JitoRpcClient>,
    timestamp: Instant,
) -> Result<Vec<String>> {
    let swap_direction = match swap_direction {
        "buy" => SwapDirection::Buy,
        "sell" => SwapDirection::Sell,
        _ => todo!(),
    };
    let in_type = "qty";
    let use_jito = true;
    let in_type = match in_type {
        "qty" => SwapInType::Qty,
        "pct" => SwapInType::Pct,
        _ => todo!(),
    };
    let swapx = Pump::new(state.rpc_nonblocking_client, state.rpc_client, state.wallet);
    println!("2.2: {:#?}", timestamp.elapsed());
    let res = match swapx
        .swap(
            mint,
            amount_in,
            swap_direction,
            slippage,
            jito_client.clone(),
            timestamp.clone(),
        )
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };
    Ok(res)
}

pub async fn raydium_swap(
    state: AppState,
    amount_in: u64,
    swap_direction: &str,
    pool_id: String,
    slippage: u64,
    mint: &str,
    jito_client: Arc<JitoRpcClient>,
    timestamp: Instant,
) -> Result<Vec<String>> {
    let swap_direction = match swap_direction {
        "buy" => SwapDirection::Buy,
        "sell" => SwapDirection::Sell,
        _ => todo!(),
    };

    let swapx = Raydium::new(state.rpc_nonblocking_client, state.rpc_client, state.wallet);
    println!("2.2: {:#?}", timestamp.elapsed());
    let res = match swapx
        .swap_by_mint(
            mint,
            swap_direction,
            amount_in,
            pool_id,
            slippage,
            timestamp.clone(),
            jito_client.clone(),
        )
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };
    Ok(res)
}
