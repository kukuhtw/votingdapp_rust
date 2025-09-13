// deploy/ts/scripts/upload.ts
// Upload CosmWasm .wasm → dapat code_id, simpan ke artifacts/code_id.txt

import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { GasPrice } from "@cosmjs/stargate"
import * as fs from "fs"
import * as path from "path"

async function main() {
  // ===== ENV =====
  const RPC = process.env.RPC || "https://rpc.uni.juno.deuslabs.fi:443"
  const CHAIN_ID = process.env.CHAIN_ID || "uni-6"
  const MNEMONIC = process.env.MNEMONIC || "word1 word2 ... word24"
  const BECH32_PREFIX = process.env.BECH32_PREFIX || "juno"
  const GAS_PRICE = process.env.GAS_PRICE || "0.025ujuno"

  // WASM path bisa dari ENV atau argumen
  const wasmArg = process.argv[2] // contoh: `node upload.js ../../onchain/artifacts/voting_cw20.wasm`
  const WASM_PATH =
    process.env.WASM_PATH ||
    wasmArg ||
    path.join(__dirname, "../../..", "onchain/artifacts/voting_cw20.wasm")

  if (!fs.existsSync(WASM_PATH)) {
    throw new Error(`WASM file not found: ${WASM_PATH}`)
  }

  // ===== Wallet & Client =====
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(MNEMONIC, { prefix: BECH32_PREFIX })
  const [account] = await wallet.getAccounts()
  console.log("Uploader address:", account.address)

  const client = await SigningCosmWasmClient.connectWithSigner(
    RPC,
    wallet,
    { prefix: BECH32_PREFIX, gasPrice: GasPrice.fromString(GAS_PRICE) }
  )

  // ===== Upload =====
  const wasm = fs.readFileSync(WASM_PATH)
  console.log(`Uploading WASM: ${WASM_PATH} (${wasm.length} bytes) ...`)
  const uploadRes = await client.upload(account.address, wasm, "auto")

  console.log("✅ Upload OK")
  console.log("code_id:", uploadRes.codeId)
  if (uploadRes.transactionHash) console.log("tx:", uploadRes.transactionHash)

  // ===== Simpan hasil =====
  const artifactsDir = path.join(__dirname, "../../..", "onchain/artifacts")
  if (!fs.existsSync(artifactsDir)) fs.mkdirSync(artifactsDir, { recursive: true })

  const codeIdTxt = path.join(artifactsDir, "code_id.txt")
  fs.writeFileSync(codeIdTxt, String(uploadRes.codeId), "utf8")

  const uploadJson = path.join(artifactsDir, "upload_result.json")
  fs.writeFileSync(
    uploadJson,
    JSON.stringify({ codeId: uploadRes.codeId, tx: upl
