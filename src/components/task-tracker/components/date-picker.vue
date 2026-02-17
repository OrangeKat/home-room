<template>
    <div class="calendar-picker-overlay" @click.self="$emit('close')">
        <div class="calendar-picker">
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
                        'selected': isSelected(date)
                    }"
                    @click="selectDate(date)"
                >
                    {{ date.getDate() }}
                </div>
            </div>

            <div class="actions">
                <button class="action-btn cancel" @click="$emit('close')">CANCEL</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';

const props = defineProps<{
    modelValue: string;
}>();

const emit = defineEmits(['update:modelValue', 'close']);

const currentYear = ref(new Date().getFullYear());
const currentMonth = ref(new Date().getMonth());

const weekDays = ['MON', 'TUE', 'WED', 'THU', 'FRI', 'SAT', 'SUN'];

const init = () => {
    if (props.modelValue) {
        const d = new Date(props.modelValue);
        if (!isNaN(d.getTime())) {
            currentYear.value = d.getFullYear();
            currentMonth.value = d.getMonth();
        }
    }
};

const currentMonthName = computed(() => {
    return new Date(currentYear.value, currentMonth.value).toLocaleString('default', { month: 'long' }).toUpperCase();
});

const calendarDays = computed(() => {
    const year = currentYear.value;
    const month = currentMonth.value;
    
    const firstDayOfMonth = new Date(year, month, 1);
    const lastDayOfMonth = new Date(year, month + 1, 0);
    
    const days: Date[] = [];
    
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
    if (!props.modelValue) return false;
    const [y, m, d] = props.modelValue.split('-').map(Number);

    return date.getDate() === d &&
           date.getMonth() === (m - 1) &&
           date.getFullYear() === y;
};

const changeMonth = (delta: number) => {
    const newDate = new Date(currentYear.value, currentMonth.value + delta, 1);
    currentMonth.value = newDate.getMonth();
    currentYear.value = newDate.getFullYear();
};

const selectDate = (date: Date) => {
    const y = date.getFullYear();
    const m = (date.getMonth() + 1).toString().padStart(2, '0');
    const d = date.getDate().toString().padStart(2, '0');
    emit('update:modelValue', `${y}-${m}-${d}`);
    emit('close');
};

onMounted(() => {
    init();
});
</script>

<style scoped>
.calendar-picker-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.calendar-picker {
    background: #1a1a1a;
    border-radius: 12px;
    padding: 20px;
    width: 400px;
    box-shadow: 0 10px 30px rgba(0,0,0,0.5);
    border: 1px solid rgba(255,255,255,0.1);
    color: var(--text-color-primary);
    user-select: none;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    font-size: 2.5rem;
    margin-bottom: 20px;
    line-height: 1;
    font-family: 'Bebas Neue', sans-serif;
}

.month-year {
    display: flex;
    gap: 15px;
}

.month {
    color: var(--text-color-primary);
}

.year {
    color: var(--text-color-secondary);;
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

.grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 5px;
    width: 100%;
    margin-bottom: 20px;
}

.weekday-header {
    text-align: center;
    font-size: 1.5rem;
    color: var(--text-color-secondary);;
    margin-bottom: 5px;
    font-family: 'Bebas Neue', sans-serif;
}

.day {
    aspect-ratio: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
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
}

.selected {
    background-color: var(--pastel-red);;
    color: var(--text-color-primary);
}

.day.selected {
    background-color: var(--pastel-red);;
    color: var(--text-color-primary);
}

.actions {
    display: flex;
    justify-content: center;
}

.action-btn {
    background: none;
    border: none;
    padding: 8px 20px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 1.2rem;
    transition: background 0.2s;
    color: #888;
    letter-spacing: 1px;
    font-family: 'JetBrains Mono', monospace;
}

.action-btn:hover {
    color: var(--text-color-primary);
    background: rgba(255,255,255,0.05);
}
</style>
