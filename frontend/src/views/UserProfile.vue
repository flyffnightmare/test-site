<template>
  <div class="user-profile">
    <div class="container">
      <!-- Hero секция -->
      <section class="profile-hero">
        <div class="hero-content">
          <h1>Мой кабинет</h1>
          <p class="hero-subtitle">Добро пожаловать, {{ user.username }}!</p>
        </div>
      </section>

      <div class="profile-content">
        <!-- Информация профиля -->
        <section class="profile-info">
          <div class="info-card">
            <h2>📊 Информация профиля</h2>
            <div class="info-grid">
              <div class="info-item">
                <label>Имя пользователя:</label>
                <span>{{ user.username }}</span>
              </div>
              <div class="info-item">
                <label>Email:</label>
                <span>{{ user.email }}</span>
              </div>
              <div class="info-item">
                <label>Дата регистрации:</label>
                <span>{{ formatDate(user.created_at) }}</span>
              </div>
              <div class="info-item">
                <label>Статус:</label>
                <span class="status-badge user">Игрок</span>
              </div>
            </div>
          </div>
        </section>

        <!-- Поддержка -->
        <section class="support-section">
          <h2>🛠️ Техническая поддержка</h2>
          <div class="support-content">
            <div class="support-info">
              <p>Есть вопросы или проблемы? Наша команда поддержки всегда готова помочь!</p>
              
              <div class="support-stats">
                <div class="stat-item">
                  <span class="stat-number">{{ supportRequests.length }}</span>
                  <span class="stat-label">Всего запросов</span>
                </div>
                <div class="stat-item">
                  <span class="stat-number">{{ openRequests }}</span>
                  <span class="stat-label">Активные</span>
                </div>
                <div class="stat-item">
                  <span class="stat-number">{{ resolvedRequests }}</span>
                  <span class="stat-label">Решено</span>
                </div>
              </div>
            </div>

            <!-- Форма создания запроса -->
            <div class="support-form-card">
              <h3>Создать новый запрос</h3>
              <form @submit.prevent="submitSupportRequest" class="support-form">
                <div class="form-group">
                  <label for="subject">Тема запроса:</label>
                  <input 
                    v-model="supportForm.subject"
                    type="text" 
                    id="subject"
                    required
                    placeholder="Опишите кратко проблему"
                    maxlength="100"
                  >
                </div>
                
                <div class="form-group">
                  <label for="message">Подробное описание:</label>
                  <textarea 
                    v-model="supportForm.message"
                    id="message"
                    required
                    rows="5"
                    placeholder="Опишите проблему максимально подробно..."
                    maxlength="1000"
                  ></textarea>
                  <div class="char-counter">{{ supportForm.message.length }}/1000</div>
                </div>

                <button 
                  type="submit" 
                  class="btn btn-primary"
                  :disabled="!canSubmit"
                >
                  📨 Отправить запрос
                </button>
              </form>
            </div>
          </div>

          <!-- История запросов -->
          <div class="requests-history" v-if="supportRequests.length > 0">
            <h3>История ваших запросов</h3>
            <div class="requests-list">
              <div 
                v-for="request in sortedRequests" 
                :key="request.id"
                class="request-item"
                :class="request.status"
              >
                <div class="request-header">
                  <h4>{{ request.subject }}</h4>
                  <span class="request-status" :class="request.status">
                    {{ getStatusText(request.status) }}
                  </span>
                </div>
                <p class="request-message">{{ truncateText(request.message, 150) }}</p>
                <div class="request-meta">
                  <span class="request-date">{{ formatDate(request.created_at) }}</span>
                  <span class="request-id">#{{ request.id.slice(0, 8) }}</span>
                </div>
              </div>
            </div>
          </div>
        </section>

        <!-- Игровая статистика (заглушка для будущего) -->
        <section class="game-stats">
          <h2>🎮 Игровая статистика</h2>
          <div class="stats-placeholder">
            <div class="placeholder-content">
              <div class="placeholder-icon">📊</div>
              <p>Статистика игр появится здесь скоро!</p>
              <small>Мы работаем над интеграцией с игровыми сервисами</small>
            </div>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

