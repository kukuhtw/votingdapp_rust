
import { createRouter, createWebHistory } from 'vue-router'

import Home from '../pages/Home.vue'
import PublicPollList from '../pages/PublicPollList.vue'
import PublicPollDetail from '../pages/PublicPollDetail.vue'
import AdminLogin from '../pages/AdminLogin.vue'
import AdminDashboard from '../pages/AdminDashboard.vue'
import AdminPollList from '../pages/AdminPollList.vue'
import AdminPollCreate from '../pages/AdminPollCreate.vue'
import AdminPollEdit from '../pages/AdminPollEdit.vue'
import AdminPollPublish from '../pages/AdminPollPublish.vue'

const routes = [
  { path: '/', component: Home },
  { path: '/polls', component: PublicPollList },
  { path: '/polls/:slug', component: PublicPollDetail, props: true },
  { path: '/admin/login', component: AdminLogin },
  { path: '/admin', component: AdminDashboard },
  { path: '/admin/polls', component: AdminPollList },
  { path: '/admin/polls/create', component: AdminPollCreate },
  { path: '/admin/polls/:id', component: AdminPollEdit, props: true },
  { path: '/admin/polls/:id/publish', component: AdminPollPublish, props: true },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
