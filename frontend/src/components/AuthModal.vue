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
          >
        </div>
        
        <div v-if="mode === 'register'" class="form-group">
          <label>Email:</label>
          <input 
            v-model="form.email" 
            type="email" 
            required 
            placeholder="Введите ваш email"
          >
        </div>
        
        <div class="form-group">
          <label>Пароль:</label>
          <input 
            v-model="form.password" 
            type="password" 
            required 
            placeholder="Введите ваш пароль"
          >
        </div>
        
        <div v-if="mode === 'login'" class="form-group">
          <label>Капча: {{ captcha }}</label>
          <input 
            v-model="form.captcha" 
            type="text" 
            required 
            placeholder="Введите капчу"
          >
        </div>
        
        <button type="submit" class="btn btn-primary" :disabled="loading">
          {{ loading ? 'Загрузка...' : (mode === 'login' ? 'Войти' : 'Зарегистрироваться') }}
        </button>
        
        <div class="auth-switch">
          <span>{{ mode === 'login' ? 'Нет аккаунта?' : 'Уже есть аккаунт?' }}</span>
          <button type="button" class="switch-btn" @click="switchMode">
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
      captcha: 'ABC123', // В реальном приложении генерируется на бэкенде
      loading: false
    }
  },
  methods: {
    async submitForm() {
      this.loading = true
      
      try {
        const url = this.mode === 'login' ? '/api/login' : '/api/register'
        const response = await axios.post(url, this.form)
        
        if (response.data.success) {
          if (this.mode === 'login') {
            localStorage.setItem('token', response.data.data.token)
            this.$emit('close')
            location.reload() // Обновляем для отображения изменений
          } else {
            this.$emit('switch-to-login')
            alert('Регистрация успешна! Теперь вы можете войти.')
          }
        }
      } catch (error) {
        alert(error.response?.data?.message || 'Произошла ошибка')
      } finally {
        this.loading = false
      }
    },
    
    switchMode() {
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
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 2000;
}

.modal-content {
  background: #1e1e2e;
  padding: 2rem;
  border-radius: 10px;
  width: 90%;
  max-width: 400px;
  position: relative;
  border: 1px solid #333;
}

.close-btn {
  position: absolute;
  top: 1rem;
  right: 1rem;
  background: none;
  border: none;
  color: #fff;
  font-size: 1.5rem;
  cursor: pointer;
}

h2 {
  text-align: center;
  margin-bottom: 1.5rem;
  background: linear-gradient(45deg, #667eea, #764ba2);
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
  font-weight: bold;
}

input {
  padding: 0.75rem;
  border: 1px solid #444;
  border-radius: 5px;
  background: #2d2d3d;
  color: #fff;
  font-size: 1rem;
}

input:focus {
  outline: none;
  border-color: #667eea;
}

.auth-switch {
  text-align: center;
  margin-top: 1rem;
}

.switch-btn {
  background: none;
  border: none;
  color: #667eea;
  cursor: pointer;
  text-decoration: underline;
}

.switch-btn:hover {
  color: #764ba2;
}
</style>