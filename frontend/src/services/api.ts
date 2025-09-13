/* # frontend/src/services/api.ts

*/
const API = import.meta.env.VITE_BACKEND_URL as string

async function http(path: string, init?: RequestInit) {
  const res = await fetch(`${API}${path}`, {
    headers: { 'Content-Type': 'application/json', ...(init?.headers || {}) },
    ...init,
  })
  if (!res.ok) throw new Error(await res.text())
  const ct = res.headers.get('content-type') || ''
  return ct.includes('application/json') ? res.json() : res.text()
}

export const Api = {
  login(email: string, password: string) {
    return http('/auth/login', { method: 'POST', body: JSON.stringify({ email, password }) })
  },
  register(email: string, password: string) {
    return http('/auth/register', { method: 'POST', body: JSON.stringify({ email, password }) })
  },
  listPublicPolls() {
    return http('/public/polls')
  },
  listAdminPolls(token: string) {
    return http('/admin/polls', { headers: { Authorization: `Bearer ${token}` } })
  },
  createPoll(token: string, payload: any) {
    return http('/admin/polls', { method: 'POST', body: JSON.stringify(payload), headers: { Authorization: `Bearer ${token}` } })
  },
  getPoll(token: string, id: string) {
    return http(`/admin/polls/${id}`, { headers: { Authorization: `Bearer ${token}` } })
  },
  updatePoll(token: string, id: string, payload: any) {
    return http(`/admin/polls/${id}`, { method: 'PATCH', body: JSON.stringify(payload), headers: { Authorization: `Bearer ${token}` } })
  },
  pushPollOnchain(token: string, id: string) {
    return http(`/admin/polls/${id}/push-onchain`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${token}` }
    })
  },
}
