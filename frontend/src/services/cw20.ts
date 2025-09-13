
import { toBase64, toUtf8 } from '@cosmjs/encoding'
import { getSigner } from './chain'

export async function voteOnce(pollId: string, optionIndex: number, amount: string) {
  const { client, address } = await getSigner()
  const cw20 = import.meta.env.VITE_CW20_ADDR as string
  const voting = import.meta.env.VITE_VOTING_ADDR as string
  const hook = { vote: { poll_id: pollId, option_index: optionIndex } }
  const msg = toBase64(toUtf8(JSON.stringify(hook)))
  const res = await client.execute(address, cw20, { send: { contract: voting, amount, msg } }, 'auto')
  return res.transactionHash
}
