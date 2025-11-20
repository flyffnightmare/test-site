import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Games from '../views/Games.vue'
import About from '../views/About.vue'
import Contacts from '../views/Contacts.vue'
import TermsOfUse from '../views/TermsOfUse.vue'
import PrivacyPolicy from '../views/PrivacyPolicy.vue'
import CodeOfConduct from '../views/CodeOfConduct.vue'
import GameDetails from '../views/GameDetails.vue'

const routes = [
  { path: '/', component: Home },
  { path: '/games', component: Games },
  { path: '/about', component: About },
  { path: '/contacts', component: Contacts },
  { path: '/terms', component: TermsOfUse },
  { path: '/privacy', component: PrivacyPolicy },
  { path: '/conduct', component: CodeOfConduct },
  { path: '/games/:id', component: GameDetails }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router