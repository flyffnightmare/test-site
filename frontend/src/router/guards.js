// router/guards.js
import axios from 'axios'

export const adminGuard = async (to, from, next) => {
  const token = localStorage.getItem('auth_token')
  
  if (!token) {
    console.log('❌ Токен не найден')
    next('/') // Перенаправляем на главную вместо /login
    return
  }

  try {
    console.log('🔐 Проверка токена...')
    
    const response = await axios.get('/api/auth/me', {
      headers: { 
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      timeout: 5000
    })

    console.log('✅ Проверка токена успешна:', response.data)

    if (response.data.success && response.data.data.role === 'admin') {
      next()
    } else {
      console.log('❌ Недостаточно прав:', response.data.data?.role)
      next('/') // Перенаправляем на главную
    }
  } catch (error) {
    console.error('❌ Ошибка проверки токена:', error.response?.status, error.response?.data)
    
    // Если токен невалидный, очищаем его
    if (error.response?.status === 401) {
      localStorage.removeItem('auth_token')
      localStorage.removeItem('user')
    }
    
    next('/') // Перенаправляем на главную
  }
}

// Guard для проверки авторизации (любой пользователь)
export const authGuard = async (to, from, next) => {
  const token = localStorage.getItem('auth_token')
  
  if (!token) {
    next('/')
    return
  }

  try {
    const response = await axios.get('/api/auth/me', {
      headers: { 'Authorization': `Bearer ${token}` }
    })

    if (response.data.success) {
      next()
    } else {
      localStorage.removeItem('auth_token')
      localStorage.removeItem('user')
      next('/')
    }
  } catch (error) {
    console.error('Auth check failed:', error)
    localStorage.removeItem('auth_token')
    localStorage.removeItem('user')
    next('/')
  }
}