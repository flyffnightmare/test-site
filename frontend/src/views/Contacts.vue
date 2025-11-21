<template>
  <div class="contacts">
    <!-- Hero секция -->
    <section class="contacts-hero">
      <div class="hero-background"></div>
      <div class="container">
        <div class="hero-content">
          <h1>Контакты</h1>
          <p class="hero-subtitle">Свяжитесь с командой SibWinterCraft</p>
        </div>
      </div>
    </section>

    <!-- Основной контент -->
    <div class="container">
      <div class="contacts-content">
        <!-- Карточки контактов -->
        <section class="contacts-grid-section">
          <h2>Наши контакты</h2>
          <p class="section-subtitle">Выберите подходящий способ связи</p>
          
          <div class="contacts-grid">
            <!-- Техподдержка -->
            <div class="contact-card support-card">
              <div class="card-header">
                <div class="card-icon">⚡</div>
                <h3>Техническая поддержка</h3>
                <div class="card-badge">Быстрый ответ</div>
              </div>
              <div class="card-content">
                <p>Возникли проблемы с игрой, аккаунтом или платежами? Наша команда поддержки всегда готова помочь!</p>
                <div class="contact-details">
                  <div class="contact-item">
                    <span class="contact-label">Email:</span>
                    <a href="mailto:support@sibwintercraft.com" class="contact-value">
                      support@sibwintercraft.com
                    </a>
                  </div>
                  <div class="contact-item">
                    <span class="contact-label">Время ответа:</span>
                    <span class="contact-value highlight">24-48 часов</span>
                  </div>
                  <div class="contact-item">
                    <span class="contact-label">Решаем:</span>
                    <span class="contact-value">Технические проблемы, баги, аккаунты</span>
                  </div>
                </div>
              </div>
              <div class="card-actions">
                <a href="mailto:support@sibwintercraft.com" class="btn btn-primary">
                  📧 Написать в поддержку
                </a>
              </div>
            </div>

            <!-- Общие вопросы -->
            <div class="contact-card info-card">
              <div class="card-header">
                <div class="card-icon">💼</div>
                <h3>Общие вопросы</h3>
                <div class="card-badge">Партнерства</div>
              </div>
              <div class="card-content">
                <p>Есть предложения о сотрудничестве, вопросы по издательству или общие вопросы о компании?</p>
                <div class="contact-details">
                  <div class="contact-item">
                    <span class="contact-label">Email:</span>
                    <a href="mailto:info@sibwintercraft.com" class="contact-value">
                      info@sibwintercraft.com
                    </a>
                  </div>
                  <div class="contact-item">
                    <span class="contact-label">Время ответа:</span>
                    <span class="contact-value">2-3 рабочих дня</span>
                  </div>
                  <div class="contact-item">
                    <span class="contact-label">Обсуждаем:</span>
                    <span class="contact-value">Сотрудничество, идеи, партнерства</span>
                  </div>
                </div>
              </div>
              <div class="card-actions">
                <a href="mailto:info@sibwintercraft.com" class="btn btn-secondary">
                  ✉️ Отправить предложение
                </a>
              </div>
            </div>

            <!-- Сообщество -->
            <div class="contact-card community-card">
              <div class="card-header">
                <div class="card-icon">👥</div>
                <h3>Сообщество</h3>
                <div class="card-badge">Активно</div>
              </div>
              <div class="card-content">
                <p>Присоединяйтесь к нашему сообществу, общайтесь с другими игроками и разработчиками!</p>
                <div class="social-links">
                  <a href="https://vk.com" target="_blank" class="social-link vk">
                    <span class="social-icon">🔷</span>
                    VK Group
                  </a>
                  <a href="https://discord.com" target="_blank" class="social-link discord">
                    <span class="social-icon">🎮</span>
                    Discord
                  </a>
                  <a href="https://reddit.com" target="_blank" class="social-link reddit">
                    <span class="social-icon">🔴</span>
                    Reddit
                  </a>
                </div>
              </div>
              <div class="card-actions">
                <a href="https://vk.com" target="_blank" class="btn btn-outline">
                  🎯 Присоединиться
                </a>
              </div>
            </div>
          </div>
        </section>

        <!-- Информация о компании -->
        <section class="company-info">
          <div class="info-grid">
            <div class="info-card">
              <h3>📍 Наш офис</h3>
              <div class="info-content">
                <p>г. Красноярск, Северное шоссе, 48</p>
                <p>Красноярский край, 660118</p>
                <p class="info-note">Сердце Сибири - источник нашего вдохновения</p>
              </div>
            </div>
            
            <div class="info-card">
              <h3>🕒 Часы работы</h3>
              <div class="info-content">
                <p><strong>Поддержка:</strong> Пн-Пт, 10:00-19:00</p>
                <p><strong>Разработка:</strong> Круглосуточно 🎮</p>
                <p class="info-note">Мы работаем, пока вы играете!</p>
              </div>
            </div>
            
            <div class="info-card">
              <h3>🎯 Наша миссия</h3>
              <div class="info-content">
                <p>Создавать игры, которые запомнятся надолго. Каждый проект - это новая история, каждый игрок - часть нашей большой семьи.</p>
                <p class="info-note">From Siberia with Love ❄️</p>
              </div>
            </div>
          </div>
        </section>

        <!-- FAQ -->
        <section class="faq-section">
          <h2>Частые вопросы</h2>
          <div class="faq-grid">
            <div class="faq-item">
              <h4>📋 Как сообщить о баге в игре?</h4>
              <p>Напишите на support@sibwintercraft.com с подробным описанием проблемы, приложите скриншоты и укажите вашу платформу.</p>
            </div>
            <div class="faq-item">
              <h4>💡 У меня есть идея для игры</h4>
              <p>Отлично! Присылайте ваши идеи на info@sibwintercraft.com. Мы всегда открыты для творческих предложений.</p>
            </div>
            <div class="faq-item">
              <h4>🤝 Хочу сотрудничать с вами</h4>
              <p>Для обсуждения партнерства и сотрудничества пишите на info@sibwintercraft.com с темой "Сотрудничество".</p>
            </div>
            <div class="faq-item">
              <h4>⏰ Сколько ждать ответа от поддержки?</h4>
              <p>Обычно мы отвечаем в течение 24-48 часов. В пиковые периоды время ответа может увеличиться до 72 часов.</p>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Contacts'
}
</script>

