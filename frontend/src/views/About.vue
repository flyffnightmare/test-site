<template>
  <div class="about">
    <!-- Hero секция -->
    <section class="about-hero">
      <div class="hero-background"></div>
      <div class="container">
        <div class="hero-content">
          <h1>О компании SibWinterCraft</h1>
          <p class="hero-subtitle">Создаем игры с душой Сибири</p>
        </div>
      </div>
    </section>

    <!-- Основной контент -->
    <div class="container">
      <div class="about-content">
        <!-- О компании -->
        <section class="about-section">
          <div class="section-content">
            <div class="text-content">
              <h2>Наша история</h2>
              <p>Основанная в Красноярске, SibWinterCraft объединила талантливых разработчиков, вдохновленных суровой красотой сибирской природы и богатым культурным наследием нашего региона.</p>
              <p>Мы верим, что настоящие игры рождаются там, где сталкиваются традиции и инновации, где каждая снежинка рассказывает свою историю, а каждый лесной шепот становится частью игрового мира.</p>
              <p>Наша студия расположена в самом сердце Сибири, что дает нам уникальное видение и подход к созданию игровых вселенных, наполненных глубиной и атмосферностью.</p>
            </div>
            <div class="image-content">
              <img src="/images/about/krasnoyarsk-1.jpg" alt="Красноярск зимой" class="about-image">
            </div>
          </div>
        </section>

        <!-- Команда -->
        <section class="team-section">
          <div class="section-content reversed">
            <div class="text-content">
              <h2>Наша команда</h2>
              <p>В SibWinterCraft работают passionate разработчики, художники и дизайнеры из Красноярска и других городов Сибири. Мы объединены любовью к играм и желанием создавать нечто особенное.</p>
              <p>Наша сила - в разнообразии талантов и единстве vision. От программирования до нарративного дизайна, каждый член команды вносит свой уникальный вклад в общее дело.</p>
              <p>Мы гордимся тем, что создаем игры мирового уровня, оставаясь верными нашим сибирским корням и ценностям.</p>
            </div>
            <div class="image-content">
              <img src="/images/about/team.jpg" alt="Команда разработчиков" class="about-image">
            </div>
          </div>
        </section>

        <!-- Технологии -->
        <section class="tech-section">
          <h2>Технологии и подход</h2>
          <div class="tech-grid">
            <div class="tech-item">
              <h3>Современные движки</h3>
              <p>Используем передовые игровые технологии для создания immersive геймплея и stunning визуальных эффектов</p>
            </div>
            <div class="tech-item">
              <h3>Player-first дизайн</h3>
              <p>Ставим опыт игрока на первое место, создавая интуитивные интерфейсы и сбалансированный геймплей</p>
            </div>
            <div class="tech-item">
              <h3>Постоянное развитие</h3>
              <p>Непрерывно учимся и внедряем новые методики разработки, чтобы наши игры оставались актуальными</p>
            </div>
          </div>
        </section>

        <!-- Контакты -->
        <section class="contact-section">
          <div class="contact-card">
            <h2>Свяжитесь с нами</h2>
            <div class="contact-content">
              <div class="contact-info">
                <div class="contact-item">
                  <strong>📍 Адрес:</strong>
                  <span>г. Красноярск, Северное шоссе, 48</span>
                </div>
                <div class="contact-item">
                  <strong>📧 Email:</strong>
                  <span>info@sibwintercraft.com</span>
                </div>
                <div class="contact-item">
                  <strong>🌐 Сайт:</strong>
                  <span>www.sibwintercraft.com</span>
                </div>
              </div>
              <div class="location-map">
                <iframe 
                  src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2374.0342547090104!2d92.88346850080109!3d56.063715792404594!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x5cd7aec36ada39e7%3A0x85ad990864d9c9e8!2z0K_RgdC90YvQuSwg0JbQuNC70L7QuSDQmtC-0LzQv9C70LXQutGB!5e0!3m2!1sru!2sru!4v1763691343742!5m2!1sru!2sru"
                  width="100%" 
                  height="300" 
                  style="border:0; border-radius: 10px;" 
                  allowfullscreen="" 
                  loading="lazy" 
                  referrerpolicy="no-referrer-when-downgrade"
                  title="Карта с расположением SibWinterCraft на Северном шоссе 48">
                </iframe>
                <p class="map-caption">Наш офис расположен в Красноярске на Северном шоссе</p>
              </div>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'About'
}
</script>

