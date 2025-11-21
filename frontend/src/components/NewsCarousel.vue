<template>
  <section class="news-section">
    <div class="container">
      <div class="section-header">
        <h2>Последние новости</h2>
        <p>Будьте в курсе всех событий SibWinterCraft</p>
      </div>
      
      <!-- Состояние загрузки -->
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>Загружаем новости...</p>
      </div>
      
      <!-- Состояние ошибки -->
      <div v-else-if="error" class="error-state">
        <p>❌ {{ error }}</p>
        <button @click="fetchNews" class="retry-btn">Попробовать снова</button>
      </div>
      
      <!-- Пустое состояние -->
      <div v-else-if="news.length === 0" class="empty-state">
        <p>Пока нет новостей</p>
      </div>
      
      <!-- Карусель с данными -->
      <div v-else class="news-carousel-wrapper">
        <div 
          class="news-carousel" 
          :class="{ 
            'has-multiple': news.length > itemsPerView,
            'no-scroll': news.length <= itemsPerView
          }"
          ref="carousel"
        >
          <div class="news-track" ref="track">
            <div 
              v-for="(item, index) in news" 
              :key="item.id" 
              class="news-card"
              :class="{ 'active': currentSlideGroup.includes(index) }"
            >
              <div class="news-image">
                <div 
                  v-if="!item.image_url" 
                  class="gradient-placeholder"
                  :style="{ background: getGradient(index) }"
                >
                  <span class="placeholder-text">Новость</span>
                </div>
                <img 
                  v-else
                  :src="getImageUrl(item.image_url)" 
                  :alt="item.title"
                  class="news-img"
                  @error="handleImageError"
                >
                <div class="news-overlay"></div>
              </div>
              <div class="news-content">
                <h3 class="news-title">{{ item.title }}</h3>
                <p class="news-excerpt">{{ truncateText(item.content, 120) }}</p>
                <div class="news-meta">
                  <span class="news-date">{{ formatDate(item.created_at) }}</span>
                  <span class="news-author">by {{ item.author_name || 'SibWinterCraft' }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- Элементы управления - показывать только если новостей больше чем itemsPerView -->
        <div v-if="news.length > itemsPerView" class="carousel-controls">
          <button 
            @click="prevSlide" 
            class="control-btn prev"
            :disabled="currentSlide === 0"
          >
            ‹
          </button>
          
          <!-- Индикаторы -->
          <div class="carousel-indicators">
            <button
              v-for="(item, index) in slideGroups"
              :key="index"
              @click="goToSlide(index)"
              class="indicator"
              :class="{ 'active': currentSlide === index }"
            >
            </button>
          </div>
          
          <button 
            @click="nextSlide" 
            class="control-btn next"
            :disabled="currentSlide === slideGroups.length - 1"
          >
            ›
          </button>
        </div>
      </div>
    </div>
  </section>
</template>

<script>
import axios from 'axios'

export default {
  name: 'NewsCarousel',
  data() {
    return {
      news: [],
      currentSlide: 0,
      autoPlayInterval: null,
      isLoading: false,
      error: null,
      itemsPerView: 3 // Показывать по 3 новости за раз
    }
  },
  computed: {
    // Вычисляем группы по itemsPerView новостей
    slideGroups() {
      const groups = []
      for (let i = 0; i < this.news.length; i += this.itemsPerView) {
        groups.push(this.news.slice(i, i + this.itemsPerView))
      }
      return groups
    },
    // Текущая группа индексов
    currentSlideGroup() {
      const startIndex = this.currentSlide * this.itemsPerView
      return Array.from({ length: this.itemsPerView }, (_, i) => startIndex + i)
    }
  },
  async mounted() {
    await this.fetchNews()
    if (this.news.length > this.itemsPerView) {
      this.startAutoPlay()
    }
    this.updateItemsPerView()
    window.addEventListener('resize', this.updateItemsPerView)
  },
  beforeUnmount() {
    this.stopAutoPlay()
    window.removeEventListener('resize', this.updateItemsPerView)
  },
  methods: {
    async fetchNews() {
      this.isLoading = true
      this.error = null
      
      try {
        const response = await axios.get('/api/news')
        
        if (response.data.success) {
          this.news = response.data.data.slice(0, 10) // Берем первые 10 новостей
          console.log('✅ Новости загружены:', this.news.length)
        } else {
          throw new Error(response.data.message || 'Ошибка при загрузке новостей')
        }
      } catch (error) {
        console.error('❌ Ошибка загрузки новостей:', error)
        this.error = this.getErrorMessage(error)
        
        // Fallback на демо-данные если API недоступно
        if (error.response?.status === 404 || error.code === 'NETWORK_ERROR') {
          console.log('🔄 Используем демо-данные')
          this.news = this.getDemoNews()
        }
      } finally {
        this.isLoading = false
      }
    },
    
    getDemoNews() {
      // Демо-данные на случай если бэкенд недоступен
      return [
        {
          id: '1',
          title: 'Tales of Wizeria выходит в ранний доступ!',
          content: 'Мы рады сообщить, что Tales of Wizeria теперь доступна в раннем доступе на Steam! Присоединяйтесь к приключению и помогите нам сделать игру еще лучше своими отзывами.',
          image_url: '/images/news/tow-early-access.jpg',
          author_name: 'admin',
          created_at: new Date().toISOString()
        },
        {
          id: '2',
          title: 'Новые локации в разработке',
          content: 'Команда разработчиков активно работает над добавлением новых захватывающих локаций в Tales of Wizeria.',
          image_url: '/images/news/new-locations.jpg',
          author_name: 'admin',
          created_at: new Date(Date.now() - 86400000).toISOString()
        },
        {
          id: '3',
          title: 'Добро пожаловать на наш новый сайт!',
          content: 'Мы запустили совершенно новый сайт SibWinterCraft! Теперь вы можете следить за нашими проектами.',
          image_url: '/images/news/new-website.jpg',
          author_name: 'admin',
          created_at: new Date(Date.now() - 172800000).toISOString()
        }
      ]
    },
    
    getErrorMessage(error) {
      if (error.response?.status === 404) {
        return 'API новостей временно недоступно'
      } else if (error.code === 'NETWORK_ERROR') {
        return 'Проблемы с подключением к серверу'
      } else {
        return 'Не удалось загрузить новости. Попробуйте позже.'
      }
    },
    
    getImageUrl(imageUrl) {
      // Обработка URL изображений
      if (!imageUrl) return ''
      
      // Если URL абсолютный, возвращаем как есть
      if (imageUrl.startsWith('http') || imageUrl.startsWith('//')) {
        return imageUrl
      }
      
      // Если относительный URL, добавляем базовый путь
      if (imageUrl.startsWith('/')) {
        return imageUrl
      }
      
      // Для путей без слеша
      return `/${imageUrl}`
    },
    
    handleImageError(event) {
      // Замена битого изображения на градиент
      const parent = event.target.parentElement
      const placeholder = parent.querySelector('.gradient-placeholder')
      if (placeholder) {
        event.target.style.display = 'none'
        placeholder.style.display = 'flex'
      }
    },
    
    updateItemsPerView() {
      // Адаптивное количество карточек в зависимости от ширины экрана
      const width = window.innerWidth
      if (width < 768) {
        this.itemsPerView = 1
      } else if (width < 1024) {
        this.itemsPerView = 2
      } else {
        this.itemsPerView = 3
      }
    },
    
    nextSlide() {
      if (this.currentSlide < this.slideGroups.length - 1) {
        this.currentSlide++
      } else {
        this.currentSlide = 0 // Циклическая прокрутка
      }
      this.updateCarousel()
    },
    
    prevSlide() {
      if (this.currentSlide > 0) {
        this.currentSlide--
      } else {
        this.currentSlide = this.slideGroups.length - 1 // Циклическая прокрутка
      }
      this.updateCarousel()
    },
    
    goToSlide(index) {
      this.currentSlide = index
      this.updateCarousel()
    },
    
    updateCarousel() {
      const track = this.$refs.track
      if (track && this.news.length > this.itemsPerView) {
        const cardWidth = track.children[0]?.offsetWidth || 350
        const gap = 32 // 2rem gap
        const scrollPosition = this.currentSlide * this.itemsPerView * (cardWidth + gap)
        track.scrollTo({
          left: scrollPosition,
          behavior: 'smooth'
        })
      }
    },
    
    startAutoPlay() {
      this.stopAutoPlay() // Останавливаем предыдущий интервал
      this.autoPlayInterval = setInterval(() => {
        if (this.news.length > this.itemsPerView) {
          this.nextSlide()
        }
      }, 5000) // Автопрокрутка каждые 5 секунд
    },
    
    stopAutoPlay() {
      if (this.autoPlayInterval) {
        clearInterval(this.autoPlayInterval)
        this.autoPlayInterval = null
      }
    },
    
    truncateText(text, length) {
      if (!text) return ''
      if (text.length <= length) return text
      return text.substring(0, length) + '...'
    },
    
    formatDate(dateString) {
      if (!dateString) return ''
      const date = new Date(dateString)
      return date.toLocaleDateString('ru-RU', {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
      })
    },
    
    getGradient(index) {
      const gradients = [
        'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)', 
        'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
        'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)'
      ]
      return gradients[index % gradients.length]
    }
  }
}
</script>

<style scoped>
.news-section {
  padding: 4rem 0;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
}

.section-header {
  text-align: center;
  margin-bottom: 3rem;
}

.section-header h2 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.section-header p {
  color: #aaa;
  font-size: 1.1rem;
}

.news-carousel-wrapper {
  position: relative;
  max-width: 1200px;
  margin: 0 auto;
}

.news-carousel {
  overflow: hidden;
  border-radius: 15px;
}

.news-carousel.has-multiple {
  padding: 0 1rem;
}

.news-track {
  display: flex;
  gap: 2rem;
  overflow-x: auto;
  scroll-behavior: smooth;
  scrollbar-width: none;
  -ms-overflow-style: none;
  padding: 1rem 0;
  scroll-snap-type: x mandatory;
}

.news-track::-webkit-scrollbar {
  display: none;
}

.news-card {
  flex: 0 0 auto;
  width: 350px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 15px;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
  scroll-snap-align: start;
}

.news-card:hover {
  transform: translateY(-10px);
  border-color: rgba(0, 174, 255, 0.3);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.4);
}

