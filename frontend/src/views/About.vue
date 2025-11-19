<template>
  <div class="about">
    <div class="container">
      <h1>О компании</h1>
      <div class="about-content">
        <div class="about-text">
          <p>Мы - GameStudio, команда passionate разработчиков, создающих уникальные игровые миры.</p>
          <p>Наша миссия - дарить незабываемые эмоции через качественные игры.</p>
        </div>
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
  padding: 2rem 0;
}

h1 {
  text-align: center;
  margin-bottom: 3rem;
  font-size: 2.5rem;
}

.about-content {
  max-width: 800px;
  margin: 0 auto;
}

.about-text p {
  font-size: 1.1rem;
  line-height: 1.6;
  margin-bottom: 1rem;
  text-align: center;
}
</style>