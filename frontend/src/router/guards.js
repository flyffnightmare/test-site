// router/guards.js
import axios from 'axios'

export const adminGuard = async (to, from, next) => {
  const token = localStorage.getItem('auth_token')
  const userData = localStorage.getItem('user')
  
  console.log('🔐 AdminGuard: проверка доступа к админ-панели')
  console.log('🔐 Токен в localStorage:', token ? `есть (${token.length} chars)` : 'нет')
  
  if (!token) {
    console.log('❌ AdminGuard: токен не найден')
    next('/')
    return
  }

  try {
    console.log('🔐 Проверка токена через API...')
    
    const response = await axios.get('/api/auth/me', {
      headers: { 
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      timeout: 5000
    })

    console.log('✅ API ответ:', response.data)

    if (response.data.success && response.data.data.role === 'admin') {
      console.log('✅ AdminGuard: доступ разрешен')
      next()
    } else {
      console.log('❌ AdminGuard: недостаточно прав')
      next('/')
    }
  } catch (error) {
    console.error('❌ AdminGuard: ошибка проверки токена:', error.response?.status)
    
    if (error.response?.status === 401) {
      console.log('🔐 Токен невалидный, но НЕ удаляем его (для отладки)')
      // localStorage.removeItem('auth_token') // Закомментируйте на время отладки
      // localStorage.removeItem('user')
    }
    
    // Все равно перенаправляем, но токен остается для отладки
    next('/')
  }
}