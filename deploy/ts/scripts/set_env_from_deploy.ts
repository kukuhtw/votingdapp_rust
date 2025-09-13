// deploy/ts/scripts/set_env_from_deploy.ts

import * as fs from "fs"
import * as path from "path"

function main() {
  // Baca contract address dari artifacts/contract_address.txt
  const addrPath = path.join(__dirname, "../../..", "onchain/artifacts/contract_address.txt")
  if (!fs.existsSync(addrPath)) {
    throw new Error("❌ contract_address.txt not found, jalankan instantiate.ts dulu")
  }
  const contractAddr = fs.readFileSync(addrPath, "utf8").trim()
  console.log("✅ Contract address:", contractAddr)

  // ===== Update config/sandbox/contracts.env =====
  const contractsEnvPath = path.join(__dirname, "../../..", "config/sandbox/contracts.env")
  let contractsEnv = `VOTING_ADDR=${contractAddr}\n`
  // kalau ada CW20 juga bisa diset manual / dari argumen
  if (process.env.CW20_ADDR) {
    contractsEnv += `CW20_ADDR=${process.env.CW20_ADDR}\n`
  }
  fs.writeFileSync(contractsEnvPath, contractsEnv, "utf8")
  console.log("✍️ Updated:", contractsEnvPath)

  // ===== Update frontend/.env.sandbox =====
  const feEnvPath = path.join(__dirname, "../../..", "frontend/.env.sandbox")
  let feEnv = ""
  if (fs.existsSync(feEnvPath)) feEnv = fs.readFileSync(feEnvPath, "utf8")

  const feLines = feEnv.split(/\r?\n/).filter(l => !l.startsWith("VITE_VOTING_ADDR="))
  feLines.push(`VITE_VOTING_ADDR=${contractAddr}`)
  if (process.env.CW20_ADDR) {
    feLines.push(`VITE_CW20_ADDR=${process.env.CW20_ADDR}`)
  }
  fs.writeFileSync(feEnvPath, feLines.join("\n"), "utf8")
  console.log("✍️ Updated:", feEnvPath)

  // ===== Update backend/.env.sandbox =====
  const beEnvPath = path.join(__dirname, "../../..", "backend/.env.sandbox")
  let beEnv = ""
  if (fs.existsSync(beEnvPath)) beEnv = fs.readFileSync(beEnvPath, "utf8")

  const beLines = beEnv.split(/\r?\n/).filter(l => !l.startsWith("CONTRACT_ADDR="))
  beLines.push(`CONTRACT_ADDR=${contractAddr}`)
  fs.writeFileSync(beEnvPath, beLines.join("\n"), "utf8")
  console.log("✍️ Updated:", beEnvPath)

  console.log("✅ All env files updated")
}

main()
