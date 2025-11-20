<template>
  <div class="games">
    <div class="container">
      <div class="games-header">
        <h1>Наши игры</h1>
        <p class="games-subtitle">Откройте для себя удивительные миры, созданные с душой</p>
      </div>
      
      <div class="games-grid">
        <div v-for="game in games" :key="game.id" class="game-card">
          <div class="game-image-container">
            <img :src="game.image_url" :alt="game.title" class="game-image">
            <div class="game-genre-tag">{{ game.genre }}</div>
          </div>
          <div class="game-info">
            <h3>{{ game.title }}</h3>
            <p class="game-description">{{ game.short_description }}</p>
            <div class="game-actions">
              <router-link :to="`/games/${game.id}`" class="btn btn-primary">
                Подробнее
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

export default {
  name: 'Games',
  data() {
    return {
      games: []
    }
  },
  async mounted() {
    try {
      const response = await axios.get('/api/games')
      if (response.data.success) {
        this.games = response.data.data
      }
    } catch (error) {
      console.error('Error fetching games:', error)
    }
  }
}
</script>

<style scoped>
.games {
  padding: 3rem 0;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
  min-height: 100vh;
}

.games-header {
  text-align: center;
  margin-bottom: 4rem;
}

.games-header h1 {
  font-size: 3rem;
  margin-bottom: 1rem;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.games-subtitle {
  color: #aaa;
  font-size: 1.2rem;
  max-width: 600px;
  margin: 0 auto;
  line-height: 1.6;
}

.games-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 2.5rem;
  max-width: 1200px;
  margin: 0 auto;
}

.game-card {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 15px;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
}

.game-card:hover {
  transform: translateY(-10px);
  border-color: rgba(0, 174, 255, 0.3);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.3);
}

.game-image-container {
  position: relative;
  height: 250px;
  overflow: hidden;
}

.game-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.game-card:hover .game-image {
  transform: scale(1.05);
}

.game-genre-tag {
  position: absolute;
  top: 1rem;
  right: 1rem;
  background: rgba(0, 174, 255, 0.9);
  color: white;
  padding: 0.4rem 1rem;
  border-radius: 20px;
  font-size: 0.8rem;
  font-weight: 600;
  backdrop-filter: blur(10px);
}

.game-info {
  padding: 1.5rem;
}

.game-info h3 {
  margin-bottom: 1rem;
  color: #00aeff;
  font-size: 1.4rem;
  font-weight: 600;
}

.game-description {
  color: #b0b0b0;
  line-height: 1.6;
  margin-bottom: 1.5rem;
  font-size: 0.95rem;
}

.game-actions {
  display: flex;
  justify-content: center;
}

.btn {
  padding: 0.8rem 2rem;
  border-radius: 8px;
  text-decoration: none;
  font-weight: 600;
  transition: all 0.3s ease;
  display: inline-block;
}

.btn-primary {
  background: linear-gradient(45deg, #00aeff, #a335ee);
  color: white;
  border: none;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 174, 255, 0.4);
}

@media (max-width: 768px) {
  .games {
    padding: 2rem 0;
  }
  
  .games-header h1 {
    font-size: 2.2rem;
  }
  
  .games-grid {
    grid-template-columns: 1fr;
    gap: 2rem;
  }
  
  .game-image-container {
    height: 200px;
  }
}
</style>