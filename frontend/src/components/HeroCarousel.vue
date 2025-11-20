<template>
  <div class="hero-carousel">
    <div class="carousel-container">
      <div 
        class="carousel-track"
        :style="{ transform: `translateX(-${currentIndex * 100}%)` }"
      >
        <div 
          v-for="(image, index) in images" 
          :key="index"
          class="carousel-slide"
        >
          <img 
            :src="image.src" 
            :alt="image.alt"
            class="carousel-image"
          />
          <div class="image-overlay"></div>
        </div>
      </div>
    </div>
    
    <!-- Контент поверх карусели -->
    <div class="hero-content">
      <slot></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: 'HeroCarousel',
  props: {
    images: {
      type: Array,
      required: true,
      default: () => []
    },
    interval: {
      type: Number,
      default: 15000
    }
  },
  data() {
    return {
      currentIndex: 0,
      intervalId: null
    }
  },
  computed: {
    hasMultipleImages() {
      return this.images.length > 1
    }
  },
  mounted() {
    if (this.hasMultipleImages) {
      this.startAutoPlay()
    }
  },
  beforeUnmount() {
    this.stopAutoPlay()
  },
  methods: {
    startAutoPlay() {
      this.intervalId = setInterval(() => {
        this.nextSlide()
      }, this.interval)
    },
    stopAutoPlay() {
      if (this.intervalId) {
        clearInterval(this.intervalId)
        this.intervalId = null
      }
    },
    nextSlide() {
      this.currentIndex = (this.currentIndex + 1) % this.images.length
    }
  }
}
</script>

<style scoped>
.hero-carousel {
  position: relative;
  width: 100%;
  height: 100vh;
  min-height: 600px;
  overflow: hidden;
}

.carousel-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.carousel-track {
  display: flex;
  height: 100%;
  transition: transform 1.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.carousel-slide {
  position: relative;
  min-width: 100%;
  height: 100%;
  flex-shrink: 0;
}

.carousel-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center;
  filter: blur(8px) brightness(0.7);
  transform: scale(1.1);
  transition: filter 1.2s ease;
}

.image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    135deg,
    rgba(10, 10, 10, 0.7) 0%,
    rgba(26, 26, 26, 0.8) 50%,
    rgba(10, 10, 10, 0.7) 100%
  );
}

.hero-content {
  position: relative;
  z-index: 10;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: white;
}

/* Адаптивность */
@media (max-width: 768px) {
  .hero-carousel {
    height: 70vh;
    min-height: 500px;
  }
  
  .carousel-image {
    filter: blur(4px) brightness(0.6);
  }
}

@media (max-width: 480px) {
  .hero-carousel {
    height: 60vh;
    min-height: 400px;
  }
}
</style>