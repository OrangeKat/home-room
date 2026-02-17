<template>
    <div class="clock">
        <div class="time">
            <div class="mainTime">
                {{ time.hours.toString().padStart(2, '0') }}:{{ time.minutes.toString().padStart(2, '0') }}
            </div>
            <div class="seconds">
                :{{ time.seconds.toString().padStart(2, '0') }}
            </div>
        </div>

        <TimeOfDay v-if="showTimeOfDay" :time="now" :daylightData="daylightData"/>

        <div v-if="daylightData" class="daylight-data">
            <div class="sunrise">
                <img src="./svg/sunrise.svg" style="filter: invert(100%);"/>
                {{ daylightData.sunrise.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false }) }}
            </div>
            <div class="sunset">
                <img src="./svg/sunset.svg" style="filter: invert(100%);"/>
            {{ daylightData.sunset.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false }) }}
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import TimeOfDay from './components/time-of-day.vue';
import type { DaylightData } from '@/types';

const props = defineProps<{
    daylightData?: DaylightData | null;
    showTimeOfDay?: boolean;
}>()

const now = ref(new Date());

const time = computed(() => {
    return {
        hours: now.value.getHours(),
        minutes: now.value.getMinutes(),
        seconds: now.value.getSeconds()
    };
});

setInterval(() => {
    now.value = new Date();
}, 1000);
</script>

<style scoped>
.clock {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: 15px;
}

.time {
    display: flex;
    align-items: baseline;
    justify-content: center;
    font-weight: bold;
    font-family: 'Bebas Neue', sans-serif;
    line-height: 1;
}

.mainTime {
    font-size: 8rem;
    color: var(--text-color-primary);
    margin: 0;
    line-height: 1;
}

.seconds {
    font-size: 4rem;
    line-height: 1;
    color: var(--text-color-secondary);;
}

.daylight-data {
    display: flex;
    align-items: center;
    justify-content: space-evenly;
    gap: 30px;
}

.sunrise {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: 5px;
}

.sunset {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: 5px;
}
</style>