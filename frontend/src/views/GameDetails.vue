<template>
  <div class="game-details">
    <div v-if="loading" class="loading">Загрузка...</div>
    
    <div v-else-if="game" class="game-content">
      <!-- Hero секция -->
      <section class="game-hero">
        <div class="hero-background" :style="{ backgroundImage: `url(${game.image_url})` }"></div>
        <div class="container">
          <div class="hero-content">
            <div class="game-header">
              <h1>{{ game.title }}</h1>
              <p class="game-subtitle">{{ game.short_description }}</p>
              <div class="game-meta">
                <span class="meta-item">{{ game.genre }}</span>
                <span class="meta-item">{{ game.release_date }}</span>
                <span class="meta-item">{{ game.developer }}</span>
              </div>
              <div class="hero-actions">
                <a v-if="game.steam_url" :href="game.steam_url" target="_blank" class="btn btn-steam">
                  <i class="steam-icon">🎮</i>
                  Купить в Steam
                </a>
                <button class="btn btn-trailer" @click="showTrailer = true">
                  <i class="trailer-icon">🎬</i>
                  Смотреть трейлер
                </button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Основной контент -->
      <div class="container">
        <div class="game-main">
          <!-- Описание -->
          <section class="game-description-section">
            <h2>Об игре</h2>
            <div class="description-content">
              <p v-for="(paragraph, index) in descriptionParagraphs" :key="index" class="description-text">
                {{ paragraph }}
              </p>
            </div>
          </section>

          <!-- Скриншоты -->
          <section class="screenshots-section" v-if="game.screenshots && game.screenshots.length > 0">
            <h2>Скриншоты</h2>
            <div class="screenshots-grid">
              <div 
                v-for="(screenshot, index) in game.screenshots" 
                :key="index"
                class="screenshot-item"
                @click="openLightbox(index)"
              >
                <img :src="screenshot" :alt="`${game.title} скриншот ${index + 1}`" class="screenshot-image">
              </div>
            </div>
          </section>

          <!-- Информация -->
          <section class="game-info-section">
            <h2>Информация об игре</h2>
            <div class="info-grid">
              <div class="info-item">
                <strong>Разработчик:</strong>
                <span>{{ game.developer }}</span>
              </div>
              <div class="info-item">
                <strong>Издатель:</strong>
                <span>{{ game.publisher }}</span>
              </div>
              <div class="info-item">
                <strong>Жанр:</strong>
                <span>{{ game.genre }}</span>
              </div>
              <div class="info-item">
                <strong>Дата выхода:</strong>
                <span>{{ game.release_date }}</span>
              </div>
              <div v-if="game.steam_url" class="info-item">
                <strong>Платформа:</strong>
                <span>Steam</span>
              </div>
            </div>
          </section>
        </div>
      </div>
    </div>

    <!-- Lightbox для скриншотов -->
    <div v-if="lightboxVisible" class="lightbox" @click="closeLightbox">
      <div class="lightbox-content" @click.stop>
        <button class="lightbox-close" @click="closeLightbox">×</button>
        <img :src="currentScreenshot" :alt="`${game.title} скриншот`" class="lightbox-image">
        <div class="lightbox-nav">
          <button @click="prevScreenshot" :disabled="currentScreenshotIndex === 0">‹</button>
          <span>{{ currentScreenshotIndex + 1 }} / {{ game.screenshots.length }}</span>
          <button @click="nextScreenshot" :disabled="currentScreenshotIndex === game.screenshots.length - 1">›</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

export default {
  name: 'GameDetails',
  data() {
    return {
      game: null,
      loading: true,
      lightboxVisible: false,
      currentScreenshotIndex: 0,
      showTrailer: false
    }
  },
  computed: {
    descriptionParagraphs() {
      if (!this.game || !this.game.full_description) return []
      return this.game.full_description.split('\n\n').filter(p => p.trim())
    },
    currentScreenshot() {
      if (!this.game || !this.game.screenshots) return ''
      return this.game.screenshots[this.currentScreenshotIndex]
    }
  },
  async mounted() {
    const gameId = this.$route.params.id
    await this.fetchGame(gameId)
  },
  methods: {
    async fetchGame(gameId) {
      try {
        const response = await axios.get(`/api/games/${gameId}`)
        if (response.data.success) {
          this.game = response.data.data
        }
      } catch (error) {
        console.error('Error fetching game:', error)
      } finally {
        this.loading = false
      }
    },
    openLightbox(index) {
      this.currentScreenshotIndex = index
      this.lightboxVisible = true
    },
    closeLightbox() {
      this.lightboxVisible = false
    },
    nextScreenshot() {
      if (this.currentScreenshotIndex < this.game.screenshots.length - 1) {
        this.currentScreenshotIndex++
      }
    },
    prevScreenshot() {
      if (this.currentScreenshotIndex > 0) {
        this.currentScreenshotIndex--
      }
    }
  }
}
</script>

