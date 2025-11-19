import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Games from '../views/Games.vue'
import About from '../views/About.vue'
import Contacts from '../views/Contacts.vue'

const routes = [
  { path: '/', component: Home },
  { path: '/games', component: Games },
  { path: '/about', component: About },
  { path: '/contacts', component: Contacts }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router