<style scoped>
.contacts {
  min-height: 100vh;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
}

/* Hero секция */
.contacts-hero {
  position: relative;
  height: 50vh;
  min-height: 400px;
  display: flex;
  align-items: center;
  overflow: hidden;
}

.hero-background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(
      135deg,
      rgba(10, 10, 10, 0.8) 0%,
      rgba(26, 26, 26, 0.9) 100%
    ),
    url('/images/contacts/hero-bg.jpg');
  background-size: cover;
  background-position: center;
  filter: brightness(0.7) blur(2px);
}

.hero-content {
  position: relative;
  z-index: 2;
  text-align: center;
  color: white;
}

.hero-content h1 {
  font-size: 3.5rem;
  margin-bottom: 1rem;
  background: linear-gradient(45deg, #ffffff, #00aeff);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
}

.hero-subtitle {
  font-size: 1.4rem;
  color: #e6e6e6;
  opacity: 0.9;
}

/* Основной контент */
.contacts-content {
  padding: 4rem 0;
}

/* Заголовки секций */
.contacts-grid-section h2,
.faq-section h2 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  color: #00aeff;
  text-align: center;
}

.section-subtitle {
  text-align: center;
  color: #aaa;
  font-size: 1.2rem;
  margin-bottom: 3rem;
}

/* Сетка контактов */
.contacts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 2rem;
  margin-bottom: 5rem;
}

.contact-card {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 20px;
  padding: 2rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
  position: relative;
  overflow: hidden;
}

.contact-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(45deg, #00aeff, #a335ee);
}

.contact-card:hover {
  transform: translateY(-10px);
  border-color: rgba(0, 174, 255, 0.3);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
}