.news-image {
  position: relative;
  height: 200px;
  overflow: hidden;
}

.news-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.news-card:hover .news-img {
  transform: scale(1.1);
}

.gradient-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  font-size: 1.2rem;
}

.placeholder-text {
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
}

.news-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(to bottom, transparent 0%, rgba(0, 0, 0, 0.7) 100%);
}

.news-content {
  padding: 1.5rem;
}

.news-title {
  color: #fff;
  font-size: 1.3rem;
  margin-bottom: 1rem;
  line-height: 1.4;
  min-height: 3.6rem;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.news-excerpt {
  color: #b0b0b0;
  line-height: 1.6;
  margin-bottom: 1rem;
  font-size: 0.95rem;
  min-height: 4.8rem;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.news-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.8rem;
  color: #666;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  padding-top: 1rem;
}

.news-date {
  color: #00aeff;
  font-weight: 600;
}

.news-author {
  color: #a335ee;
}

/* Элементы управления */
.carousel-controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 2rem;
  margin-top: 2rem;
  padding: 0 1rem;
}

.control-btn {
  background: rgba(0, 174, 255, 0.8);
  border: none;
  color: white;
  width: 50px;
  height: 50px;
  border-radius: 50%;
  font-size: 1.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.control-btn:hover:not(:disabled) {
  background: rgba(0, 174, 255, 1);
  transform: scale(1.1);
}

.control-btn:disabled {
  background: rgba(255, 255, 255, 0.2);
  color: rgba(255, 255, 255, 0.5);
  cursor: not-allowed;
  transform: none;
}

.carousel-indicators {
  display: flex;
  gap: 0.5rem;
}

.indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: none;
  background: rgba(255, 255, 255, 0.3);
  cursor: pointer;
  transition: all 0.3s ease;
}