<style scoped>
.about {
  min-height: 100vh;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
}

/* Hero секция */
.about-hero {
  position: relative;
  height: 60vh;
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
    url('/images/about/hero-bg.jpg');
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
.about-content {
  padding: 4rem 0;
}

/* Секции с контентом */
.about-section,
.team-section {
  margin-bottom: 6rem;
}

.section-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
}

.section-content.reversed {
  grid-template-columns: 1fr 1fr;
  direction: rtl;
}

.section-content.reversed > .text-content {
  direction: ltr;
}

.section-content.reversed > .image-content {
  direction: ltr;
}

.text-content h2 {
  font-size: 2.5rem;
  margin-bottom: 1.5rem;
  color: #00aeff;
  border-bottom: 2px solid #00aeff;
  padding-bottom: 0.5rem;
  display: inline-block;
}

.text-content p {
  color: #e6e6e6;
  line-height: 1.8;
  margin-bottom: 1.5rem;
  font-size: 1.1rem;
}

.about-image {
  width: 100%;
  height: 400px;
  object-fit: cover;
  border-radius: 15px;
  border: 2px solid rgba(0, 174, 255, 0.3);
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
  transition: transform 0.3s ease;
}

.about-image:hover {
  transform: scale(1.02);
}

/* Технологии */
.tech-section {
  margin-bottom: 6rem;
}

.tech-section h2 {
  font-size: 2.5rem;
  margin-bottom: 3rem;
  color: #00aeff;
  text-align: center;
  border-bottom: 2px solid #00aeff;
  padding-bottom: 0.5rem;
  display: inline-block;
}

.tech-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
}

.tech-item {
  background: rgba(255, 255, 255, 0.05);
  padding: 2rem;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
}

.tech-item:hover {
  transform: translateY(-5px);
  border-color: rgba(0, 174, 255, 0.3);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
}

.tech-item h3 {
  color: #00aeff;
  margin-bottom: 1rem;
  font-size: 1.3rem;
}

.tech-item p {
  color: #b0b0b0;
  line-height: 1.6;
}

/* Контакты */
.contact-section {
  margin-bottom: 4rem;
}

.contact-card {
  background: rgba(255, 255, 255, 0.05);
  padding: 3rem;
  border-radius: 15px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
}

.contact-card h2 {
  font-size: 2.2rem;
  margin-bottom: 2rem;
  color: #00aeff;
  text-align: center;
}

.contact-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 3rem;
  align-items: start;
}

.contact-info {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.contact-item {
  background: rgba(255, 255, 255, 0.03);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.contact-item strong {
  color: #00aeff;
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.contact-item span {
  color: #e6e6e6;
  font-size: 1.1rem;
  font-weight: 600;
}

.location-map {
  position: relative;
}

.map-caption {
  text-align: center;
  color: #b0b0b0;
  margin-top: 1rem;
  font-size: 0.9rem;
}

/* Адаптивность */
@media (max-width: 768px) {
  .about-hero {
    height: 50vh;
    min-height: 300px;
  }
  
  .hero-content h1 {
    font-size: 2.5rem;
  }
  
  .hero-subtitle {
    font-size: 1.2rem;
  }
  
  .about-content {
    padding: 2rem 0;
  }
  
  .section-content {
    grid-template-columns: 1fr;
    gap: 2rem;
  }
  
  .section-content.reversed {
    direction: ltr;
  }
  
  .text-content h2 {
    font-size: 2rem;
  }
  
  .tech-grid {
    grid-template-columns: 1fr;
  }
  
  .contact-content {
    grid-template-columns: 1fr;
    gap: 2rem;
  }
  
  .contact-card {
    padding: 2rem 1.5rem;
  }
  
  .about-image {
    height: 250px;
  }
}

@media (max-width: 480px) {
  .hero-content h1 {
    font-size: 2rem;
  }
  
  .tech-item {
    padding: 1.5rem;
  }
  
  .contact-item {
    padding: 1rem;
  }
}
</style>