/* Специфичные стили для карточек */
.support-card::before {
  background: linear-gradient(45deg, #00ff88, #00aeff);
}

.info-card::before {
  background: linear-gradient(45deg, #a335ee, #ff6b6b);
}

.community-card::before {
  background: linear-gradient(45deg, #ffd700, #ff6b6b);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.card-icon {
  font-size: 2.5rem;
}

.card-header h3 {
  color: #fff;
  font-size: 1.5rem;
  margin: 0;
  flex: 1;
}

.card-badge {
  background: rgba(0, 174, 255, 0.2);
  color: #00aeff;
  padding: 0.3rem 0.8rem;
  border-radius: 15px;
  font-size: 0.8rem;
  font-weight: 600;
  border: 1px solid rgba(0, 174, 255, 0.3);
}

.card-content p {
  color: #b0b0b0;
  line-height: 1.6;
  margin-bottom: 1.5rem;
}

.contact-details {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.contact-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

.contact-label {
  color: #888;
  font-size: 0.9rem;
  font-weight: 600;
  min-width: 100px;
}

.contact-value {
  color: #e6e6e6;
  text-align: right;
  flex: 1;
}

.contact-value.highlight {
  color: #00ff88;
  font-weight: 600;
}

.contact-value a {
  color: #00aeff;
  text-decoration: none;
  transition: color 0.3s ease;
}

.contact-value a:hover {
  color: #a335ee;
  text-decoration: underline;
}

/* Социальные ссылки */
.social-links {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}

.social-link {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  padding: 0.8rem;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  color: #e6e6e6;
  text-decoration: none;
  transition: all 0.3s ease;
  border: 1px solid transparent;
}

.social-link:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(0, 174, 255, 0.3);
  transform: translateX(5px);
}

.social-icon {
  font-size: 1.2rem;
}

/* Кнопки действий */
.card-actions {
  margin-top: 2rem;
  text-align: center;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.8rem 1.5rem;
  border-radius: 10px;
  text-decoration: none;
  font-weight: 600;
  transition: all 0.3s ease;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
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
  background: linear-gradient(45deg, #a335ee, #ff6b6b);
  color: white;
}

.btn-secondary:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(163, 53, 238, 0.4);
}

.btn-outline {
  background: transparent;
  color: #00aeff;
  border: 2px solid #00aeff;
}

.btn-outline:hover {
  background: rgba(0, 174, 255, 0.1);
  transform: translateY(-2px);
}

/* Информация о компании */
.company-info {
  margin-bottom: 5rem;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
}

.info-card {
  background: rgba(255, 255, 255, 0.03);
  padding: 2rem;
  border-radius: 15px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.info-card h3 {
  color: #00aeff;
  margin-bottom: 1rem;
  font-size: 1.3rem;
}

.info-content p {
  color: #e6e6e6;
  line-height: 1.6;
  margin-bottom: 0.8rem;
}

.info-note {
  color: #888 !important;
  font-style: italic;
  font-size: 0.9rem;
  margin-top: 1rem !important;
}

/* FAQ */
.faq-section {
  margin-bottom: 3rem;
}

.faq-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.faq-item {
  background: rgba(255, 255, 255, 0.03);
  padding: 1.5rem;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
}

.faq-item:hover {
  border-color: rgba(0, 174, 255, 0.3);
  transform: translateY(-2px);
}

.faq-item h4 {
  color: #00aeff;
  margin-bottom: 1rem;
  font-size: 1.1rem;
}

.faq-item p {
  color: #b0b0b0;
  line-height: 1.6;
  margin: 0;
}

/* Адаптивность */
@media (max-width: 768px) {
  .contacts-hero {
    height: 40vh;
    min-height: 300px;
  }
  
  .hero-content h1 {
    font-size: 2.5rem;
  }
  
  .contacts-content {
    padding: 2rem 0;
  }
  
  .contacts-grid {
    grid-template-columns: 1fr;
  }
  
  .contact-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .contact-value {
    text-align: left;
  }
  
  .info-grid,
  .faq-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 480px) {
  .hero-content h1 {
    font-size: 2rem;
  }
  
  .contacts-grid-section h2,
  .faq-section h2 {
    font-size: 2rem;
  }
  
  .contact-card {
    padding: 1.5rem;
  }
  
  .card-header {
    flex-direction: column;
    text-align: center;
    gap: 0.5rem;
  }
}
</style>