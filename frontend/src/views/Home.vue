<template>
  <div class="home">
    <HeroCarousel :images="carouselImages" :interval="15000">
      <div class="hero-content-inner">
        <h1>Добро пожаловать в СибВинтерКрафт</h1>
        <p class="hero-subtitle">Создаем лучшие игровые впечатления в сибирском стиле</p>
        <button class="btn btn-primary hero-btn" @click="$router.push('/games')">
          Смотреть игры
        </button>
      </div>
    </HeroCarousel>
    
    <section class="features">
      <div class="container">
        <h2>Наши преимущества</h2>
        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon">🎮</div>
            <h3>Уникальные миры</h3>
            <p>Создаем игры с атмосферой сибирской природы и культуры</p>
          </div>
          <div class="feature-card">
            <div class="feature-icon">⚡</div>
            <h3>Современные технологии</h3>
            <p>Используем передовые игровые движки и графические решения</p>
          </div>
          <div class="feature-card">
            <div class="feature-icon">👥</div>
            <h3>Активное сообщество</h3>
            <p>Поддерживаем связь с игроками и учитываем их пожелания</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
import HeroCarousel from '../components/HeroCarousel.vue'

export default {
  name: 'Home',
  components: {
    HeroCarousel
  },
  data() {
    return {
      carouselImages: [
        {
          src: '/images/hero/hero-1.jpg',
          alt: 'Игровой мир СибВинтерКрафт'
        }
      ]
    }
  }
}
</script>

<style scoped>
.home {
  min-height: 100vh;
}

.hero-content-inner {
  max-width: 800px;
  padding: 0 2rem;
}

.hero-content-inner h1 {
  font-size: 3.5rem;
  margin-bottom: 1.5rem;
  background: linear-gradient(45deg, #ffffff, #00aeff);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  font-weight: 700;
  letter-spacing: -0.5px;
}

.hero-subtitle {
  font-size: 1.4rem;
  margin-bottom: 2.5rem;
  color: #e6e6e6;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
  line-height: 1.5;
  opacity: 0.95;
}

.hero-btn {
  padding: 1rem 2.5rem;
  font-size: 1.1rem;
  font-weight: 600;
  border-radius: 8px;
  box-shadow: 0 8px 25px rgba(0, 174, 255, 0.3);
  transition: all 0.3s ease;
}

.hero-btn:hover {
  transform: translateY(-3px);
  box-shadow: 0 12px 35px rgba(0, 174, 255, 0.4);
}

.features {
  padding: 5rem 0;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
}

.features h2 {
  text-align: center;
  margin-bottom: 4rem;
  font-size: 2.8rem;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 2.5rem;
  max-width: 1200px;
  margin: 0 auto;
}

.feature-card {
  background: rgba(255, 255, 255, 0.05);
  padding: 2.5rem 2rem;
  border-radius: 15px;
  text-align: center;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
}

.feature-card:hover {
  transform: translateY(-10px);
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(0, 174, 255, 0.3);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
}

.feature-icon {
  font-size: 3rem;
  margin-bottom: 1.5rem;
  display: block;
}

.feature-card h3 {
  margin-bottom: 1rem;
  color: #00aeff;
  font-size: 1.4rem;
  font-weight: 600;
}

.feature-card p {
  color: #b0b0b0;
  line-height: 1.6;
  font-size: 1rem;
}

/* Адаптивность */
@media (max-width: 768px) {
  .hero-content-inner h1 {
    font-size: 2.5rem;
  }
  
  .hero-subtitle {
    font-size: 1.2rem;
  }
  
  .features {
    padding: 3rem 0;
  }
  
  .features h2 {
    font-size: 2.2rem;
    margin-bottom: 3rem;
  }
  
  .features-grid {
    gap: 2rem;
  }
  
  .feature-card {
    padding: 2rem 1.5rem;
  }
}

@media (max-width: 480px) {
  .hero-content-inner h1 {
    font-size: 2rem;
  }
  
  .hero-subtitle {
    font-size: 1.1rem;
  }
  
  .hero-btn {
    padding: 0.9rem 2rem;
    font-size: 1rem;
  }
  
  .features h2 {
    font-size: 1.8rem;
  }
  
  .feature-icon {
    font-size: 2.5rem;
  }
}
</style>