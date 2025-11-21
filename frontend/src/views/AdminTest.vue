<template>
  <div class="admin-test">
    <div class="container">
      <h1>Тест админ-доступа</h1>
      
      <div v-if="user" class="user-info">
        <h2>Информация о пользователе:</h2>
        <p><strong>Имя:</strong> {{ user.username }}</p>
        <p><strong>Email:</strong> {{ user.email }}</p>
        <p><strong>Роль:</strong> <span :class="user.role">{{ user.role }}</span></p>
        
        <div v-if="user.role === 'admin'" class="admin-access">
          <h3>🎉 У вас есть доступ к админ-панели!</h3>
          <router-link to="/admin" class="btn btn-primary">
            Перейти в админ-панель
          </router-link>
        </div>
        
        <div v-else class="no-access">
          <h3>❌ У вас нет прав администратора</h3>
          <p>Обратитесь к администратору для получения прав.</p>
        </div>
      </div>
      
      <div v-else class="not-logged-in">
        <p>Вы не авторизованы</p>
        <button @click="$router.push('/')" class="btn btn-primary">
          На главную
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'AdminTest',
  data() {
    return {
      user: null
    }
  },
  mounted() {
    const userData = localStorage.getItem('user')
    if (userData) {
      this.user = JSON.parse(userData)
    }
  }
}
</script>

<style scoped>
.admin-test {
  min-height: 100vh;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
  padding: 2rem 0;
  color: white;
}

.user-info {
  background: rgba(255, 255, 255, 0.05);
  padding: 2rem;
  border-radius: 15px;
  margin: 2rem 0;
}

.admin-access {
  background: rgba(0, 255, 0, 0.1);
  padding: 1.5rem;
  border-radius: 10px;
  border: 1px solid rgba(0, 255, 0, 0.3);
  margin-top: 1rem;
}

.no-access {
  background: rgba(255, 0, 0, 0.1);
  padding: 1.5rem;
  border-radius: 10px;
  border: 1px solid rgba(255, 0, 0, 0.3);
  margin-top: 1rem;
}

.not-logged-in {
  text-align: center;
  padding: 3rem;
}

.user-role {
  padding: 0.3rem 0.8rem;
  border-radius: 15px;
  font-weight: bold;
}

.user-role.admin {
  background: rgba(0, 174, 255, 0.2);
  color: #00aeff;
}

.user-role.user {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}
</style>