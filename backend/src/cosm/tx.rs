// backend/src/cosm/tx.rs
use anyhow::Result;
use cosmrs::{Coin, AccountId};
use serde::Serialize;

use crate::util::config::Config;
use super::client;

pub struct CreatePollMsg {
    pub slug: String,
    pub title: String,
    pub options: Vec<String>,
    pub start: Option<i64>,
    pub end: i64,
    pub vote_price: String, // contoh "10000ujuno" atau "10000uusdc"
}

#[derive(Serialize)]
struct ExecCreatePoll<'a> {
    create_poll: ExecCreatePollData<'a>,
}

#[derive(Serialize)]
struct ExecCreatePollData<'a> {
    slug: &'a str,
    title: &'a str,
    options: &'a [String],
    start: Option<i64>,
    end: i64,
    vote_price: &'a str,
}

/// Kirim MsgExecuteContract ke CONTRACT_ADDR dengan msg JSON:
/// { "create_poll": { slug, title, options, start, end, vote_price } }
pub async fn create_poll_onchain(
    contract_addr: &str,
    msg: &CreatePollMsg,
) -> Result<String> {
    // Ambil env via Config yang sudah kamu punya
    let cfg = Config::from_env()?;

    let grpc = std::env::var("COSM_GRPC_URL")?;
    let chain_id = std::env::var("COSM_CHAIN_ID")?;
    let bech32 = std::env::var("COSM_BECH32_PREFIX")?;
    let mnemonic = std::env::var("COSM_SENDER_MNEMONIC")?;
    let fee_denom = std::env::var("COSM_FEE_DENOM")?;
    let fee_amount: u128 = std::env::var("COSM_FEE_AMOUNT")?.parse()?;
    let gas_limit: u64 = std::env::var("COSM_GAS_LIMIT")?.parse()?;
    let broadcast_mode = std::env::var("COSM_BROADCAST_MODE").unwrap_or_else(|_| "block".into());

    // Derive signer
    let (signer, sender_addr) = client::signer_from_mnemonic(&mnemonic, &bech32)?;

    // Query account meta
    let meta = client::query_account(&grpc, &sender_addr).await?;

    // Build exec msg
    let exec_msg = ExecCreatePoll {
        create_poll: ExecCreatePollData {
            slug: &msg.slug,
            title: &msg.title,
            options: &msg.options,
            start: msg.start,
            end: msg.end,
            vote_price: &msg.vote_price,
        },
    };
    let exec_json = serde_json::to_value(&exec_msg)?;

    // Build Any MsgExecuteContract
    let funds: Vec<Coin> = vec![]; // create_poll biasanya tanpa funds
    let any = client::build_execute_contract_any(
        &sender_addr,
        contract_addr,
        &exec_json,
        funds,
    )?;

    // Sign & broadcast
    let txhash = client::sign_and_broadcast(
        &grpc,
        &chain_id,
        &signer,
        &meta,
        &fee_denom,
        fee_amount,
        gas_limit,
        vec![any],
        "",              // memo
        &broadcast_mode, // "block" default
    ).await?;

    Ok(txhash)
}
