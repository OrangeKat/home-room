<template>
    <div class="day-summary-overlay" @click.self="$emit('close')">
        <div class="day-summary-content">
            <div class="header">
                <div class="date-display">{{ formattedDate }}</div>
                <button class="close-btn" @click="$emit('close')">×</button>
            </div>

            <div class="content-grid">
                <div class="column tasks-column">
                    <div class="column-header">TASKS</div>
                    <div v-if="loading" class="loading">Loading...</div>
                    <div v-else class="list-container">
                        <div v-if="tasks.length === 0" class="empty-state">NO TASKS SCHEDULED</div>
                        <div 
                            v-for="task in tasks" 
                            :key="task.id" 
                            class="item task-item"
                            :class="{ completed: task.completed, urgent: task.isUrgent }"
                        >
                            <div class="item-time">{{ formatTime(task.time) }}</div>
                            <div class="item-title">{{ task.title }}</div>
                            <div class="status-icon" v-if="task.completed">✓</div>
                        </div>
                    </div>
                    
                    <div class="add-task-inline">
                        <input 
                            v-model="newTaskTitle" 
                            type="text" 
                            placeholder="New Task..." 
                            class="text-input"
                            @keyup.enter="addTask"
                        />
                        <input 
                            v-model="newTaskTime" 
                            type="time" 
                            class="time-input"
                        />
                        <button 
                            class="urgent-toggle" 
                            :class="{ active: isUrgentInput }" 
                            @click="isUrgentInput = !isUrgentInput"
                            title="Urgent"
                        >!</button>
                        <button class="add-btn" @click="addTask">+</button>
                    </div>
                </div>

                <div class="column habits-column">
                    <div class="column-header">HABITS</div>
                    <div v-if="loading" class="loading">Loading...</div>
                    <div v-else-if="habits.length === 0" class="empty-state">NO HABITS TRACKED</div>
                    <div v-else class="list-container">
                        <div 
                            v-for="h in habits" 
                            :key="h.habit.id" 
                            class="item habit-item"
                            :class="{ completed: h.completed }"
                        >
                            <div class="checkbox">
                                <span v-if="h.completed">✓</span>
                            </div>
                            <div class="item-title">{{ h.habit.title }}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Task, Habit } from '@/types';

const props = defineProps<{
    date: Date;
}>();

const emit = defineEmits(['close']);

const tasks = ref<Task[]>([]);
const habits = ref<{ habit: Habit; completed: boolean }[]>([]);
const loading = ref(true);
const isTauri = ref(false);

const newTaskTitle = ref('');
const newTaskTime = ref('');
const isUrgentInput = ref(false);

const formattedDate = computed(() => {
    return props.date.toLocaleDateString(undefined, { 
        weekday: 'long', 
        year: 'numeric', 
        month: 'long', 
        day: 'numeric' 
    }).toUpperCase();
});

const formatTime = (time: string) => {
    if (!time || time.startsWith('00:00')) return '--:--';
    return time.substring(0, 5);
};

const fetchData = async () => {
    loading.value = true;
    
    if (!isTauri.value) {
        await new Promise(r => setTimeout(r, 500));
        tasks.value = [
            { id: 1, title: 'Mock Task 1', date: '', time: '09:00', isUrgent: true, completed: false },
            { id: 2, title: 'Mock Task 2', date: '', time: '14:30', isUrgent: false, completed: true }
        ];
        habits.value = [
            { habit: { id: 1, title: 'Drink Water' }, completed: true },
            { habit: { id: 2, title: 'Exercise' }, completed: false }
        ];
        loading.value = false;
        return;
    }

    try {
        const year = props.date.getFullYear();
        const month = String(props.date.getMonth() + 1).padStart(2, '0');
        const day = String(props.date.getDate()).padStart(2, '0');
        const dateStr = `${year}-${month}-${day}`;

        const [allTasks, allHabits, completedHabitIds] = await Promise.all([
            invoke<Task[]>('fetch_tasks'),
            invoke<Habit[]>('fetch_habits'),
            invoke<number[]>('fetch_habit_completions', { date: dateStr })
        ]);

        tasks.value = allTasks.filter(t => t.date === dateStr);
        habits.value = allHabits.map(h => ({
            habit: h,
            completed: completedHabitIds.includes(h.id)
        }));

    } catch (e) {
        console.error('Failed to fetch day data:', e);
    } finally {
        loading.value = false;
    }
};

