<template>
    <div class="time-of-day">
        <img v-if="isDaytime" src="../svg/sun.svg" style="filter: invert(100%);" width="200px" height="200px"/>
        <img v-else src="../svg/moon.svg" style="filter: invert(100%);" width="200px" height="200px"/>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { DaylightData } from '@/types';

const props = defineProps<{
    time: Date;
    daylightData?: DaylightData | null;
}>();

const isDaytime = computed(() => {
    if (!props.daylightData) {
        const hours = props.time.getHours();
        return hours >= 6 && hours < 18;
    }
    
    const now = props.time.getTime();
    const sunrise = props.daylightData.sunrise.getTime();
    const sunset = props.daylightData.sunset.getTime();
    
    return now >= sunrise && now < sunset;
});
</script>

<style scoped>
.time-of-day {
    font-size: 1.5rem;
    color: var(--text-color, #fff);
}
</style>