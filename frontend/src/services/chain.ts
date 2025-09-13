
import { SigningCosmWasmClient, CosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { GasPrice } from '@cosmjs/stargate'

export async function getSigner() {
  const chainId = import.meta.env.VITE_CHAIN_ID as string
  if (!window.keplr) throw new Error('Keplr not found')
  await window.keplr.enable(chainId)
  const offlineSigner = window.getOfflineSigner!(chainId)
  const [account] = await offlineSigner.getAccounts()
  const rpc = import.meta.env.VITE_RPC as string
  const gas = import.meta.env.VITE_GAS_PRICE as string
  const client = await SigningCosmWasmClient.connectWithSigner(rpc, offlineSigner, { gasPrice: GasPrice.fromString(gas) })
  return { client, address: account.address }
}

export async function readonlyClient() {
  const rpc = import.meta.env.VITE_RPC as string
  return CosmWasmClient.connect(rpc)
}

declare global {
  interface Window {
    keplr?: any;
    getOfflineSigner?: any;
  }
}
