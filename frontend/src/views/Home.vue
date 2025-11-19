<template>
  <div class="home">
    <section class="hero">
      <div class="container">
        <h1>Добро пожаловать в GameStudio</h1>
        <p>Создаем лучшие игровые впечатления</p>
        <button class="btn btn-primary" @click="$router.push('/games')">Смотреть игры</button>
      </div>
    </section>
    
    <section class="features">
      <div class="container">
        <h2>Наши преимущества</h2>
        <div class="features-grid">
          <div class="feature-card">
            <h3>Качественная графика</h3>
            <p>Современные технологии и детальная проработка</p>
          </div>
          <div class="feature-card">
            <h3>Увлекательный геймплей</h3>
            <p>Захватывающие сюжеты и механики</p>
          </div>
          <div class="feature-card">
            <h3>Поддержка</h3>
            <p>Постоянное обновление и техподдержка</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: 'Home'
}
</script>

<style scoped>
.hero {
  text-align: center;
  padding: 4rem 0;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
}

.hero h1 {
  font-size: 3rem;
  margin-bottom: 1rem;
  background: linear-gradient(45deg, #667eea, #764ba2);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.hero p {
  font-size: 1.2rem;
  margin-bottom: 2rem;
  opacity: 0.8;
}

.features {
  padding: 4rem 0;
}

.features h2 {
  text-align: center;
  margin-bottom: 3rem;
  font-size: 2.5rem;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
}

.feature-card {
  background: rgba(255, 255, 255, 0.05);
  padding: 2rem;
  border-radius: 10px;
  text-align: center;
  border: 1px solid #333;
  transition: transform 0.3s ease;
}

.feature-card:hover {
  transform: translateY(-5px);
}

.feature-card h3 {
  margin-bottom: 1rem;
  color: #667eea;
}
</style>