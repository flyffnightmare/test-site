<template>
  <div class="contacts">
    <div class="container">
      <h1>Контакты</h1>
      <div class="contacts-content">
        <div class="contact-info">
          <h2>Свяжитесь с нами</h2>
          <div class="contact-item">
            <strong>Email:</strong> info@gamestudio.com
          </div>
          <div class="contact-item">
            <strong>Телефон:</strong> +7 (999) 123-45-67
          </div>
          <div class="contact-item">
            <strong>Адрес:</strong> г. Москва, ул. Игровая, д. 123
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Contacts'
}
</script>

<style scoped>
.contacts {
  padding: 2rem 0;
}

h1 {
  text-align: center;
  margin-bottom: 3rem;
  font-size: 2.5rem;
}

.contacts-content {
  max-width: 600px;
  margin: 0 auto;
}

.contact-info {
  background: rgba(255, 255, 255, 0.05);
  padding: 2rem;
  border-radius: 10px;
  border: 1px solid #333;
}

.contact-info h2 {
  margin-bottom: 1.5rem;
  color: #667eea;
}

.contact-item {
  margin-bottom: 1rem;
  font-size: 1.1rem;
}
</style>