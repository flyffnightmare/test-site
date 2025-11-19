<template>
  <header class="header">
    <div class="container">
      <nav class="nav">
        <div class="logo">
          <img :src="logoUrl" alt="Game Company Logo" class="logo-img">
          <span class="logo-text">GameStudio</span>
        </div>
        
        <ul class="nav-links">
          <li><router-link to="/">Главная</router-link></li>
          <li><router-link to="/games">Игры</router-link></li>
          <li><router-link to="/about">О компании</router-link></li>
          <li><router-link to="/contacts">Контакты</router-link></li>
        </ul>
        
        <div class="auth-buttons">
          <button v-if="!user" @click="$emit('open-login')" class="btn btn-secondary">Войти</button>
          <button v-if="!user" @click="$emit('open-register')" class="btn btn-primary">Регистрация</button>
          <div v-else class="user-menu">
            <span>Привет, {{ user.username }}!</span>
            <button @click="logout" class="btn btn-secondary">Выйти</button>
          </div>
        </div>
      </nav>
    </div>
  </header>
</template>

<script>
export default {
  name: 'Header',
  data() {
    return {
      logoUrl: '/images/logo.png', // Замените на свой логотип
      user: null
    }
  },
  methods: {
    logout() {
      this.user = null
      localStorage.removeItem('token')
    }
  },
  mounted() {
    // Проверка авторизации
    const token = localStorage.getItem('token')
    if (token) {
      // В реальном приложении здесь будет запрос для получения данных пользователя
      this.user = { username: 'User' }
    }
  }
}
</script>

<style scoped>
.header {
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid #333;
  position: sticky;
  top: 0;
  z-index: 1000;
}

.nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 0;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.logo-img {
  height: 40px;
  width: auto;
}

.logo-text {
  font-size: 1.5rem;
  font-weight: bold;
  background: linear-gradient(45deg, #667eea, #764ba2);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.nav-links {
  display: flex;
  list-style: none;
  gap: 2rem;
}

.nav-links a {
  color: #fff;
  text-decoration: none;
  transition: color 0.3s ease;
}

.nav-links a:hover,
.nav-links a.router-link-active {
  color: #667eea;
}

.auth-buttons {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.user-menu {
  display: flex;
  align-items: center;
  gap: 1rem;
}

@media (max-width: 768px) {
  .nav {
    flex-direction: column;
    gap: 1rem;
  }
  
  .nav-links {
    gap: 1rem;
  }
}
</style>