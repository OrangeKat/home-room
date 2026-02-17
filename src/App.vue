<template>
  <div class="app-layout">
    <Sidebar />
    <main class="container">
      <div class="item size-small">
        <Clock :daylightData="daylightData" />
      </div>
      <div class="item size-small">
        <Weather :daylightData="daylightData" />
      </div>
      <div class="item size-medium">
        <HabitTracker />
      </div>
      <div class="item size-medium">
        <Calendar />
      </div>
      <div class="item size-medium">
        <TaskTracker />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import Sidebar from './components/sidebar/sidebar.vue';
import Clock from './components/clock/clock.vue';
import Weather from './components/weather/weather.vue';
import HabitTracker from './components/habit-tracker/habit-tracker.vue';
import Calendar from './components/calendar/calendar.vue';
import TaskTracker from './components/task-tracker/task-tracker.vue';
import { ref, onMounted } from 'vue';

const coordinates = {lat: 48.88210440613535, lng: 2.381232566945616}
const daylightData = ref<{ sunrise: Date, sunset: Date } | null>(null);

async function fetchDaylightData(lat: number, lng: number) {
  try {
    const response = await fetch(`https://api.sunrise-sunset.org/json?lat=${lat}&lng=${lng}&formatted=0`)
    const data = await response.json()
    
    daylightData.value = {
      sunrise: new Date(data.results.sunrise),
      sunset: new Date(data.results.sunset)
    }
  } catch (error) {
    console.error('Failed to fetch daylight data:', error);
  }
}

onMounted(() => {
  fetchDaylightData(coordinates.lat, coordinates.lng);
});
</script>

<style scoped>
.app-layout {
  display: flex;
  min-height: 100vh;
}

.container {
  margin: 0;
  min-height: 100vh;
  margin-left: 60px;
  flex-grow: 1;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-template-rows: repeat(2, 1fr);
  gap: 10px;
  padding: 10px;
}

.item {
  display: flex;
  align-items: center;
  justify-content: center;
}

.size-small {
  grid-column: span 1;
}

.size-medium {
  grid-column: span 2;
}
</style>
