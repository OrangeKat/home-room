<template>
    <div class="weather">
        <div v-if="loading" class="spinner"></div>
        <div v-else-if="weather" class="content">
            <div class="header">
                <span class="city">Paris</span>
            </div>
            <div class="display">
                <img :src="weatherIcon" class="icon" />
                <div class="temp">
                    {{ Math.round(weather.current.temperature_2m) }}°
                </div>
            </div>
            <div class="condition">
                {{ weatherCondition }}
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { weatherData } from './conversions';
import { isDaytime } from '@/utils';
import type { DaylightData } from '@/types';

const weather = ref<any>(null);
const weatherIcon = ref<string>('');
const weatherCondition = ref<string>('');
const loading = ref(true);

const props = defineProps<{
    daylightData?: DaylightData | null;
}>();

const fetchWeather = async () => {
    loading.value = true;
    try {
        const response = await fetch(
            `https://api.open-meteo.com/v1/forecast?latitude=48.8566&longitude=2.3522&current=temperature_2m,is_day,weather_code&timezone=auto`
        );
        weather.value = await response.json();
        
        if (weather.value?.current) {
            const code = weather.value.current.weather_code.toString();
            const data = weatherData[code];
            if (data) {
                const isDay = isDaytime(props.daylightData, new Date());
                const details = isDay ? data.day : data.night;
                weatherIcon.value = details.image;
                weatherCondition.value = details.description;
            }
        }
    } catch (error) {
        console.error('Weather fetch error:', error);
    } finally {
        loading.value = false;
    }
};

onMounted(fetchWeather);
</script>

<style scoped>
.weather {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-color-primary);
    padding: 20px;
    text-align: center;
}

.content {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.header {
    font-size: 1.5rem;
    opacity: 0.7;
    text-transform: uppercase;
    letter-spacing: 1px;
}

.display {
    display: flex;
    align-items: center;
    gap: 15px;
}

.icon {
    width: 100px;
    height: 100px;
}

.temp {
    font-size: 3.5rem;
    font-weight: 700;
}

.condition {
    font-size: 1.1rem;
    opacity: 0.8;
}

.spinner {
    width: 30px;
    height: 30px;
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}
</style>
