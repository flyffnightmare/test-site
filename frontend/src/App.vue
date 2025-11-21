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
    
    <!-- Модальные окна -->
    <AuthModal 
      v-if="showLoginModal" 
      mode="login" 
      @close="showLoginModal = false"
      @switch-to-register="switchToRegister"
      @login-success="onLoginSuccess"
    />
    
    <AuthModal 
      v-if="showRegisterModal" 
      mode="register" 
      @close="showRegisterModal = false"
      @switch-to-login="switchToLogin"
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
      if (this.$refs.header) {
        this.$refs.header.user = user
      }
      
      if (user.role === 'admin') {
        console.log('👮 Администратор вошел в систему')
        setTimeout(() => {
          if (confirm(`🎮 Добро пожаловать, ${user.username}!\n\nВы вошли как администратор. Хотите перейти в админ-панель?`)) {
            this.$router.push('/admin')
          }
        }, 500)
      }
    },
    
    switchToRegister() {
      this.showLoginModal = false
      setTimeout(() => {
        this.showRegisterModal = true
      }, 300)
    },
    
    switchToLogin() {
      this.showRegisterModal = false
      setTimeout(() => {
        this.showLoginModal = true
      }, 300)
    }
  },
  watch: {
    showLoginModal(newVal) {
      this.toggleBodyScroll(!newVal)
    },
    showRegisterModal(newVal) {
      this.toggleBodyScroll(!newVal)
    }
  },
  methods: {
    toggleBodyScroll(enable) {
      if (enable) {
        document.body.classList.remove('modal-open')
      } else {
        document.body.classList.add('modal-open')
      }
    }
  }
}
</script>

<style>
body.modal-open {
  overflow: hidden;
}

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

/* Блокировка скролла когда модалка открыта */
body.modal-open {
  overflow: hidden;
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

.legal-page {
  padding: 2rem 0;
  min-height: calc(100vh - 200px);
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
}

.legal-header {
  text-align: center;
  margin-bottom: 3rem;
  padding-bottom: 2rem;
  border-bottom: 1px solid #333;
}

.legal-header h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.last-updated {
  color: #888;
  font-size: 0.9rem;
}

.legal-content {
  max-width: 800px;
  margin: 0 auto;
}

.legal-section {
  margin-bottom: 3rem;
  background: rgba(255, 255, 255, 0.05);
  padding: 2rem;
  border-radius: 10px;
  border: 1px solid #333;
}

.legal-section h2 {
  color: #00aeff;
  margin-bottom: 1.5rem;
  font-size: 1.5rem;
  border-bottom: 2px solid #00aeff;
  padding-bottom: 0.5rem;
}

.legal-section p {
  color: #e6e6e6;
  line-height: 1.6;
  margin-bottom: 1rem;
  font-size: 1rem;
}

.legal-section ul {
  color: #e6e6e6;
  padding-left: 1.5rem;
  margin-bottom: 1rem;
}

.legal-section li {
  margin-bottom: 0.5rem;
  line-height: 1.5;
}

.legal-section strong {
  color: #fff;
}

@media (max-width: 768px) {
  .legal-page {
    padding: 1rem 0;
  }
  
  .legal-header h1 {
    font-size: 2rem;
  }
  
  .legal-section {
    padding: 1.5rem;
    margin-bottom: 2rem;
  }
  
  .legal-section h2 {
    font-size: 1.3rem;
  }
}
</style>