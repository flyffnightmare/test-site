<template>
  <div id="app">
    <Header 
      @open-login="showLoginModal = true" 
      @open-register="showRegisterModal = true" 
      ref="header"
    />
    <main>
      <router-view />
    </main>
    <Footer />
    
    <AuthModal 
      v-if="showLoginModal" 
      mode="login" 
      @close="showLoginModal = false"
      @switch-to-register="showLoginModal = false; showRegisterModal = true"
      @login-success="onLoginSuccess"
    />
    
    <AuthModal 
      v-if="showRegisterModal" 
      mode="register" 
      @close="showRegisterModal = false"
      @switch-to-login="showRegisterModal = false; showLoginModal = true"
    />
  </div>
</template>

<script>
import Header from './components/Header.vue'
import Footer from './components/Footer.vue'
import AuthModal from './components/AuthModal.vue'

export default {
  name: 'App',
  components: {
    Header,
    Footer,
    AuthModal
  },
  data() {
    return {
      showLoginModal: false,
      showRegisterModal: false
    }
  },
  methods: {
    onLoginSuccess(user) {
      // Обновляем данные пользователя в Header компоненте
      this.$refs.header.user = user
    }
  }
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Arial', sans-serif;
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
  color: #fff;
  min-height: 100vh;
}

#app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

main {
  flex: 1;
  padding: 2rem 0;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.3s ease;
}

.btn-primary {
  background: linear-gradient(45deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0,0,0,0.3);
}

.btn-secondary {
  background: transparent;
  border: 2px solid #667eea;
  color: #667eea;
}

.btn-secondary:hover {
  background: #667eea;
  color: white;
}
</style>