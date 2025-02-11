use std::{future::Future, str::FromStr, sync::LazyLock, time::Duration};

use anyhow::{anyhow, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{seq::IteratorRandom, thread_rng};
use serde::Deserialize;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use tokio::{
    sync::RwLock,
    time::{sleep, Instant},
};

use crate::common::utils::{import_env_var, log_message};

pub static BLOCK_ENGINE_URL: LazyLock<String> =
    LazyLock::new(|| import_env_var("JITO_BLOCK_ENGINE_URL"));
pub static TIP_STREAM_URL: LazyLock<String> =
    LazyLock::new(|| import_env_var("JITO_TIP_STREAM_URL"));
pub static TIP_PERCENTILE: LazyLock<String> =
    LazyLock::new(|| import_env_var("JITO_TIP_PERCENTILE"));

pub static TIP_ACCOUNTS: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(vec![]));

#[derive(Debug)]
pub struct TipAccountResult {
    pub accounts: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct BundleStatus {
    pub bundle_id: String,
    pub transactions: Vec<String>,
    pub slot: u64,
    pub confirmation_status: String,
    pub err: ErrorStatus,
}
#[derive(Deserialize, Debug)]
pub struct ErrorStatus {
    #[serde(rename = "Ok")]
    pub ok: Option<()>,
}

pub fn new_progress_bar() -> ProgressBar {
    let progress_bar = ProgressBar::new(42);
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {wide_msg}")
            .expect("ProgressStyle::template direct input to be correct"),
    );
    progress_bar.enable_steady_tick(Duration::from_millis(100));
    progress_bar
}
