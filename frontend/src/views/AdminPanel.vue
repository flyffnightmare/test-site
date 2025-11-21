<template>
  <div class="admin-panel">
    <div class="container">
      <!-- Hero секция -->
      <section class="admin-hero">
        <div class="hero-content">
          <h1>🎮 Панель администратора</h1>
          <p class="hero-subtitle">Управление контентом и пользователями SibWinterCraft</p>
        </div>
      </section>

      <!-- Состояние загрузки -->
      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>Загрузка данных... {{ loadingStep }}</p>
      </div>

      <!-- Состояние ошибки токена -->
      <div v-else-if="tokenError" class="error-state">
        <h2>❌ Ошибка доступа</h2>
        <p>Не удалось проверить ваши права доступа. Возможно, токен истек.</p>
        <button @click="logoutAndRedirect" class="btn btn-primary">
          Выйти и войти снова
        </button>
      </div>

      <!-- Доступ запрещен -->
      <div v-else-if="!hasAdminAccess" class="access-denied">
        <div class="denied-content">
          <h2>🚫 Доступ запрещен</h2>
          <p>У вас недостаточно прав для просмотра этой страницы.</p>
          <p v-if="currentUser">Ваша роль: <strong>{{ currentUser.role }}</strong></p>
          <p>Требуется роль: <strong>admin</strong></p>
          <router-link to="/" class="btn btn-primary">На главную</router-link>
        </div>
      </div>

      <!-- Основной контент (только для админов) -->
      <div v-else class="admin-content">
        <!-- Навигация по разделам -->
        <nav class="admin-nav">
          <button 
            v-for="tab in tabs" 
            :key="tab.id"
            @click="activeTab = tab.id"
            class="nav-tab"
            :class="{ active: activeTab === tab.id }"
          >
            <span class="tab-icon">{{ tab.icon }}</span>
            {{ tab.name }}
          </button>
        </nav>

        <!-- Контент разделов -->
        <div class="admin-tab-content">
          <!-- Дашборд -->
          <div v-if="activeTab === 'dashboard'" class="tab-panel">
            <div class="dashboard-stats">
              <div class="stat-card">
                <div class="stat-icon">👥</div>
                <div class="stat-info">
                  <span class="stat-number">{{ stats.users }}</span>
                  <span class="stat-label">Пользователей</span>
                </div>
              </div>
              <div class="stat-card">
                <div class="stat-icon">🎮</div>
                <div class="stat-info">
                  <span class="stat-number">{{ stats.games }}</span>
                  <span class="stat-label">Игр</span>
                </div>
              </div>
              <div class="stat-card">
                <div class="stat-icon">📰</div>
                <div class="stat-info">
                  <span class="stat-number">{{ stats.news }}</span>
                  <span class="stat-label">Новостей</span>
                </div>
              </div>
              <div class="stat-card">
                <div class="stat-icon">🛠️</div>
                <div class="stat-info">
                  <span class="stat-number">{{ stats.supportRequests }}</span>
                  <span class="stat-label">Запросов в поддержку</span>
                </div>
              </div>
            </div>

            <div class="recent-activity">
              <h3>Последняя активность</h3>
              <div class="activity-list">
                <div class="activity-item" v-for="activity in recentActivity" :key="activity.id">
                  <span class="activity-icon">{{ activity.icon }}</span>
                  <div class="activity-content">
                    <p class="activity-text">{{ activity.text }}</p>
                    <span class="activity-time">{{ formatTimeAgo(activity.created_at) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Управление пользователями -->
          <div v-if="activeTab === 'users'" class="tab-panel">
            <div class="panel-header">
              <h2>Управление пользователями</h2>
              <button class="btn btn-primary" @click="showUserModal = true">
                👤 Добавить пользователя
              </button>
            </div>

            <div class="users-list">
              <div class="user-card" v-for="user in users" :key="user.id">
                <div class="user-avatar">
                  {{ user.username.charAt(0).toUpperCase() }}
                </div>
                <div class="user-info">
                  <h4>{{ user.username }}</h4>
                  <p class="user-email">{{ user.email }}</p>
                  <span class="user-role" :class="user.role">{{ user.role }}</span>
                </div>
                <div class="user-actions">
                  <button class="btn btn-sm btn-outline" @click="editUser(user)">
                    ✏️ Редактировать
                  </button>
                  <button 
                    class="btn btn-sm btn-danger" 
                    @click="deleteUser(user.id)"
                    v-if="user.role !== 'admin'"
                  >
                    🗑️ Удалить
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Управление играми -->
          <div v-if="activeTab === 'games'" class="tab-panel">
            <div class="panel-header">
              <h2>Управление играми</h2>
              <button class="btn btn-primary" @click="showGameModal = true">
                🎮 Добавить игру
              </button>
            </div>

            <div class="games-list">
              <div class="game-card" v-for="game in games" :key="game.id">
                <img :src="game.image_url" :alt="game.title" class="game-thumb">
                <div class="game-info">
                  <h4>{{ game.title }}</h4>
                  <p class="game-genre">{{ game.genre }}</p>
                  <p class="game-description">{{ truncateText(game.short_description, 100) }}</p>
                </div>
                <div class="game-actions">
                  <button class="btn btn-sm btn-outline" @click="editGame(game)">
                    ✏️ Редактировать
                  </button>
                  <button class="btn btn-sm btn-danger" @click="deleteGame(game.id)">
                    🗑️ Удалить
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Управление новостями -->
          <div v-if="activeTab === 'news'" class="tab-panel">
            <div class="panel-header">
              <h2>Управление новостями</h2>
              <button class="btn btn-primary" @click="showNewsModal = true">
                📰 Добавить новость
              </button>
            </div>

            <div class="news-list">
              <div class="news-card" v-for="item in news" :key="item.id">
                <img 
                  :src="item.image_url || '/images/news/default.jpg'" 
                  :alt="item.title"
                  class="news-thumb"
                >
                <div class="news-content">
                  <h4>{{ item.title }}</h4>
                  <p class="news-excerpt">{{ truncateText(item.content, 150) }}</p>
                  <div class="news-meta">
                    <span class="news-date">{{ formatDate(item.created_at) }}</span>
                    <span class="news-author">Автор: {{ item.author_name }}</span>
                  </div>
                </div>
                <div class="news-actions">
                  <button class="btn btn-sm btn-outline" @click="editNews(item)">
                    ✏️ Редактировать
                  </button>
                  <button class="btn btn-sm btn-danger" @click="deleteNews(item.id)">
                    🗑️ Удалить
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Запросы в поддержку -->
          <div v-if="activeTab === 'support'" class="tab-panel">
            <div class="panel-header">
              <h2>Запросы в поддержку</h2>
              <div class="support-filters">
                <button 
                  v-for="filter in supportFilters" 
                  :key="filter.value"
                  @click="supportFilter = filter.value"
                  class="filter-btn"
                  :class="{ active: supportFilter === filter.value }"
                >
                  {{ filter.label }}
                </button>
              </div>
            </div>

            <div class="support-requests">
              <div 
                v-for="request in filteredSupportRequests" 
                :key="request.id"
                class="support-request"
                :class="request.status"
              >
                <div class="request-header">
                  <h4>{{ request.subject }}</h4>
                  <span class="request-status" :class="request.status">
                    {{ getStatusText(request.status) }}
                  </span>
                </div>
                <p class="request-message">{{ request.message }}</p>
                <div class="request-meta">
                  <span class="request-user">От: {{ request.user_name }}</span>
                  <span class="request-date">{{ formatDate(request.created_at) }}</span>
                </div>
                <div class="request-actions">
                  <button 
                    class="btn btn-sm btn-outline"
                    @click="changeRequestStatus(request.id, 'in_progress')"
                    v-if="request.status === 'open'"
                  >
                    🛠️ В работу
                  </button>
                  <button 
                    class="btn btn-sm btn-success"
                    @click="changeRequestStatus(request.id, 'resolved')"
                    v-if="request.status !== 'resolved'"
                  >
                    ✅ Решено
                  </button>
                  <button class="btn btn-sm btn-danger" @click="deleteRequest(request.id)">
                    🗑️ Удалить
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Модальные окна -->
    
    <!-- Модалка добавления/редактирования пользователя -->
    <div v-if="showUserModal" class="modal-overlay" @click="showUserModal = false">
      <div class="modal-content" @click.stop>
        <h3>{{ editingUser ? 'Редактировать пользователя' : 'Добавить пользователя' }}</h3>
        <form @submit.prevent="saveUser" class="modal-form">
          <div class="form-group">
            <label>Имя пользователя:</label>
            <input v-model="userForm.username" type="text" required>
          </div>
          <div class="form-group">
            <label>Email:</label>
            <input v-model="userForm.email" type="email" required>
          </div>
          <div class="form-group">
            <label>Роль:</label>
            <select v-model="userForm.role">
              <option value="user">Пользователь</option>
              <option value="admin">Администратор</option>
              <option value="moderator">Модератор</option>
            </select>
          </div>
          <div class="form-actions">
            <button type="button" @click="showUserModal = false" class="btn btn-secondary">
              Отмена
            </button>
            <button type="submit" class="btn btn-primary">
              {{ editingUser ? 'Обновить' : 'Создать' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Модалка добавления/редактирования игры -->
    <div v-if="showGameModal" class="modal-overlay" @click="showGameModal = false">
      <div class="modal-content" @click.stop>
        <h3>{{ editingGame ? 'Редактировать игру' : 'Добавить игру' }}</h3>
        <form @submit.prevent="saveGame" class="modal-form">
          <div class="form-group">
            <label>Название:</label>
            <input v-model="gameForm.title" type="text" required>
          </div>
          <div class="form-group">
            <label>Краткое описание:</label>
            <textarea v-model="gameForm.short_description" required></textarea>
          </div>
          <div class="form-group">
            <label>Полное описание:</label>
            <textarea v-model="gameForm.full_description" rows="4"></textarea>
          </div>
          <div class="form-group">
            <label>Жанр:</label>
            <input v-model="gameForm.genre" type="text" required>
          </div>
          <div class="form-group">
            <label>Платформа:</label>
            <input v-model="gameForm.platform" type="text" required>
          </div>
          <div class="form-group">
            <label>URL изображения:</label>
            <input v-model="gameForm.image_url" type="url">
          </div>
          <div class="form-group">
            <label>Steam URL:</label>
            <input v-model="gameForm.steam_url" type="url">
          </div>
          <div class="form-actions">
            <button type="button" @click="showGameModal = false" class="btn btn-secondary">
              Отмена
            </button>
            <button type="submit" class="btn btn-primary">
              {{ editingGame ? 'Обновить' : 'Создать' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Модалка добавления/редактирования новости -->
    <div v-if="showNewsModal" class="modal-overlay" @click="showNewsModal = false">
      <div class="modal-content" @click.stop>
        <h3>{{ editingNews ? 'Редактировать новость' : 'Добавить новость' }}</h3>
        <form @submit.prevent="saveNews" class="modal-form">
          <div class="form-group">
            <label>Заголовок:</label>
            <input v-model="newsForm.title" type="text" required>
          </div>
          <div class="form-group">
            <label>Содержание:</label>
            <textarea v-model="newsForm.content" rows="6" required></textarea>
          </div>
          <div class="form-group">
            <label>URL изображения:</label>
            <input v-model="newsForm.image_url" type="url">
          </div>
          <div class="form-actions">
            <button type="button" @click="showNewsModal = false" class="btn btn-secondary">
              Отмена
            </button>
            <button type="submit" class="btn btn-primary">
              {{ editingNews ? 'Обновить' : 'Создать' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

export default {
  name: 'AdminPanel',
  data() {
    return {
      activeTab: 'dashboard',
      tabs: [
        { id: 'dashboard', name: 'Дашборд', icon: '📊' },
        { id: 'users', name: 'Пользователи', icon: '👥' },
        { id: 'games', name: 'Игры', icon: '🎮' },
        { id: 'news', name: 'Новости', icon: '📰' },
        { id: 'support', name: 'Поддержка', icon: '🛠️' }
      ],
      stats: {
        users: 0,
        games: 0,
        news: 0,
        supportRequests: 0
      },
      users: [],
      games: [],
      news: [],
      supportRequests: [],
      recentActivity: [],
      supportFilter: 'all',
      supportFilters: [
        { value: 'all', label: 'Все' },
        { value: 'open', label: 'Открытые' },
        { value: 'in_progress', label: 'В работе' },
        { value: 'resolved', label: 'Решено' }
      ],
      
      // Модальные окна
      showUserModal: false,
      showGameModal: false,
      showNewsModal: false,
      
      // Формы
      userForm: {
        username: '',
        email: '',
        role: 'user'
      },
      gameForm: {
        title: '',
        short_description: '',
        full_description: '',
        genre: '',
        platform: '',
        image_url: '',
        steam_url: ''
      },
      newsForm: {
        title: '',
        content: '',
        image_url: ''
      },
      editingUser: null,
      editingGame: null,
      editingNews: null,
      
      // Состояния
      isLoading: true,
      loadingStep: 'Проверка прав доступа...',
      tokenError: false,
      hasAdminAccess: false,
      currentUser: null
    }
  },
  computed: {
    filteredSupportRequests() {
      if (this.supportFilter === 'all') return this.supportRequests
      return this.supportRequests.filter(req => req.status === this.supportFilter)
    }
  },
  async mounted() {
    await this.verifyToken()
    if (this.hasAdminAccess) {
      await this.loadData()
    }
  },
  methods: {
    async verifyToken() {
      const token = localStorage.getItem('auth_token')
      if (!token) {
        this.tokenError = true
        this.isLoading = false
        return
      }

      try {
        this.loadingStep = 'Проверка прав доступа...'
        const response = await axios.get('/api/auth/me', {
          headers: { 
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
          }
        })

        if (response.data.success && response.data.data.role === 'admin') {
          this.hasAdminAccess = true
          this.currentUser = response.data.data
          console.log('✅ Доступ к админ-панели разрешен')
        } else {
          this.tokenError = true
        }
      } catch (error) {
        console.error('❌ Ошибка проверки токена:', error)
        this.tokenError = true
        
        if (error.response?.status === 401) {
          this.clearAuthData()
        }
      } finally {
        this.isLoading = false
      }
    },

    async loadData() {
      this.isLoading = true
      this.loadingStep = 'Загрузка данных...'
      
      try {
        const token = localStorage.getItem('auth_token')
        const headers = { Authorization: `Bearer ${token}` }

        // Загрузка статистики
        const statsRes = await axios.get('/api/admin/stats', { headers })
        if (statsRes.data.success) this.stats = statsRes.data.data

        // Загрузка пользователей
        const usersRes = await axios.get('/api/admin/users', { headers })
        if (usersRes.data.success) this.users = usersRes.data.data

        // Загрузка игр
        const gamesRes = await axios.get('/api/games', { headers })
        if (gamesRes.data.success) this.games = gamesRes.data.data

        // Загрузка новостей
        const newsRes = await axios.get('/api/news', { headers })
        if (newsRes.data.success) this.news = newsRes.data.data

        // Загрузка запросов поддержки
        const supportRes = await axios.get('/api/admin/support-requests', { headers })
        if (supportRes.data.success) this.supportRequests = supportRes.data.data

        // Загрузка активности
        const activityRes = await axios.get('/api/admin/activity', { headers })
        if (activityRes.data.success) this.recentActivity = activityRes.data.data

      } catch (error) {
        console.error('❌ Ошибка загрузки данных:', error)
        // Fallback на демо-данные для разработки
        this.loadDemoData()
      } finally {
        this.isLoading = false
      }
    },

    loadDemoData() {
      console.log('🔄 Используем демо-данные')
      this.stats = { users: 156, games: 3, news: 5, supportRequests: 12 }
      
      this.users = [
        {
          id: '1',
          username: 'admin',
          email: 'admin@sibwintercraft.com',
          role: 'admin',
          created_at: new Date().toISOString()
        },
        {
          id: '2',
          username: 'ivanov',
          email: 'ivanov@example.com',
          role: 'user',
          created_at: new Date().toISOString()
        }
      ]
      
      this.games = [
        {
          id: '1',
          title: 'Tales of Wizeria',
          genre: 'Платформер',
          short_description: 'Увлекательный платформер в волшебном мире магии и приключений',
          image_url: '/images/games/tales-of-wizeria/main.jpg',
          created_at: new Date().toISOString()
        }
      ]
      
      this.news = [
        {
          id: '1',
          title: 'Tales of Wizeria выходит в ранний доступ!',
          content: 'Мы рады сообщить, что Tales of Wizeria теперь доступна в раннем доступе на Steam!',
          image_url: '/images/news/tow-early-access.jpg',
          author_name: 'admin',
          created_at: new Date().toISOString()
        }
      ]
      
      this.supportRequests = [
        {
          id: '1',
          subject: 'Проблема с запуском игры',
          message: 'Игра не запускается на моем компьютере...',
          status: 'open',
          user_name: 'ivanov',
          created_at: new Date().toISOString()
        }
      ]
      
      this.recentActivity = [
        {
          id: '1',
          icon: '👤',
          text: 'Новый пользователь зарегистрировался',
          created_at: new Date(Date.now() - 300000).toISOString()
        },
        {
          id: '2',
          icon: '🎮',
          text: 'Игра Tales of Wizeria обновлена',
          created_at: new Date(Date.now() - 7200000).toISOString()
        }
      ]
    },

    // Методы для пользователей
    editUser(user) {
      this.editingUser = user
      this.userForm = { ...user }
      this.showUserModal = true
    },
    
    async saveUser() {
      try {
        const token = localStorage.getItem('auth_token')
        const url = this.editingUser 
          ? `/api/admin/users/${this.editingUser.id}`
          : '/api/admin/users'
        
        const method = this.editingUser ? 'put' : 'post'
        
        const response = await axios[method](url, this.userForm, {
          headers: { Authorization: `Bearer ${token}` }
        })

        if (response.data.success) {
          this.showUserModal = false
          this.resetForms()
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка сохранения пользователя:', error)
        alert('Ошибка при сохранении пользователя')
      }
    },
    
    async deleteUser(userId) {
      if (!confirm('Вы уверены, что хотите удалить этого пользователя?')) return

      try {
        const token = localStorage.getItem('auth_token')
        const response = await axios.delete(`/api/admin/users/${userId}`, {
          headers: { Authorization: `Bearer ${token}` }
        })

        if (response.data.success) {
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка удаления пользователя:', error)
        alert('Ошибка при удалении пользователя')
      }
    },

    // Методы для игр
    editGame(game) {
      this.editingGame = game
      this.gameForm = { ...game }
      this.showGameModal = true
    },
    
    async saveGame() {
      try {
        const token = localStorage.getItem('auth_token')
        const url = this.editingGame 
          ? `/api/admin/games/${this.editingGame.id}`
          : '/api/admin/games'
        
        const method = this.editingGame ? 'put' : 'post'
        
        const response = await axios[method](url, this.gameForm, {
          headers: { Authorization: `Bearer ${token}` }
        })

        if (response.data.success) {
          this.showGameModal = false
          this.resetForms()
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка сохранения игры:', error)
        alert('Ошибка при сохранении игры')
      }
    },
    
    async deleteGame(gameId) {
      if (!confirm('Вы уверены, что хотите удалить эту игру?')) return

      try {
        const token = localStorage.getItem('auth_token')
        const response = await axios.delete(`/api/admin/games/${gameId}`, {
          headers: { Authorization: `Bearer ${token}` }
        })

        if (response.data.success) {
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка удаления игры:', error)
        alert('Ошибка при удалении игры')
      }
    },

    // Методы для новостей
    editNews(news) {
      this.editingNews = news
      this.newsForm = { ...news }
      this.showNewsModal = true
    },
    
    async saveNews() {
      try {
        const token = localStorage.getItem('auth_token')
        const response = await axios.post('/api/news', this.newsForm, {
          headers: { Authorization: `Bearer ${token}` }
        })

        if (response.data.success) {
          this.showNewsModal = false
          this.resetForms()
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка сохранения новости:', error)
        alert('Ошибка при сохранении новости')
      }
    },
    
    async deleteNews(newsId) {
      if (!confirm('Вы уверены, что хотите удалить эту новость?')) return

      try {
        const token = localStorage.getItem('auth_token')
        const response = await axios.delete(`/api/news/${newsId}`, {
          headers: { Authorization: `Bearer ${token}` }
        })

        if (response.data.success) {
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка удаления новости:', error)
        alert('Ошибка при удалении новости')
      }
    },

    // Методы для поддержки
    async changeRequestStatus(requestId, status) {
      try {
        const token = localStorage.getItem('auth_token')
        const response = await axios.put(`/api/admin/support-requests/${requestId}`, 
          { status },
          { headers: { Authorization: `Bearer ${token}` } }
        )

        if (response.data.success) {
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка изменения статуса:', error)
        alert('Ошибка при изменении статуса')
      }
    },
    
    async deleteRequest(requestId) {
      if (!confirm('Вы уверены, что хотите удалить этот запрос?')) return

      try {
        const token = localStorage.getItem('auth_token')
        const response = await axios.delete(`/api/admin/support-requests/${requestId}`, {
          headers: { Authorization: `Bearer ${token}` }
        })

        if (response.data.success) {
          await this.loadData()
        }
      } catch (error) {
        console.error('❌ Ошибка удаления запроса:', error)
        alert('Ошибка при удалении запроса')
      }
    },

    // Вспомогательные методы
    resetForms() {
      this.userForm = { username: '', email: '', role: 'user' }
      this.gameForm = { title: '', short_description: '', full_description: '', genre: '', platform: '', image_url: '', steam_url: '' }
      this.newsForm = { title: '', content: '', image_url: '' }
      this.editingUser = null
      this.editingGame = null
      this.editingNews = null
    },

    logoutAndRedirect() {
      this.clearAuthData()
      this.$router.push('/')
    },

    clearAuthData() {
      localStorage.removeItem('auth_token')
      localStorage.removeItem('user')
    },

    truncateText(text, length) {
      if (!text) return ''
      if (text.length <= length) return text
      return text.substring(0, length) + '...'
    },

    formatDate(dateString) {
      if (!dateString) return ''
      const date = new Date(dateString)
      return date.toLocaleDateString('ru-RU')
    },

    formatTimeAgo(dateString) {
      const date = new Date(dateString)
      const now = new Date()
      const diffMs = now - date
      const diffMins = Math.floor(diffMs / 60000)
      const diffHours = Math.floor(diffMs / 3600000)
      const diffDays = Math.floor(diffMs / 86400000)

      if (diffMins < 1) return 'только что'
      if (diffMins < 60) return `${diffMins} мин. назад`
      if (diffHours < 24) return `${diffHours} ч. назад`
      if (diffDays < 7) return `${diffDays} дн. назад`
      
      return date.toLocaleDateString('ru-RU')
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
.admin-panel {
  min-height: 100vh;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
  padding-bottom: 4rem;
}

.admin-hero {
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

.admin-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 2rem 0;
}

/* Навигация */
.admin-nav {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.nav-tab {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem 1.5rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: #e6e6e6;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 1rem;
  font-weight: 600;
}

.nav-tab:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(0, 174, 255, 0.3);
}

.nav-tab.active {
  background: linear-gradient(45deg, #00aeff, #a335ee);
  color: white;
  border-color: transparent;
}

.tab-icon {
  font-size: 1.2rem;
}

/* Контент вкладок */
.admin-tab-content {
  background: rgba(255, 255, 255, 0.03);
  border-radius: 15px;
  padding: 2rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
}

/* Дашборд */
.dashboard-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.stat-icon {
  font-size: 2.5rem;
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
  font-size: 0.9rem;
}

.recent-activity h3 {
  color: #00aeff;
  margin-bottom: 1.5rem;
  font-size: 1.5rem;
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.activity-item {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.activity-icon {
  font-size: 1.2rem;
}

.activity-content {
  flex: 1;
}

.activity-text {
  color: #e6e6e6;
  margin-bottom: 0.5rem;
}

.activity-time {
  color: #666;
  font-size: 0.8rem;
}

/* Заголовки панелей */
.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.panel-header h2 {
  color: #00aeff;
  margin: 0;
}

/* Списки */
.users-list,
.games-list,
.news-list,
.support-requests {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

/* Карточки пользователей */
.user-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.user-avatar {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: bold;
  font-size: 1.2rem;
}

.user-info {
  flex: 1;
}

.user-info h4 {
  color: #e6e6e6;
  margin: 0 0 0.5rem 0;
}

.user-email {
  color: #888;
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
}

.user-role {
  padding: 0.2rem 0.8rem;
  border-radius: 12px;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
}

.user-role.user {
  background: rgba(0, 174, 255, 0.2);
  color: #00aeff;
}

.user-role.admin {
  background: rgba(163, 53, 238, 0.2);
  color: #a335ee;
}

.user-actions {
  display: flex;
  gap: 0.5rem;
}

/* Карточки игр и новостей */
.game-card,
.news-card {
  display: flex;
  gap: 1.5rem;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  align-items: flex-start;
}

.game-thumb,
.news-thumb {
  width: 100px;
  height: 80px;
  object-fit: cover;
  border-radius: 8px;
}

.game-info,
.news-content {
  flex: 1;
}

.game-info h4,
.news-content h4 {
  color: #e6e6e6;
  margin: 0 0 0.5rem 0;
}

.game-genre {
  color: #00aeff;
  font-weight: 600;
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
}

.game-description,
.news-excerpt {
  color: #b0b0b0;
  line-height: 1.5;
  margin: 0 0 0.5rem 0;
}

.news-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.8rem;
  color: #666;
}

.game-actions,
.news-actions {
  display: flex;
  gap: 0.5rem;
}

/* Запросы в поддержку */
.support-filters {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.filter-btn {
  padding: 0.5rem 1rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  color: #e6e6e6;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.8rem;
}

.filter-btn.active {
  background: #00aeff;
  color: white;
  border-color: #00aeff;
}

.support-request {
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-left: 4px solid #666;
}

.support-request.open {
  border-left-color: #ff6b6b;
}

.support-request.in_progress {
  border-left-color: #ffd700;
}

.support-request.resolved {
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
  margin-bottom: 1rem;
}

.request-actions {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

/* Модальные окна */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.modal-content {
  background: #1a1a1a;
  border-radius: 15px;
  padding: 2rem;
  max-width: 500px;
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
  border: 1px solid #333;
}

.modal-content h3 {
  color: #00aeff;
  margin-bottom: 1.5rem;
  text-align: center;
}

.modal-form {
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
.form-group textarea,
.form-group select {
  padding: 0.8rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: #e6e6e6;
  font-size: 1rem;
  transition: border-color 0.3s ease;
}

.form-group input:focus,
.form-group textarea:focus,
.form-group select:focus {
  outline: none;
  border-color: #00aeff;
}

.form-group textarea {
  resize: vertical;
  min-height: 100px;
}

.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
}

/* Кнопки */
.btn {
  padding: 0.5rem 1rem;
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
  font-size: 0.9rem;
}

.btn-sm {
  padding: 0.4rem 0.8rem;
  font-size: 0.8rem;
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
  background: rgba(255, 255, 255, 0.1);
  color: #e6e6e6;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.2);
}

.btn-outline {
  background: transparent;
  color: #00aeff;
  border: 1px solid #00aeff;
}

.btn-outline:hover {
  background: rgba(0, 174, 255, 0.1);
}

.btn-success {
  background: linear-gradient(45deg, #00ff88, #00cc66);
  color: white;
}

.btn-success:hover {
  transform: translateY(-2px);
}

.btn-danger {
  background: linear-gradient(45deg, #ff6b6b, #ff4757);
  color: white;
}

.btn-danger:hover {
  transform: translateY(-2px);
}

/* Адаптивность */
@media (max-width: 768px) {
  .admin-hero {
    padding: 2rem 0;
  }
  
  .hero-content h1 {
    font-size: 2.2rem;
  }
  
  .admin-nav {
    flex-direction: column;
  }
  
  .nav-tab {
    justify-content: center;
  }
  
  .admin-tab-content {
    padding: 1.5rem;
  }
  
  .dashboard-stats {
    grid-template-columns: 1fr 1fr;
  }
  
  .panel-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .user-card,
  .game-card,
  .news-card {
    flex-direction: column;
    align-items: stretch;
  }
  
  .user-actions,
  .game-actions,
  .news-actions {
    justify-content: stretch;
  }
  
  .user-actions .btn,
  .game-actions .btn,
  .news-actions .btn {
    flex: 1;
  }
}

@media (max-width: 480px) {
  .hero-content h1 {
    font-size: 1.8rem;
  }
  
  .dashboard-stats {
    grid-template-columns: 1fr;
  }
  
  .form-actions {
    flex-direction: column;
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

.access-denied {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 50vh;
  text-align: center;
}

.denied-content h2 {
  color: #ff6b6b;
  margin-bottom: 1rem;
}

.denied-content p {
  color: #aaa;
  margin-bottom: 2rem;
} 

.debug-info {
  background: rgba(255, 255, 255, 0.1);
  padding: 1rem;
  border-radius: 8px;
  margin: 1rem 0;
  font-family: monospace;
  font-size: 0.8rem;
  max-height: 200px;
  overflow-y: auto;
}
</style>