export default {
  name: 'UserProfile',
  data() {
    return {
      user: {
        username: '',
        email: '',
        created_at: ''
      },
      supportRequests: [],
      supportForm: {
        subject: '',
        message: ''
      },
      loading: false
    }
  },
  computed: {
    canSubmit() {
      return this.supportForm.subject.trim() && this.supportForm.message.trim()
    },
    openRequests() {
      return this.supportRequests.filter(req => req.status === 'open').length
    },
    resolvedRequests() {
      return this.supportRequests.filter(req => req.status === 'resolved').length
    },
    sortedRequests() {
      return [...this.supportRequests].sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
    }
  },
  async mounted() {
    await this.loadUserData()
    await this.loadSupportRequests()
  },
  methods: {
    async loadUserData() {
      const userData = localStorage.getItem('user')
      if (userData) {
        this.user = JSON.parse(userData)
      }
    },
    async loadSupportRequests() {
      // Заглушка - в реальном приложении здесь будет запрос к API
      this.supportRequests = [
        {
          id: '1',
          subject: 'Проблема с запуском игры',
          message: 'Игра не запускается, выдает ошибку при старте...',
          status: 'resolved',
          created_at: new Date().toISOString()
        },
        {
          id: '2', 
          subject: 'Вопрос по геймплею',
          message: 'Не понимаю, как пройти уровень в Ледяных пещерах...',
          status: 'open',
          created_at: new Date(Date.now() - 86400000).toISOString()
        }
      ]
    },
    async submitSupportRequest() {
      if (!this.canSubmit) return
      
      this.loading = true
      try {
        // В реальном приложении здесь будет запрос к API
        const newRequest = {
          id: Date.now().toString(),
          subject: this.supportForm.subject,
          message: this.supportForm.message,
          status: 'open',
          created_at: new Date().toISOString()
        }
        
        this.supportRequests.unshift(newRequest)
        
        this.supportForm.subject = ''
        this.supportForm.message = ''
        
        alert('Запрос успешно отправлен! Мы ответим вам в ближайшее время.')
      } catch (error) {
        alert('Ошибка при отправке запроса. Попробуйте позже.')
      } finally {
        this.loading = false
      }
    },
    formatDate(dateString) {
      if (!dateString) return ''
      const date = new Date(dateString)
      return date.toLocaleDateString('ru-RU', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
      })
    },
    truncateText(text, length) {
      if (!text) return ''
      if (text.length <= length) return text
      return text.substring(0, length) + '...'
    },
    getStatusText(status) {
      const statusMap = {
        'open': 'Открыт',
        'in_progress': 'В работе', 
        'resolved': 'Решено'
      }
      return statusMap[status] || status
    }
  }
}
</script>

<style scoped>
.user-profile {
  min-height: 100vh;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
  padding-bottom: 4rem;
}

.profile-hero {
  padding: 3rem 0;
  text-align: center;
  border-bottom: 1px solid #333;
}

.hero-content h1 {
  font-size: 3rem;
  margin-bottom: 1rem;
  background: linear-gradient(45deg, #ffffff, #00aeff);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.hero-subtitle {
  font-size: 1.4rem;
  color: #aaa;
}

.profile-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem 0;
}

/* Карточки информации */
.profile-info,
.support-section,
.game-stats {
  margin-bottom: 3rem;
}

.info-card,
.support-form-card {
  background: rgba(255, 255, 255, 0.05);
  padding: 2rem;
  border-radius: 15px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
}

