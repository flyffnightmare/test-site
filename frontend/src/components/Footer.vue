<template>
  <footer class="footer">
    <div class="container">
      <div class="footer-content">
        <div class="footer-logo">
          <img :src="logoUrl" alt="Game Company Logo" class="logo-img">
          <span class="logo-text">GameStudio</span>
        </div>
        <div class="footer-links">
          <router-link to="/">Главная</router-link>
          <router-link to="/games">Игры</router-link>
          <router-link to="/about">О компании</router-link>
          <router-link to="/contacts">Контакты</router-link>
        </div>
        <div class="footer-copy">
          &copy; 2024 GameStudio. Все права защищены.
        </div>
      </div>
    </div>
  </footer>
</template>

<script>
export default {
  name: 'Footer',
  data() {
    return {
      logoUrl: '/images/logo.png'
    }
  }
}
</script>

<style scoped>
.footer {
  background: rgba(0, 0, 0, 0.9);
  border-top: 1px solid #333;
  padding: 2rem 0;
  margin-top: auto;
}

.footer-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 1rem;
}

.footer-logo {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.footer-links {
  display: flex;
  gap: 1.5rem;
}

.footer-links a {
  color: #fff;
  text-decoration: none;
  transition: color 0.3s ease;
}

.footer-links a:hover {
  color: #667eea;
}

.footer-copy {
  color: #888;
}

@media (max-width: 768px) {
  .footer-content {
    flex-direction: column;
    text-align: center;
  }
}
</style>