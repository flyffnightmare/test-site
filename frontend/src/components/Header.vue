<template>
  <header class="header">
    <div class="header-container">
      <nav class="nav">
        <div class="nav-left">
          <div class="logo">
            <img :src="logoUrl" alt="Game Company Logo" class="logo-img">
            <span class="logo-text">GameStudio</span>
          </div>
        </div>
        
        <ul class="nav-links">
          <li><router-link to="/" class="nav-link">Главная</router-link></li>
          <li><router-link to="/games" class="nav-link">Игры</router-link></li>
          <li><router-link to="/about" class="nav-link">О компании</router-link></li>
          <li><router-link to="/contacts" class="nav-link">Контакты</router-link></li>
        </ul>
        
        <div class="nav-right">
          <div class="auth-buttons">
            <button v-if="!user" @click="$emit('open-login')" class="auth-btn login-btn">Войти</button>
            <button v-if="!user" @click="$emit('open-register')" class="auth-btn register-btn">Регистрация</button>
            <div v-else class="user-menu">
              <span class="username">Привет, {{ user.username }}!</span>
              <button @click="logout" class="auth-btn logout-btn">Выйти</button>
            </div>
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
      logoUrl: '/images/logo.png',
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
    const token = localStorage.getItem('token')
    if (token) {
      this.user = { username: 'User' }
    }
  }
}
</script>

<style scoped>
.header {
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
  border-bottom: 1px solid #333;
  position: sticky;
  top: 0;
  z-index: 1000;
  backdrop-filter: blur(10px);
}

.header-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1rem;
}

.nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 70px;
}

.nav-left {
  display: flex;
  align-items: center;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0;
}

.logo-img {
  height: 32px;
  width: auto;
  object-fit: contain;
}

.logo-text {
  font-size: 1.4rem;
  font-weight: 700;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  letter-spacing: -0.5px;
}

.nav-links {
  display: flex;
  list-style: none;
  gap: 2rem;
  margin: 0;
  padding: 0;
}

.nav-link {
  color: #e6e6e6;
  text-decoration: none;
  font-weight: 500;
  font-size: 0.95rem;
  padding: 0.5rem 0;
  position: relative;
  transition: color 0.3s ease;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.nav-link:hover {
  color: #00aeff;
}

.nav-link::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 0;
  height: 2px;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  transition: width 0.3s ease;
}

.nav-link:hover::after,
.nav-link.router-link-active::after {
  width: 100%;
}

.nav-link.router-link-active {
  color: #00aeff;
}

.nav-right {
  display: flex;
  align-items: center;
}

.auth-buttons {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.auth-btn {
  padding: 0.6rem 1.25rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 600;
  transition: all 0.3s ease;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.login-btn {
  background: transparent;
  color: #e6e6e6;
  border: 1px solid #444;
}

.login-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: #00aeff;
  color: #00aeff;
}

.register-btn {
  background: linear-gradient(45deg, #00aeff, #a335ee);
  color: white;
}

.register-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 174, 255, 0.3);
}

.user-menu {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.username {
  color: #e6e6e6;
  font-size: 0.9rem;
  font-weight: 500;
}

.logout-btn {
  background: transparent;
  color: #ff6b6b;
  border: 1px solid #ff6b6b;
  padding: 0.5rem 1rem;
}

.logout-btn:hover {
  background: rgba(255, 107, 107, 0.1);
}

@media (max-width: 768px) {
  .nav {
    height: 60px;
  }

  .nav-links {
    gap: 1rem;
  }

  .nav-link {
    font-size: 0.85rem;
  }

  .logo-text {
    font-size: 1.2rem;
  }

  .logo-img {
    height: 28px;
  }

  .auth-buttons {
    gap: 0.5rem;
  }

  .auth-btn {
    padding: 0.5rem 1rem;
    font-size: 0.8rem;
  }
}

@media (max-width: 640px) {
  .nav {
    flex-wrap: wrap;
    height: auto;
    padding: 1rem 0;
  }

  .nav-links {
    order: 3;
    width: 100%;
    justify-content: center;
    margin-top: 1rem;
    gap: 0.75rem;
  }

  .nav-link {
    font-size: 0.8rem;
  }

  .logo {
    gap: 0.5rem;
  }

  .logo-text {
    font-size: 1.1rem;
  }

  .logo-img {
    height: 24px;
  }
}
</style>