.indicator.active {
  background: #00aeff;
  transform: scale(1.2);
}

.indicator:hover {
  background: rgba(0, 174, 255, 0.7);
}

/* Адаптивность */
@media (max-width: 768px) {
  .news-section {
    padding: 2rem 0;
  }
  
  .section-header h2 {
    font-size: 2rem;
  }
  
  .news-carousel.has-multiple {
    padding: 0 0.5rem;
  }
  
  .news-card {
    width: 300px;
  }
  
  .carousel-controls {
    gap: 1rem;
  }
  
  .control-btn {
    width: 40px;
    height: 40px;
    font-size: 1.2rem;
  }
}

@media (max-width: 480px) {
  .news-card {
    width: 280px;
  }
  
  .news-content {
    padding: 1rem;
  }
  
  .news-title {
    font-size: 1.1rem;
    min-height: 3.2rem;
  }
  
  .news-excerpt {
    font-size: 0.9rem;
    min-height: 4.2rem;
  }
  
  .carousel-indicators {
    gap: 0.3rem;
  }
  
  .indicator {
    width: 8px;
    height: 8px;
  }
}

/* Скрыть элементы управления на очень маленьких экранах */
@media (max-width: 360px) {
  .carousel-controls {
    flex-wrap: wrap;
  }
}

.loading-state {
  text-align: center;
  padding: 3rem;
  color: #aaa;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid rgba(255, 255, 255, 0.1);
  border-left: 4px solid #00aeff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-state {
  text-align: center;
  padding: 2rem;
  color: #ff6b6b;
}

.retry-btn {
  background: #00aeff;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 5px;
  cursor: pointer;
  margin-top: 1rem;
}

.retry-btn:hover {
  background: #0095d9;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: #aaa;
  font-style: italic;
}

/* Адаптивность для разного количества карточек */
@media (max-width: 1023px) {
  .news-carousel.has-multiple {
    padding: 0 0.5rem;
  }
}

@media (max-width: 767px) {
  .news-carousel.has-multiple {
    padding: 0 0.25rem;
  }
}
</style>