<template>
  <footer class="footer">
    <div class="container">
      <div class="footer-main">
        <div class="footer-logo-section">
          <div class="footer-logo">
            <img :src="logoUrl" alt="Game Company Logo" class="logo-img">
            <span class="logo-text">SibWinterCraft</span>
          </div>
          <p class="company-description">
            Создаем незабываемые игровые впечатления. Присоединяйтесь к нашему сообществу!
          </p>
        </div>
        
        <div class="footer-social">
          <h3>Следите за нами</h3>
          <div class="social-links">
            <a href="https://vk.com" target="_blank" class="social-link vk">
              <img src="/images/social/vk.svg" alt="VK" class="social-icon">
              VK
            </a>
            <a href="https://bsky.app" target="_blank" class="social-link bluesky">
              <img src="/images/social/bluesky.svg" alt="Bluesky" class="social-icon">
              Bluesky
            </a>
            <a href="https://reddit.com" target="_blank" class="social-link reddit">
              <img src="/images/social/reddit.svg" alt="Reddit" class="social-icon">
              Reddit
            </a>
            <a href="https://rutube.ru" target="_blank" class="social-link rutube">
              <img src="/images/social/rutube.svg" alt="Rutube" class="social-icon">
              Rutube
            </a>
          </div>
        </div>
      </div>
      
      <div class="footer-bottom">
        <div class="footer-copy">
          <p>&copy; 2025 SibWinterCraft. Все права защищены.</p>
          <p>Все товарные знаки являются собственностью их правообладателей.</p>
        </div>
        <div class="footer-legal">
          <router-link to="/terms" class="legal-link">Условия использования</router-link>
          <router-link to="/privacy" class="legal-link">Политика конфиденциальности</router-link>
          <router-link to="/conduct" class="legal-link">Правила поведения</router-link>
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
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
  border-top: 1px solid #333;
  padding: 3rem 0 1.5rem;
  margin-top: auto;
}

.footer-main {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 3rem;
  align-items: start;
  margin-bottom: 2rem;
}

.footer-logo-section {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.footer-logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-img {
  height: 40px;
  width: auto;
  object-fit: contain;
}

.logo-text {
  font-size: 1.5rem;
  font-weight: bold;
  background: linear-gradient(45deg, #667eea, #764ba2);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.company-description {
  color: #aaa;
  font-size: 0.95rem;
  line-height: 1.5;
  max-width: 300px;
  margin: 0;
}

.footer-social h3 {
  color: #fff;
  font-size: 1rem;
  margin-bottom: 1rem;
  font-weight: 600;
}

.social-links {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.social-link {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  color: #aaa;
  text-decoration: none;
  transition: all 0.3s ease;
  padding: 0.5rem;
  border-radius: 6px;
  font-size: 0.9rem;
}

.social-icon {
  width: 20px;
  height: 20px;
  object-fit: contain;
  filter: brightness(0.8);
  transition: filter 0.3s ease;
}

.social-link:hover {
  color: #fff;
  background: rgba(255, 255, 255, 0.1);
  transform: translateX(5px);
}

.social-link:hover .social-icon {
  filter: brightness(1);
}

.social-link.vk:hover {
  color: #4a76a8;
}

.social-link.bluesky:hover {
  color: #0085ff;
}

.social-link.reddit:hover {
  color: #ff4500;
}

.social-link.rutube:hover {
  color: #ff0000;
}

.footer-bottom {
  border-top: 1px solid #333;
  padding-top: 1.5rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 1rem;
}

.footer-copy p {
  color: #888;
  font-size: 0.85rem;
  margin: 0.25rem 0;
  line-height: 1.4;
}

.footer-legal {
  display: flex;
  gap: 1.5rem;
}

.legal-link {
  color: #667eea;
  text-decoration: none;
  font-size: 0.85rem;
  transition: color 0.3s ease;
}

.legal-link:hover {
  color: #764ba2;
  text-decoration: underline;
}

@media (max-width: 768px) {
  .footer {
    padding: 2rem 0 1rem;
  }

  .footer-main {
    grid-template-columns: 1fr;
    gap: 2rem;
    text-align: center;
  }

  .footer-logo {
    justify-content: center;
  }

  .company-description {
    max-width: none;
    text-align: center;
  }

  .footer-social h3 {
    text-align: center;
  }

  .social-links {
    align-items: center;
  }

  .footer-bottom {
    flex-direction: column;
    text-align: center;
    gap: 1.5rem;
  }

  .footer-legal {
    justify-content: center;
    flex-wrap: wrap;
    gap: 1rem;
  }
}

@media (max-width: 480px) {
  .footer {
    padding: 1.5rem 0 1rem;
  }

  .logo-img {
    height: 32px;
  }

  .logo-text {
    font-size: 1.25rem;
  }

  .company-description {
    font-size: 0.9rem;
  }

  .social-links {
    gap: 0.5rem;
  }

  .social-link {
    font-size: 0.85rem;
    padding: 0.4rem;
  }

  .footer-legal {
    flex-direction: column;
    gap: 0.75rem;
  }
}
</style>