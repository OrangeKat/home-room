<template>
    <div class="tracker-pane">
        <div class="circular-chart">
            <svg viewBox="0 0 36 36" class="circular-chart-svg">
                <path class="circle-bg"
                    d="M18 2.0845
                       a 15.9155 15.9155 0 0 1 0 31.831
                       a 15.9155 15.9155 0 0 1 0 -31.831"
                />
                <path class="circle"
                    :stroke-dasharray="progressStrokeDashArray"
                    d="M18 2.0845
                       a 15.9155 15.9155 0 0 1 0 31.831
                       a 15.9155 15.9155 0 0 1 0 -31.831"
                />
            </svg>
            <div class="percentage-text">
                <span class="percent">{{ Math.round(progressPercentage) }}%</span>
                <span class="label">TODAY</span>
            </div>
        </div>
        <div class="stats-text" v-if="totalHabits > 0">
            {{ completedCount }} / {{ totalHabits }} DONE
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
    totalHabits: number;
    completedCount: number;
}>();

const progressPercentage = computed(() => {
    if (props.totalHabits === 0) return 0;
    return (props.completedCount / props.totalHabits) * 100;
});

const progressStrokeDashArray = computed(() => {
    return `${progressPercentage.value}, 100`;
});
</script>

<style scoped>
.tracker-pane {
    flex: 0 0 40%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border-left: 1px solid rgba(255, 255, 255, 0.1);
    padding-left: 10px;
}

.circular-chart {
    position: relative;
    width: 100%;
    max-width: 150px;
    margin-bottom: 10px;
}

.circular-chart-svg {
    display: block;
    margin: 0 auto;
    max-width: 100%;
    max-height: 250px;
}

.circle-bg {
    fill: none;
    stroke: rgba(255, 255, 255, 0.1);
    stroke-width: 2.5;
}

.circle {
    fill: none;
    stroke-width: 2.5;
    stroke-linecap: round;
    stroke: var(--pastel-green);
    animation: progress 1s ease-out forwards;
    transition: stroke-dasharray 0.5s ease;
}

.percentage-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.percent {
    font-size: 2rem;
    font-weight: bold;
    font-family: 'Bebas Neue', sans-serif;
    letter-spacing: 1px;
    line-height: 1;
}

.label {
    font-size: 0.8rem;
    color: var(--text-color-secondary);
    letter-spacing: 1px;
    font-family: 'JetBrains Mono', monospace;
}

.stats-text {
    font-size: 0.9rem;
    color: var(--text-color-secondary);
    font-family: 'JetBrains Mono', monospace;
}

@keyframes progress {
    0% {
        stroke-dasharray: 0, 100;
    }
}
</style>