const addTask = async () => {
    if (!newTaskTitle.value) return;

    if (!isTauri.value) {
        alert('Cannot add tasks in browser mode.');
        return;
    }

    let timeWithZone = '';
    const dateForOffset = new Date();
    const offsetMinutes = dateForOffset.getTimezoneOffset();
    const sign = offsetMinutes > 0 ? '-' : '+';
    const absOffset = Math.abs(offsetMinutes);
    const offsetHours = String(Math.floor(absOffset / 60)).padStart(2, '0');
    const offsetMins = String(absOffset % 60).padStart(2, '0');

    if (newTaskTime.value) {
        timeWithZone = `${newTaskTime.value}:00${sign}${offsetHours}:${offsetMins}`;
    } else {
        timeWithZone = `00:00:00${sign}${offsetHours}:${offsetMins}`;
    }

    try {
        const year = props.date.getFullYear();
        const month = String(props.date.getMonth() + 1).padStart(2, '0');
        const day = String(props.date.getDate()).padStart(2, '0');
        const dateStr = `${year}-${month}-${day}`;

        await invoke('add_task', {
            title: newTaskTitle.value,
            date: dateStr,
            time: timeWithZone, 
            isUrgent: isUrgentInput.value 
        });
        
        newTaskTitle.value = '';
        newTaskTime.value = '';
        isUrgentInput.value = false;
        
        await fetchData();
    } catch (e) {
        console.error('Failed to add task:', e);
    }
};

onMounted(() => {
    isTauri.value = !!(window as any).__TAURI_INTERNALS__;
    fetchData();
});

watch(() => props.date, fetchData);

</script>

<style scoped>

.day-summary-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.85);
    backdrop-filter: blur(5px);
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.2s ease-out;
}

@keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
}

.day-summary-content {
    background: #1a1a1a;
    width: 90%;
    max-width: 900px;
    height: 80vh;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    flex-direction: column;
    padding: 30px;
    box-sizing: border-box;
    box-shadow: 0 20px 50px rgba(0,0,0,0.5);
}

.header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 30px;
    border-bottom: 2px solid #333;
    padding-bottom: 15px;
}

.close-btn {
    background: none;
    border: none;
    color: var(--text-color-secondary);
    font-size: 2rem;
    cursor: pointer;
    line-height: 0.5;
    padding: 0 10px;
    font-family: 'JetBrains Mono', monospace;
}

.close-btn:hover {
    color: var(--text-color-primary);
}

.date-display {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 2.5rem;
    letter-spacing: 2px;
    color: var(--text-color-primary);
}

.content-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 30px;
    flex: 1;
    overflow: hidden;
}

.column {
    display: flex;
    flex-direction: column;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 12px;
    padding: 20px;
    overflow: hidden;
}

.column-header {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 1.5rem;
    color: var(--text-color-secondary);
    margin-bottom: 15px;
    letter-spacing: 2px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    padding-bottom: 8px;
}

.list-container {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.empty-state, .loading {
    color: #555;
    font-family: 'JetBrains Mono', monospace;
    text-align: center;
    margin-top: 50px;
    font-size: 1rem;
}

.item {
    background: rgba(255, 255, 255, 0.05);
    padding: 12px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    gap: 12px;
    font-family: 'JetBrains Mono', monospace;
    font-size: 1rem;
}

.task-item {
    border-left: 4px solid transparent;
}

.task-item.urgent {
    border-left-color: var(--pastel-red);
}

.task-item.completed .item-title {
    text-decoration: line-through;
    color: var(--text-color-secondary);
}

.item-time {
    color: var(--text-color-secondary);
    font-size: 0.8rem;
    width: 45px;
}

.status-icon {
    color: var(--pastel-green);
    font-weight: bold;
}

.habit-item {
    transition: background 0.2s;
}

.habit-item.completed {
    background: rgba(119, 221, 119, 0.1);
}

.habit-item .checkbox {
    width: 20px;
    height: 20px;
    border: 2px solid #555;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--pastel-green);
}

.habit-item.completed .checkbox {
    border-color: var(--pastel-green);
    background: rgba(119, 221, 119, 0.2);
}

.add-task-inline {
    display: flex;
    gap: 8px;
    padding-top: 15px;
    margin-top: 10px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.text-input {
    flex: 1;
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: var(--text-color-primary);
    padding: 8px 12px;
    border-radius: 4px;
    font-family: 'JetBrains Mono', monospace;
    outline: none;
}

.time-input {
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: var(--text-color-primary);
    padding: 8px;
    border-radius: 4px;
    font-family: 'JetBrains Mono', monospace;
    outline: none;
    max-width: 110px;
}

.time-input::-webkit-calendar-picker-indicator {
    filter: invert(1);
    cursor: pointer;
}

.urgent-toggle {
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: #555;
    width: 35px;
    border-radius: 4px;
    font-weight: bold;
    cursor: pointer;
    font-family: 'JetBrains Mono', monospace;
}

.urgent-toggle.active {
    background: rgba(255, 105, 97, 0.2);
    color: var(--pastel-red);
}

.add-btn {
    background: #fff;
    border: none;
    color: #000;
    width: 35px;
    border-radius: 4px;
    font-weight: bold;
    cursor: pointer;
    font-size: 1.2rem;
}

.add-btn:hover {
    background: #ddd;
}
</style>
