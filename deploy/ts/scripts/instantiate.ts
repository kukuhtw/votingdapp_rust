// deploy/ts/scripts/instantiate.ts

import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import * as fs from "fs"
import * as path from "path"

async function main() {
  // Ambil ENV dari config
  const rpc = process.env.RPC || "https://rpc.uni.juno.deuslabs.fi:443"
  const chainId = process.env.CHAIN_ID || "uni-6"
  const mnemonic = process.env.MNEMONIC || "word1 word2 ... word24"
  const bech32Prefix = process.env.BECH32_PREFIX || "juno"

  // Wallet signer
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix: bech32Prefix })
  const [account] = await wallet.getAccounts()
  console.log("Deploy from address:", account.address)

  // Client connect
  const client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet, {
    prefix: bech32Prefix,
    gasPrice: "0.025ujuno", // sesuaikan dengan testnet
  })

  // Ambil codeId dari file upload result (misal disimpan di artifacts/code_id.txt)
  const codeIdPath = path.join(__dirname, "../../artifacts/code_id.txt")
  if (!fs.existsSync(codeIdPath)) {
    throw new Error("code_id.txt not found, jalankan upload.ts dulu")
  }
  const codeId = parseInt(fs.readFileSync(codeIdPath, "utf8").trim(), 10)
  console.log("Using codeId:", codeId)

  // Instantiate Msg
  const initMsg = {
    // contoh schema instantiate untuk voting contract
    admin: account.address,
    cw20_token: process.env.CW20_ADDR || "",
    name: "Demo Voting DApp",
  }

  const label = "voting-contract-" + Date.now()

  const res = await client.instantiate(
    account.address,
    codeId,
    initMsg,
    label,
    "auto", // bisa juga: { amount: [{ denom: "ujuno", amount: "2500" }], gas: "250000" }
    { admin: account.address }
  )

  const contractAddr = res.contractAddress
  console.log("✅ Contract instantiated at:", contractAddr)

  // Simpan ke file artifacts/contract_address.txt
  const addrPath = path.join(__dirname, "../../artifacts/contract_address.txt")
  fs.writeFileSync(addrPath, contractAddr, "utf8")

  // Bisa langsung update ke config/contracts.env
  const contractsEnv = `VOTING_ADDR=${contractAddr}\n`
  fs.writeFileSync(path.join(__dirname, "../../config/sandbox/contracts.env"), contractsEnv, "utf8")
}

main().catch(err => {
  console.error(err)
  process.exit(1)
})
