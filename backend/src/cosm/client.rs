/*
backend/src/cosm/client.rs
*/
use anyhow::{anyhow, Result};
use cosmrs::{
    proto,
    bip32::{DerivationPath, Mnemonic},
    crypto::secp256k1::SigningKey,
    tx::{self, SignDoc},
    AccountId, Coin,
};
use prost::Message;
use serde_json::Value;
use tonic::transport::Channel;

/// Data akun (untuk number/sequence)
pub struct AccountMeta {
    pub account_number: u64,
    pub sequence: u64,
    pub address: AccountId,
}

/// Ambil SigningKey & AccountId dari mnemonic
pub fn signer_from_mnemonic(mnemonic: &str, bech32_prefix: &str) -> Result<(SigningKey, AccountId)> {
    let m = Mnemonic::new(mnemonic, cosmrs::bip32::Language::English)?;
    // path default Cosmos: m/44'/118'/0'/0/0
    let path = DerivationPath::from_str("m/44'/118'/0'/0/0")?;
    let sk = SigningKey::derive_from_path(&m, &path)?;
    let pk = sk.public_key();
    let addr = AccountId::new(bech32_prefix, &pk)?;
    Ok((sk, addr))
}

fn from_str(path: &str) -> Result<DerivationPath> {
    DerivationPath::from_str(path).map_err(|e| anyhow!(e))
}

/// Query account_number & sequence dari gRPC
pub async fn query_account(grpc_url: &str, addr: &AccountId) -> Result<AccountMeta> {
    let channel = Channel::from_shared(grpc_url.to_string())?
        .connect()
        .await?;
    let mut client = proto::cosmos::auth::v1beta1::query_client::QueryClient::new(channel);

    let req = proto::cosmos::auth::v1beta1::QueryAccountRequest {
        address: addr.to_string(),
    };
    let res = client.account(req).await?.into_inner();

    // parse BaseAccount
    let any = res.account.ok_or_else(|| anyhow!("account not found"))?;
    // Any -> BaseAccount
    let base = proto::cosmos::auth::v1beta1::BaseAccount::decode(any.value.as_slice())
        .map_err(|_| anyhow!("failed to decode BaseAccount"))?;

    Ok(AccountMeta {
        account_number: base.account_number,
        sequence: base.sequence,
        address: addr.clone(),
    })
}

/// Build MsgExecuteContract (protobuf Any)
pub fn build_execute_contract_any(
    sender: &AccountId,
    contract: &str,
    msg_json: &serde_json::Value,
    funds: Vec<Coin>,
) -> Result<prost_types::Any> {
    // protobuf type
    let msg = proto::cosmwasm::wasm::v1::MsgExecuteContract {
        sender: sender.to_string(),
        contract: contract.to_string(),
        msg: serde_json::to_vec(msg_json)?,
        funds: funds
            .into_iter()
            .map(|c| proto::cosmos::base::v1beta1::Coin {
                denom: c.denom,
                amount: c.amount.to_string(),
            })
            .collect(),
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf)?;
    Ok(prost_types::Any {
        type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".into(),
        value: buf,
    })
}

/// Sign & broadcast TX
pub async fn sign_and_broadcast(
    grpc_url: &str,
    chain_id: &str,
    signer: &SigningKey,
    account: &AccountMeta,
    fee_denom: &str,
    fee_amount: u128,
    gas_limit: u64,
    msgs: Vec<prost_types::Any>,
    memo: &str,
    broadcast_mode: &str, // "sync" | "async" | "block"
) -> Result<String> {
    // Body
    let body = tx::Body::new(msgs, memo, 0u32);
    let body_bytes = body.to_bytes()?;

    // AuthInfo
    let pk = signer.public_key();
    let signer_info = tx::SignerInfo::single_direct(pk, account.sequence);
    let fee = tx::Fee::from_amount_and_gas(Coin {
        denom: fee_denom.to_string(),
        amount: fee_amount,
    }, gas_limit);
    let auth_info = tx::AuthInfo::new(vec![signer_info], fee);
    let auth_info_bytes = auth_info.to_bytes()?;

    // SignDoc
    let sign_doc = SignDoc::new(&body_bytes, &auth_info_bytes, chain_id.parse()?, account.account_number)?;
    let tx_raw = sign_doc.sign(signer)?;

    // gRPC broadcast
    let channel = Channel::from_shared(grpc_url.to_string())?
        .connect()
        .await?;
    let mut tx_client = proto::cosmos::tx::v1beta1::service_client::ServiceClient::new(channel);

    use proto::cosmos::tx::v1beta1::{BroadcastTxRequest, BroadcastMode};
    let mode = match broadcast_mode {
        "async" => BroadcastMode::Async,
        "sync" => BroadcastMode::Sync,
        _ => BroadcastMode::Block,
    } as i32;

    let req = BroadcastTxRequest {
        tx_bytes: tx_raw.to_bytes()?,
        mode,
    };
    let resp = tx_client.broadcast_tx(req).await?.into_inner();

    if let Some(tx_response) = resp.tx_response {
        // code == 0 -> success
        if tx_response.code == 0 {
            Ok(tx_response.txhash)
        } else {
            Err(anyhow!(
                "broadcast failed (code={}): {}",
                tx_response.code,
                tx_response.raw_log
            ))
        }
    } else {
        Err(anyhow!("empty tx_response"))
    }
}

/// (Opsional) Smart query placeholder via REST – tetap biarkan stub
pub async fn smart_query(_rest_url: &str, _contract_addr: &str, _msg: &serde_json::Value) -> Result<Value> {
    Ok(serde_json::json!({"mock":"ok"}))
}
