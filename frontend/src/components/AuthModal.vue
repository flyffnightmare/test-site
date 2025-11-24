<template>
  <div class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <button class="close-btn" @click="$emit('close')">×</button>
      
      <h2>{{ mode === 'login' ? 'Вход' : 'Регистрация' }}</h2>
      
      <form @submit.prevent="submitForm" class="auth-form">
        <div class="form-group">
          <label>Логин:</label>
          <input 
            v-model="form.username" 
            type="text" 
            required 
            placeholder="Введите ваш логин"
            :disabled="loading"
          >
        </div>
        
        <div v-if="mode === 'register'" class="form-group">
          <label>Email:</label>
          <input 
            v-model="form.email" 
            type="email" 
            required 
            placeholder="Введите ваш email"
            :disabled="loading"
          >
        </div>
        
        <div class="form-group">
          <label>Пароль:</label>
          <input 
            v-model="form.password" 
            type="password" 
            required 
            placeholder="Введите ваш пароль"
            :disabled="loading"
          >
        </div>
        
        <!-- Капча для ВСЕХ форм -->
        <div class="form-group">
          <label>Капча: <span class="captcha-text">{{ captcha }}</span></label>
          <input 
            v-model="form.captcha" 
            type="text" 
            required 
            placeholder="Введите капчу"
            :disabled="loading"
          >
        </div>
        
        <button type="submit" class="btn btn-primary" :disabled="loading">
          {{ loading ? 'Загрузка...' : (mode === 'login' ? 'Войти' : 'Зарегистрироваться') }}
        </button>
        
        <!-- УБИРАЕМ автоматический редирект в confirm -->
        <div v-if="mode === 'login' && !loading" class="admin-redirect">
          <button type="button" class="btn btn-outline" @click="loginWithoutRedirect">
            Войти без перехода в админку
          </button>
        </div>
        
        <div class="auth-switch">
          <span>{{ mode === 'login' ? 'Нет аккаунта?' : 'Уже есть аккаунт?' }}</span>
          <button type="button" class="switch-btn" @click="switchMode" :disabled="loading">
            {{ mode === 'login' ? 'Зарегистрироваться' : 'Войти' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import axios from 'axios'

export default {
  name: 'AuthModal',
  props: {
    mode: {
      type: String,
      default: 'login'
    }
  },
  data() {
    return {
      form: {
        username: '',
        email: '',
        password: '',
        captcha: ''
      },
      captcha: this.generateCaptcha(),
      loading: false
    }
  },
  methods: {
    generateCaptcha() {
      const chars = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789'
      let result = ''
      for (let i = 0; i < 4; i++) {
        result += chars.charAt(Math.floor(Math.random() * chars.length))
      }
      return result
    },

    async submitForm() {
      this.loading = true
      
      try {
        if (this.mode === 'register' && this.form.captcha !== this.captcha) {
          alert('Неверная капча!')
          this.captcha = this.generateCaptcha()
          this.form.captcha = ''
          return
        }

        const url = this.mode === 'login' ? '/api/login' : '/api/register'
        
        const requestData = this.mode === 'login' 
          ? {
              username: this.form.username,
              password: this.form.password,
              captcha: this.form.captcha
            }
          : {
              username: this.form.username,
              email: this.form.email,
              password: this.form.password,
              captcha: this.form.captcha
            }

        const response = await axios.post(url, requestData, {
          timeout: 10000,
          headers: {
            'Content-Type': 'application/json'
          }
        })

        if (response.data.success) {
          if (this.mode === 'login') {
            const token = response.data.data.token
            const user = response.data.data.user
            
            // СОХРАНЯЕМ ТОКЕН И ПОЛЬЗОВАТЕЛЯ
            localStorage.setItem('auth_token', token)
            localStorage.setItem('user', JSON.stringify(user))
            
            console.log('✅ Успешный вход, токен сохранен')
            
            // ОТПРАВЛЯЕМ СОБЫТИЕ О УСПЕШНОМ ЛОГИНЕ
            this.$emit('login-success', user)
            this.$emit('close')
            
            // ПРЕДЛАГАЕМ ПЕРЕЙТИ В АДМИНКУ, НО НЕ ДЕЛАЕМ АВТОМАТИЧЕСКИ
            if (user.role === 'admin') {
              setTimeout(() => {
                if (confirm(`🎮 Добро пожаловать, ${user.username}!\n\nВы вошли как администратор. Хотите перейти в админ-панель?`)) {
                  this.goToAdmin()
                } else {
                  alert(`✅ Добро пожаловать, ${user.username}!`)
                }
              }, 100)
            } else {
              alert(`✅ Добро пожаловать, ${user.username}!`)
            }
          } else {
            this.$emit('switch-to-login')
            alert('✅ Регистрация успешна! Теперь вы можете войти.')
          }
        }
      } catch (error) {
        console.error('❌ Ошибка авторизации:', error)
        const errorMessage = error.response?.data?.message || 'Произошла ошибка'
        alert(`❌ ${errorMessage}`)
        
        if (this.mode === 'register') {
          this.captcha = this.generateCaptcha()
          this.form.captcha = ''
        }
      } finally {
        this.loading = false
      }
    },

    // НОВЫЙ МЕТОД: Вход без предложения перейти в админку
    async loginWithoutRedirect() {
      this.loading = true
      
      try {
        const url = '/api/login'
        const requestData = {
          username: this.form.username,
          password: this.form.password,
          captcha: this.form.captcha
        }

        const response = await axios.post(url, requestData)
        
        if (response.data.success) {
          const token = response.data.data.token
          const user = response.data.data.user
          
          localStorage.setItem('auth_token', token)
          localStorage.setItem('user', JSON.stringify(user))
          
          console.log('✅ Успешный вход (без редиректа)')
          
          this.$emit('login-success', user)
          this.$emit('close')
          alert(`✅ Добро пожаловать, ${user.username}!`)
        }
      } catch (error) {
        console.error('❌ Ошибка входа:', error)
        const errorMessage = error.response?.data?.message || 'Произошла ошибка'
        alert(`❌ ${errorMessage}`)
      } finally {
        this.loading = false
      }
    },

    // Метод для перехода в админку
    goToAdmin() {
      console.log('🔄 Переход в админ-панель...')
      console.log('🔐 Токен перед переходом:', localStorage.getItem('auth_token') ? 'есть' : 'нет')
      
      // Проверяем что токен сохранен
      const token = localStorage.getItem('auth_token')
      if (!token) {
        alert('❌ Ошибка: токен не найден. Войдите снова.')
        return
      }
      
      // Переходим в админку
      this.$router.push('/admin')
    },
    
    switchMode() {
      this.form = {
        username: '',
        email: '',
        password: '',
        captcha: ''
      }
      this.captcha = this.generateCaptcha()
      
      if (this.mode === 'login') {
        this.$emit('switch-to-register')
      } else {
        this.$emit('switch-to-login')
      }
    }
  }
}
</script>


<style scoped>

.admin-redirect {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  text-align: center;
}

.btn-outline {
  background: transparent;
  border: 1px solid #00aeff;
  color: #00aeff;
}

.btn-outline:hover {
  background: rgba(0, 174, 255, 0.1);
}

.modal-overlay {
  position: fixed;
  top: 0 !important;
  left: 0 !important;
  width: 100vw !important;
  height: 100vh !important;
  background: rgba(0, 0, 0, 0.8) !important;
  display: flex !important;
  justify-content: center !important;
  align-items: center !important;
  z-index: 9999 !important;
  padding: 20px;
}

.modal-content {
  background: #1a1a1a;
  padding: 2rem;
  border-radius: 15px;
  width: 100%;
  max-width: 400px;
  border: 1px solid #333;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
  position: relative;
}

.close-btn {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  background: none;
  border: none;
  color: #aaa;
  font-size: 1.5rem;
  cursor: pointer;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  z-index: 1;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

h2 {
  text-align: center;
  margin-bottom: 1.5rem;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

label {
  font-weight: 600;
  color: #e6e6e6;
  font-size: 0.9rem;
}

.captcha-text {
  font-family: monospace;
  font-size: 1.1rem;
  font-weight: bold;
  color: #00aeff;
  background: rgba(0, 174, 255, 0.1);
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  letter-spacing: 2px;
}

input {
  padding: 0.75rem;
  border: 1px solid #444;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.05);
  color: #fff;
  font-size: 1rem;
  transition: all 0.3s ease;
}

input:focus {
  outline: none;
  border-color: #00aeff;
  box-shadow: 0 0 0 2px rgba(0, 174, 255, 0.2);
}

input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn {
  padding: 0.75rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  background: linear-gradient(45deg, #00aeff, #a335ee);
  color: white;
  margin-top: 0.5rem;
}

.btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 174, 255, 0.4);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.auth-switch {
  text-align: center;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  color: #aaa;
  font-size: 0.9rem;
}

.switch-btn {
  background: none;
  border: none;
  color: #00aeff;
  cursor: pointer;
  text-decoration: underline;
  margin-left: 0.5rem;
  font-size: 0.9rem;
}

.switch-btn:hover:not(:disabled) {
  color: #a335ee;
}

.switch-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Стили для отладочной информации */
.debug-info {
  margin-top: 1rem;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  border: 1px solid #333;
  font-size: 0.8rem;
  color: #aaa;
}

.debug-info h4 {
  color: #ff6b6b;
  margin-bottom: 0.5rem;
}

.debug-info pre {
  white-space: pre-wrap;
  word-wrap: break-word;
  margin: 0;
}

/* Адаптивность для мобильных */
@media (max-width: 480px) {
  .modal-overlay {
    padding: 10px;
  }
  
  .modal-content {
    padding: 1.5rem;
  }
  
  h2 {
    font-size: 1.3rem;
    margin-bottom: 1rem;
  }
}
</style>