.info-card h2,
.support-section h2,
.game-stats h2 {
  color: #00aeff;
  margin-bottom: 1.5rem;
  font-size: 1.8rem;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.info-item label {
  color: #888;
  font-size: 0.9rem;
  font-weight: 600;
}

.info-item span {
  color: #e6e6e6;
  font-size: 1.1rem;
}

.status-badge {
  padding: 0.3rem 0.8rem;
  border-radius: 15px;
  font-size: 0.8rem;
  font-weight: 600;
  display: inline-block;
}

.status-badge.user {
  background: rgba(0, 174, 255, 0.2);
  color: #00aeff;
  border: 1px solid rgba(0, 174, 255, 0.3);
}

/* Поддержка */
.support-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  margin-bottom: 2rem;
}

.support-info p {
  color: #b0b0b0;
  line-height: 1.6;
  margin-bottom: 2rem;
}

.support-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

.stat-item {
  text-align: center;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.stat-number {
  display: block;
  font-size: 2rem;
  font-weight: bold;
  color: #00aeff;
  margin-bottom: 0.5rem;
}

.stat-label {
  color: #888;
  font-size: 0.8rem;
}

/* Форма поддержки */
.support-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  color: #e6e6e6;
  font-weight: 600;
}

.form-group input,
.form-group textarea {
  padding: 0.8rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: #e6e6e6;
  font-size: 1rem;
  transition: border-color 0.3s ease;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #00aeff;
}

.form-group textarea {
  resize: vertical;
  min-height: 120px;
}

.char-counter {
  text-align: right;
  color: #666;
  font-size: 0.8rem;
}

/* История запросов */
.requests-history h3 {
  color: #00aeff;
  margin-bottom: 1.5rem;
  font-size: 1.4rem;
}

.requests-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.request-item {
  background: rgba(255, 255, 255, 0.03);
  padding: 1.5rem;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-left: 4px solid #666;
}

.request-item.open {
  border-left-color: #ff6b6b;
}

.request-item.in_progress {
  border-left-color: #ffd700;
}

.request-item.resolved {
  border-left-color: #00ff88;
}

.request-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
  gap: 1rem;
}

.request-header h4 {
  color: #e6e6e6;
  margin: 0;
  flex: 1;
}

.request-status {
  padding: 0.3rem 0.8rem;
  border-radius: 12px;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
  white-space: nowrap;
}

.request-status.open {
  background: rgba(255, 107, 107, 0.2);
  color: #ff6b6b;
}

.request-status.in_progress {
  background: rgba(255, 215, 0, 0.2);
  color: #ffd700;
}

.request-status.resolved {
  background: rgba(0, 255, 136, 0.2);
  color: #00ff88;
}

.request-message {
  color: #b0b0b0;
  line-height: 1.5;
  margin-bottom: 1rem;
}

.request-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.8rem;
  color: #666;
}

.request-id {
  font-family: monospace;
}

/* Игровая статистика */
.stats-placeholder {
  background: rgba(255, 255, 255, 0.03);
  border: 2px dashed rgba(255, 255, 255, 0.1);
  border-radius: 15px;
  padding: 3rem 2rem;
  text-align: center;
}

.placeholder-content .placeholder-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.placeholder-content p {
  color: #e6e6e6;
  font-size: 1.2rem;
  margin-bottom: 0.5rem;
}

.placeholder-content small {
  color: #888;
}

/* Кнопки */
.btn {
  padding: 0.8rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  justify-content: center;
}

.btn-primary {
  background: linear-gradient(45deg, #00aeff, #a335ee);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 174, 255, 0.4);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Адаптивность */
@media (max-width: 768px) {
  .profile-hero {
    padding: 2rem 0;
  }
  
  .hero-content h1 {
    font-size: 2.2rem;
  }
  
  .support-content {
    grid-template-columns: 1fr;
  }
  
  .support-stats {
    grid-template-columns: repeat(3, 1fr);
  }
  
  .info-grid {
    grid-template-columns: 1fr;
  }
  
  .request-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
}

@media (max-width: 480px) {
  .hero-content h1 {
    font-size: 1.8rem;
  }
  
  .info-card,
  .support-form-card {
    padding: 1.5rem;
  }
  
  .support-stats {
    grid-template-columns: 1fr;
  }
}
</style>