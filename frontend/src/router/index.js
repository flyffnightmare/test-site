import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Games from '../views/Games.vue'
import About from '../views/About.vue'
import Contacts from '../views/Contacts.vue'
import TermsOfUse from '../views/TermsOfUse.vue'
import PrivacyPolicy from '../views/PrivacyPolicy.vue'
import CodeOfConduct from '../views/CodeOfConduct.vue'
import GameDetails from '../views/GameDetails.vue'
import UserProfile from '../views/UserProfile.vue'
import AdminPanel from '../views/AdminPanel.vue'
import Login from '../views/Login.vue'
import { adminGuard } from './guards' // 👈 ТОЛЬКО adminGuard

const routes = [
  { 
    path: '/', 
    name: 'Home',
    component: Home 
  },
  { 
    path: '/games', 
    name: 'Games',
    component: Games 
  },
  { 
    path: '/about', 
    name: 'About',
    component: About 
  },
  { 
    path: '/contacts', 
    name: 'Contacts',
    component: Contacts 
  },
  { 
    path: '/terms', 
    name: 'TermsOfUse',
    component: TermsOfUse 
  },
  { 
    path: '/privacy', 
    name: 'PrivacyPolicy',
    component: PrivacyPolicy 
  },
  { 
    path: '/conduct', 
    name: 'CodeOfConduct',
    component: CodeOfConduct 
  },
  { 
    path: '/games/:id', 
    name: 'GameDetails',
    component: GameDetails 
  },
  { 
    path: '/profile', 
    name: 'UserProfile',
    component: UserProfile,
    // Убираем beforeEnter если не используем authGuard
  },
  {
    path: '/admin',
    name: 'AdminPanel',
    component: AdminPanel,
    beforeEnter: adminGuard
  },
  { 
    path: '/login', 
    name: 'Login',
    component: Login 
  },
  { 
    path: '/:pathMatch(.*)*', 
    name: 'NotFound',
    redirect: '/' 
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  console.log(`🔄 Навигация: ${from.path} → ${to.path}`)
  next()
})

export default router