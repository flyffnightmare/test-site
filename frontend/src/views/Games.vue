<template>
  <div class="games">
    <div class="container">
      <h1>Наши игры</h1>
      <div class="games-grid">
        <div v-for="game in games" :key="game.id" class="game-card">
          <img :src="game.image_url" :alt="game.title" class="game-image">
          <div class="game-info">
            <h3>{{ game.title }}</h3>
            <p class="game-genre">{{ game.genre }}</p>
            <p class="game-description">{{ game.description }}</p>
            <div class="game-price">${{ game.price }}</div>
            <button class="btn btn-primary">Купить</button>
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
  padding: 2rem 0;
}

h1 {
  text-align: center;
  margin-bottom: 3rem;
  font-size: 2.5rem;
}

.games-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
}

.game-card {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  overflow: hidden;
  border: 1px solid #333;
  transition: transform 0.3s ease;
}

.game-card:hover {
  transform: translateY(-5px);
}

.game-image {
  width: 100%;
  height: 200px;
  object-fit: cover;
}

.game-info {
  padding: 1.5rem;
}

.game-info h3 {
  margin-bottom: 0.5rem;
  color: #667eea;
}

.game-genre {
  color: #888;
  font-size: 0.9rem;
  margin-bottom: 1rem;
}

.game-description {
  margin-bottom: 1rem;
  line-height: 1.5;
}

.game-price {
  font-size: 1.5rem;
  font-weight: bold;
  margin-bottom: 1rem;
  color: #4CAF50;
}
</style>