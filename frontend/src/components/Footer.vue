<template>
  <footer class="footer">
    <div class="container">
      <div class="footer-main">
        <div class="footer-logo-section">
          <div class="footer-logo">
            <img :src="logoUrl" alt="Game Company Logo" class="logo-img">
            <span class="logo-text">GameStudio</span>
          </div>
          <p class="company-description">
            Создаем незабываемые игровые впечатления. Присоединяйтесь к нашему сообществу!
          </p>
        </div>
        
        <div class="footer-social">
          <h3>Следите за нами</h3>
          <div class="social-links">
            <a href="https://vk.com" target="_blank" class="social-link vk">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M15.07 2H8.93C3.33 2 2 3.33 2 8.93v6.14C2 20.67 3.33 22 8.93 22h6.14c5.6 0 6.93-1.33 6.93-6.93V8.93C22 3.33 20.67 2 15.07 2zm2.77 14.39c-.52 1.45-1.63 1.41-2.38.94-1.17-.73-1.67-1.2-3.46-1.2-1.79 0-2.29.47-3.46 1.2-.75.47-1.86.51-2.38-.94-.52-1.45.47-2.29 1.2-3.46 1.2-1.91 2.4-3.82 3.6-5.73.52-.83 1.04-1.67 1.56-2.5.52-.83 1.04-1.25 1.56-1.25.52 0 1.04.42 1.56 1.25.52.83 1.04 1.67 1.56 2.5 1.2 1.91 2.4 3.82 3.6 5.73.73 1.17 1.72 2.01 1.2 3.46z"/>
              </svg>
              VK
            </a>
            <a href="https://bsky.app" target="_blank" class="social-link bluesky">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-6h2v6zm4 0h-2v-6h2v6z"/>
              </svg>
              Bluesky
            </a>
            <a href="https://reddit.com" target="_blank" class="social-link reddit">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0zm5.01 4.74c1.69 0 2.83.73 3.18 1.97.21 1.05-.26 2.03-1.23 2.69.74.14 1.43.44 1.97.91.85-.79 1.41-1.97 1.41-3.34 0-2.39-1.97-4.33-4.33-4.33-2.39 0-4.33 1.94-4.33 4.33 0 .18.01.36.03.54.28-.22.6-.4.95-.54-.03-.18-.05-.36-.05-.54 0-2.39 1.94-4.33 4.33-4.33zm-9.98 0c2.39 0 4.33 1.94 4.33 4.33 0 .18-.02.36-.05.54.35.14.67.32.95.54.02-.18.03-.36.03-.54 0-2.39 1.94-4.33 4.33-4.33 2.39 0 4.33 1.94 4.33 4.33 0 1.37-.56 2.55-1.41 3.34.54-.47 1.23-.77 1.97-.91-.97-.66-1.44-1.64-1.23-2.69.35-1.24 1.49-1.97 3.18-1.97 1.72 0 3.12 1.4 3.12 3.12 0 1.72-1.4 3.12-3.12 3.12-.18 0-.36-.02-.54-.05-.22.28-.4.6-.54.95.18.03.36.05.54.05 2.39 0 4.33-1.94 4.33-4.33 0-2.39-1.94-4.33-4.33-4.33z"/>
              </svg>
              Reddit
            </a>
            <a href="https://rutube.ru" target="_blank" class="social-link rutube">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M20.8 8.6c-.2-1.5-.4-2.6-1-3.5-.6-.9-1.4-1.4-2.4-1.7-1.2-.3-2.8-.4-4.8-.4h-3.2c-2 0-3.6.1-4.8.4-1 .3-1.8.8-2.4 1.7-.6.9-.8 2-1 3.5C1 10.2 1 11.9 1 14v2c0 2.1.1 3.8.4 5.4.2 1.5.4 2.6 1 3.5.6.9 1.4 1.4 2.4 1.7 1.2.3 2.8.4 4.8.4h3.2c2 0 3.6-.1 4.8-.4 1-.3 1.8-.8 2.4-1.7.6-.9.8-2 1-3.5.3-1.6.4-3.3.4-5.4v-2c0-2.1-.1-3.8-.4-5.4zm-5.6 7.4l-5.2 3.1c-.4.2-.8.1-1-.2-.1-.1-.2-.3-.2-.5v-6.2c0-.4.3-.7.7-.7.1 0 .3.1.4.2l5.2 3.1c.3.2.4.5.3.8 0 .3-.2.5-.4.6z"/>
              </svg>
              Rutube
            </a>
          </div>
        </div>
      </div>
      
      <div class="footer-bottom">
        <div class="footer-copy">
          <p>&copy; 2024 GameStudio. Все права защищены.</p>
          <p>Все товарные знаки являются собственностью их правообладателей.</p>
        </div>
        <div class="footer-legal">
          <a href="#" class="legal-link">Условия использования</a>
          <a href="#" class="legal-link">Политика конфиденциальности</a>
          <a href="#" class="legal-link">Правила поведения</a>
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
  gap: 0.5rem;
  color: #aaa;
  text-decoration: none;
  transition: all 0.3s ease;
  padding: 0.5rem;
  border-radius: 6px;
  font-size: 0.9rem;
}

.social-link:hover {
  color: #fff;
  background: rgba(255, 255, 255, 0.1);
  transform: translateX(5px);
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