<style scoped>
.game-details {
  min-height: 100vh;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
}

.loading {
  text-align: center;
  padding: 4rem;
  color: #00aeff;
  font-size: 1.2rem;
}

/* Hero секция */
.game-hero {
  position: relative;
  height: 70vh;
  min-height: 500px;
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
  background-size: cover;
  background-position: center;
  filter: brightness(0.4) blur(4px);
  transform: scale(1.1);
}

.hero-content {
  position: relative;
  z-index: 2;
  width: 100%;
}

.game-header h1 {
  font-size: 3.5rem;
  margin-bottom: 1rem;
  background: linear-gradient(45deg, #ffffff, #00aeff);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
}

.game-subtitle {
  font-size: 1.4rem;
  color: #e6e6e6;
  margin-bottom: 2rem;
  max-width: 600px;
  line-height: 1.5;
}

.game-meta {
  display: flex;
  gap: 1.5rem;
  margin-bottom: 2.5rem;
  flex-wrap: wrap;
}

.meta-item {
  background: rgba(0, 174, 255, 0.2);
  color: #00aeff;
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-size: 0.9rem;
  font-weight: 600;
  border: 1px solid rgba(0, 174, 255, 0.3);
}

.hero-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem 2rem;
  border-radius: 8px;
  text-decoration: none;
  font-weight: 600;
  transition: all 0.3s ease;
  border: none;
  cursor: pointer;
  font-size: 1rem;
}

.btn-steam {
  background: linear-gradient(45deg, #1b2838, #2a475e);
  color: white;
}

.btn-steam:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(27, 40, 56, 0.4);
}

.btn-trailer {
  background: rgba(255, 255, 255, 0.1);
  color: white;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.btn-trailer:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: translateY(-2px);
}

/* Основной контент */
.game-main {
  padding: 4rem 0;
  display: flex;
  flex-direction: column;
  gap: 4rem;
}

.game-description-section h2,
.screenshots-section h2,
.game-info-section h2 {
  font-size: 2.2rem;
  margin-bottom: 2rem;
  color: #00aeff;
  border-bottom: 2px solid #00aeff;
  padding-bottom: 0.5rem;
  display: inline-block;
}

.description-text {
  color: #e6e6e6;
  line-height: 1.8;
  margin-bottom: 1.5rem;
  font-size: 1.1rem;
}

/* Скриншоты */
.screenshots-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.screenshot-item {
  border-radius: 10px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s ease;
  border: 2px solid transparent;
}

.screenshot-item:hover {
  transform: translateY(-5px);
  border-color: #00aeff;
  box-shadow: 0 10px 25px rgba(0, 174, 255, 0.3);
}

.screenshot-image {
  width: 100%;
  height: 200px;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.screenshot-item:hover .screenshot-image {
  transform: scale(1.05);
}

/* Информация об игре */
.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
}

.info-item {
  background: rgba(255, 255, 255, 0.05);
  padding: 1.5rem;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.info-item strong {
  color: #00aeff;
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.info-item span {
  color: #e6e6e6;
  font-size: 1.1rem;
  font-weight: 600;
}

/* Lightbox */
.lightbox {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.lightbox-content {
  position: relative;
  max-width: 90%;
  max-height: 90%;
}

.lightbox-close {
  position: absolute;
  top: -50px;
  right: 0;
  background: none;
  border: none;
  color: white;
  font-size: 2rem;
  cursor: pointer;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.lightbox-image {
  max-width: 100%;
  max-height: 70vh;
  object-fit: contain;
  border-radius: 10px;
}

.lightbox-nav {
  position: absolute;
  bottom: -60px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 2rem;
  color: white;
}

.lightbox-nav button {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: white;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  cursor: pointer;
  font-size: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.lightbox-nav button:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.lightbox-nav button:not(:disabled):hover {
  background: rgba(255, 255, 255, 0.3);
}

@media (max-width: 768px) {
  .game-header h1 {
    font-size: 2.5rem;
  }
  
  .game-subtitle {
    font-size: 1.2rem;
  }
  
  .hero-actions {
    flex-direction: column;
  }
  
  .btn {
    justify-content: center;
  }
  
  .game-main {
    padding: 2rem 0;
    gap: 3rem;
  }
  
  .screenshots-grid {
    grid-template-columns: 1fr;
  }
  
  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>