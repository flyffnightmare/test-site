<!-- views/Login.vue -->
<template>
  <div class="login-page">
    <div class="container">
      <div class="login-message">
        <h1>🔐 Авторизация</h1>
        <p>Для входа в систему используйте модальное окно на главной странице.</p>
        <div class="action-buttons">
          <router-link to="/" class="btn btn-primary">
            На главную
          </router-link>
          <button v-if="isAuthenticated" @click="goToAdmin" class="btn btn-secondary">
            В админ-панель
          </button>
        </div>
        
        <div v-if="debugInfo" class="debug-info">
          <h3>Отладочная информация:</h3>
          <p><strong>Токен:</strong> {{ debugInfo.token ? 'есть' : 'нет' }}</p>
          <p><strong>Пользователь:</strong> {{ debugInfo.user || 'не авторизован' }}</p>
          <p><strong>Роль:</strong> {{ debugInfo.role || 'не определена' }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Login',
  data() {
    return {
      debugInfo: null
    }
  },
  computed: {
    isAuthenticated() {
      return !!localStorage.getItem('auth_token')
    }
  },
  mounted() {
    this.loadDebugInfo()
    console.log('📍 Страница логина загружена')
  },
  methods: {
    loadDebugInfo() {
      const token = localStorage.getItem('auth_token')
      const userData = localStorage.getItem('user')
      
      this.debugInfo = {
        token: !!token,
        user: userData ? JSON.parse(userData).username : null,
        role: userData ? JSON.parse(userData).role : null
      }
    },
    
    goToAdmin() {
      this.$router.push('/admin')
    }
  }
}
</script>

<style scoped>
.login-page {
  min-height: 60vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem 0;
}

.login-message {
  text-align: center;
  background: rgba(255, 255, 255, 0.05);
  padding: 3rem;
  border-radius: 15px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  max-width: 500px;
  width: 100%;
}

.login-message h1 {
  background: linear-gradient(45deg, #00aeff, #a335ee);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  margin-bottom: 1rem;
  font-size: 2rem;
}

.login-message p {
  color: #aaa;
  margin-bottom: 2rem;
  font-size: 1.1rem;
  line-height: 1.5;
}

.action-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
  margin-bottom: 2rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  text-decoration: none;
  display: inline-block;
  transition: all 0.3s ease;
}

.btn-primary {
  background: linear-gradient(45deg, #00aeff, #a335ee);
  color: white;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 174, 255, 0.4);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.1);
  color: #e6e6e6;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.2);
}

.debug-info {
  margin-top: 2rem;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  text-align: left;
}

.debug-info h3 {
  color: #00aeff;
  margin-bottom: 1rem;
  font-size: 1.1rem;
}

.debug-info p {
  margin: 0.5rem 0;
  color: #b0b0b0;
  font-size: 0.9rem;
}

.debug-info strong {
  color: #e6e6e6;
}

@media (max-width: 768px) {
  .login-message {
    padding: 2rem;
    margin: 1rem;
  }
  
  .action-buttons {
    flex-direction: column;
  }
  
  .btn {
    width: 100%;
  }
}
</style>