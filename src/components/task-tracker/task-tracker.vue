<template>
    <div class="task-tracker">
        <div class="header">
            TASKS
        </div>
        <div class="task-list">
            <TaskGroup 
                v-for="group in groupedTasks" 
                :key="group.label" 
                :label="group.label" 
                :tasks="group.tasks"
                @toggle="toggleTask"
                @remove="removeTask"
            />
            <div v-if="groupedTasks.length === 0" class="empty-state">
                NO TASKS
            </div>
        </div>
        
        <TaskAdd 
            :is-tauri="isTauri"
            @add="handleAddTask"
        />
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Task } from '@/types';
import TaskGroup from './components/task-group.vue';
import TaskAdd from './components/task-add.vue';

const tasks = ref<Task[]>([]);
const isTauri = ref(!!(window as any).__TAURI_INTERNALS__);

const groupedTasks = computed(() => {
    const sorted = [...tasks.value].sort((a, b) => {
        if (a.isUrgent !== b.isUrgent) {
             return a.isUrgent ? -1 : 1;
        }
        
        const dateA = new Date(`${a.date}T${a.time}`);
        const dateB = new Date(`${b.date}T${b.time}`);
        return dateA.getTime() - dateB.getTime();
    });

    const groups: { label: string, tasks: Task[] }[] = [];
    const today = new Date().toISOString().split('T')[0];
    const tomorrow = new Date(Date.now() + 86400000).toISOString().split('T')[0];

    sorted.forEach(task => {
        let label = task.date;
        if (task.date === today) label = 'TODAY';
        else if (task.date === tomorrow) label = 'TOMORROW';
        else {
            const d = new Date(task.date);
            label = d.toLocaleDateString(undefined, { month: 'short', day: 'numeric' }).toUpperCase();
        }

        const lastGroup = groups[groups.length - 1];
        if (lastGroup && lastGroup.label === label) {
            lastGroup.tasks.push(task);
        } else {
            groups.push({ label, tasks: [task] });
        }
    });

    return groups;
});

const loadTasks = async () => {
    if (!isTauri.value) return;
    try {
        tasks.value = await invoke<Task[]>('fetch_tasks');
    } catch (e) {
        console.error('Failed to load tasks:', e);
    }
};

const handleAddTask = async (taskData: { title: string, date: string, time: string, isUrgent: boolean }) => {
    let timeWithZone = '';
    
    const dateForOffset = new Date();
    const offsetMinutes = dateForOffset.getTimezoneOffset();
    const sign = offsetMinutes > 0 ? '-' : '+';
    const absOffset = Math.abs(offsetMinutes);
    const offsetHours = String(Math.floor(absOffset / 60)).padStart(2, '0');
    const offsetMins = String(absOffset % 60).padStart(2, '0');
    const timezoneString = `${sign}${offsetHours}:${offsetMins}`;

    if (taskData.time) {
        timeWithZone = `${taskData.time}:00${timezoneString}`;
    } else {
        timeWithZone = `00:00:00${timezoneString}`;
    }

    try {
        await invoke('add_task', {
            title: taskData.title,
            date: taskData.date,
            time: timeWithZone, 
            isUrgent: taskData.isUrgent 
        });
        await loadTasks();
        window.dispatchEvent(new CustomEvent('task-updated'));
    } catch (e) {
        console.error('Failed to add task:', e);
    }
};

const removeTask = async (id: number) => {
    if (!isTauri.value) return;
    try {
        await invoke('delete_task', { id });
        tasks.value = tasks.value.filter(t => t.id !== id);
        window.dispatchEvent(new CustomEvent('task-updated'));
    } catch (e) {
        console.error('Failed to delete task:', e);
    }
};

const toggleTask = async (id: number) => {
    if (!isTauri.value) return;
    const task = tasks.value.find(t => t.id === id);
    if (task) {
        const newState = !task.completed;
        task.completed = newState;
        
        try {
            await invoke('toggle_task_completion', { id, completed: newState });
            window.dispatchEvent(new CustomEvent('task-updated'));
        } catch (e) {
            task.completed = !newState;
            console.error('Failed to toggle task:', e);
        }
    }
};

onMounted(() => {
    isTauri.value = !!(window as any).__TAURI_INTERNALS__;
    if (isTauri.value) {
        loadTasks();
    } else {
        console.warn('Running in browser mode. Backend features are disabled.');
        tasks.value = [
            { id: 1, title: 'Mock Task (Browser Mode)', date: new Date().toISOString().split('T')[0], time: '12:00', isUrgent: false, completed: false }
        ];
    }
});
</script>

<style scoped>
.task-tracker {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 10px;
    box-sizing: border-box;
    color: var(--text-color-primary);
    overflow: hidden;
}

.header {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 2rem;
    margin-bottom: 10px;
    text-align: center;
    letter-spacing: 2px;
}

.task-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 10px;
    padding-right: 5px;
}

.task-list::-webkit-scrollbar {
    width: 4px;
}
.task-list::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.1);
}
.task-list::-webkit-scrollbar-thumb {
    background: #fff;
    border-radius: 2px;
}

.empty-state {
    text-align: center;
    color: #555;
    margin-top: 20px;
    font-size: 1.5rem;
}
</style>

