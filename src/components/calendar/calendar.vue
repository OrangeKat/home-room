<template>
    <div class="calendar">
        <div class="header">
            <button class="nav-btn" @click="changeMonth(-1)">&lt;</button>
            <div class="month-year">
                <span class="month">{{ currentMonthName }}</span>
                <span class="year">{{ currentYear }}</span>
            </div>
            <button class="nav-btn" @click="changeMonth(1)">&gt;</button>
        </div>
        <div class="grid">
            <div v-for="day in weekDays" :key="day" class="weekday-header">
                {{ day }}
            </div>
            <div 
                v-for="(date, index) in calendarDays" 
                :key="index"
                class="day"
                :class="{ 
                    'today': isToday(date),
                    'other-month': !isCurrentMonth(date),
                    'selected': isSelected(date),
                    'has-tasks': hasTasks(date)
                }"
                @click="openDaySummary(date)"
            >
                {{ date.getDate() }}
            </div>
        </div>

        <DaySummary 
            v-if="showSummary && selectedDate" 
            :date="selectedDate" 
            @close="closeSummary" 
        />
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Task } from '@/types';
import DaySummary from './components/day.vue';

const today = new Date();
const currentMonth = ref(today.getMonth());
const currentYear = ref(today.getFullYear());
const selectedDate = ref<Date | null>(null);
const showSummary = ref(false);
const taskDates = ref<Set<string>>(new Set());
const isTauri = ref(false);

const weekDays = ['MON', 'TUE', 'WED', 'THU', 'FRI', 'SAT', 'SUN'];

const currentMonthName = computed(() => {
    return new Date(currentYear.value, currentMonth.value).toLocaleString('default', { month: 'long' }).toUpperCase();
});

const calendarDays = computed(() => {
    const year = currentYear.value;
    const month = currentMonth.value;
    
    const firstDayOfMonth = new Date(year, month, 1);
    const lastDayOfMonth = new Date(year, month + 1, 0);
    
    const days = [];
    
    let startDayOfWeek = firstDayOfMonth.getDay();
    startDayOfWeek = startDayOfWeek === 0 ? 6 : startDayOfWeek - 1;

    for (let i = startDayOfWeek; i > 0; i--) {
        days.push(new Date(year, month, 1 - i));
    }
    
    for (let i = 1; i <= lastDayOfMonth.getDate(); i++) {
        days.push(new Date(year, month, i));
    }
    
    const remainingCells = 42 - days.length;
    for (let i = 1; i <= remainingCells; i++) {
        days.push(new Date(year, month + 1, i));
    }
    
    return days;
});

const isToday = (date: Date) => {
    const t = new Date();
    return date.getDate() === t.getDate() &&
           date.getMonth() === t.getMonth() &&
           date.getFullYear() === t.getFullYear();
};

const isCurrentMonth = (date: Date) => {
    return date.getMonth() === currentMonth.value;
};

const isSelected = (date: Date) => {
    if (!selectedDate.value) return false;
    return date.getDate() === selectedDate.value.getDate() &&
           date.getMonth() === selectedDate.value.getMonth() &&
           date.getFullYear() === selectedDate.value.getFullYear();
};

const hasTasks = (date: Date) => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return taskDates.value.has(`${year}-${month}-${day}`);
};

const changeMonth = (delta: number) => {
    const newDate = new Date(currentYear.value, currentMonth.value + delta, 1);
    currentMonth.value = newDate.getMonth();
    currentYear.value = newDate.getFullYear();
};

const fetchTaskDates = async () => {
    if (!isTauri.value) {
        taskDates.value = new Set([
            new Date().toISOString().split('T')[0] 
        ]);
        return;
    }
    
    try {
        const tasks = await invoke<Task[]>('fetch_tasks');
        const dates = new Set<string>();
        tasks.forEach(t => {
            if (!t.completed) {
                dates.add(t.date);
            }
        });
        taskDates.value = dates;
    } catch (e) {
        console.error('Failed to fetch tasks for calendar:', e);
    }
};

const openDaySummary = (date: Date) => {
    selectedDate.value = date;
    showSummary.value = true;
};

const closeSummary = () => {
    showSummary.value = false;
    selectedDate.value = null;
    fetchTaskDates();
};

onMounted(() => {
    isTauri.value = !!(window as any).__TAURI_INTERNALS__;
    fetchTaskDates();
    window.addEventListener('task-updated', fetchTaskDates);
});

onUnmounted(() => {
    window.removeEventListener('task-updated', fetchTaskDates);
});

watch([currentMonth, currentYear], fetchTaskDates);
</script>

<style scoped>
.calendar {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-color-primary);
    padding: 10px;
    box-sizing: border-box;
    user-select: none;
    position: relative;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    max-width: 400px;
    font-size: 2.5rem;
    margin-bottom: 20px;
    line-height: 1;
    font-family: 'Bebas Neue', sans-serif;
}

.month-year {
    display: flex;
    gap: 15px;
}

.nav-btn {
    background: none;
    border: none;
    color: var(--text-color-primary);
    cursor: pointer;
    font-family: inherit;
    font-size: inherit;
    padding: 0 10px;
    opacity: 0.5;
    transition: opacity 0.2s;
}

.nav-btn:hover {
    opacity: 1;
}

.month {
    color: var(--text-color-primary);
}

.year {
    color: var(--text-color-secondary);;
}

.grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 5px;
    width: 100%;
    max-width: 400px;
}

.weekday-header {
    text-align: center;
    font-size: 1.2rem;
    color: var(--text-color-secondary);;
    margin-bottom: 5px;
}

.day {
    aspect-ratio: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
}

.day:hover {
    background: rgba(255, 255, 255, 0.1);
}

.other-month {
    color: #444;
}

.today {
    background-color: #fff;
    color: #000;
    font-weight: bold;
}


.selected {
    border: 2px solid var(--pastel-green);
}

.has-tasks {
    text-decoration: underline;
    text-decoration-thickness: 3px;
    text-underline-offset: 4px;
    text-decoration-color: currentColor;
    font-weight: bold;
}

</style>
