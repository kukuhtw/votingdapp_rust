
import { reactive } from 'vue'

const state = reactive({
  token: localStorage.getItem('jwt') || ''
})

export function useAuth() {
  function setToken(t: string) {
    state.token = t
    localStorage.setItem('jwt', t)
  }
  function clear() {
    state.token = ''
    localStorage.removeItem('jwt')
  }
  return { state, setToken, clear }
}
