<template>
    <div class="habit-tracker">
        <div class="list-pane">
            <div class="header">HABITS</div>
            
            <div class="habit-list">
                <div v-if="habits.length === 0" class="empty-state">
                    NO HABITS
                </div>
                <HabitItem 
                    v-for="habit in habits" 
                    :key="habit.id" 
                    :habit="habit"
                    :is-completed="isCompleted(habit.id)"
                    @toggle="toggleHabit"
                    @delete="removeHabit"
                />
            </div>

            <HabitAdd @add="addHabit" />
            
            <HabitDeleteModal 
                :show="showDeleteConfirm"
                @cancel="cancelDelete"
                @confirm="confirmDelete"
            />
        </div>

        <HabitStats 
            :total-habits="habits.length"
            :completed-count="completedCount"
        />
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import type { Habit } from '@/types';
import HabitStats from './components/habit-stats.vue';
import HabitItem from './components/habit-item.vue';
import HabitAdd from './components/habit-add.vue';
import HabitDeleteModal from './components/habit-delete-modal.vue';

const habits = ref<Habit[]>([]);
const completedHabitIds = ref<number[]>([]);
const isTauri = ref(!!(window as any).__TAURI_INTERNALS__);
const today = new Date().toISOString().split('T')[0];
const showDeleteConfirm = ref(false);
const habitToDelete = ref<number | null>(null);

const isCompleted = (id: number) => completedHabitIds.value.includes(id);

const completedCount = computed(() => completedHabitIds.value.length);

const loadData = async () => {
    if (!isTauri.value) return;
    try {
        habits.value = await invoke('fetch_habits');
        const ids = await invoke<number[]>('fetch_habit_completions', { date: today });
        completedHabitIds.value = ids;
    } catch (e) {
        console.error('Failed to load habits:', e);
    }
};

const addHabit = async (title: string) => {
    if (!isTauri.value) {
        alert('Cannot add habits in browser mode.');
        return;
    }

    try {
        await invoke('add_habit', { title });
        await loadData();
    } catch (e) {
        console.error('Failed to add habit:', e);
    }
};

const removeHabit = (id: number) => {
    habitToDelete.value = id;
    showDeleteConfirm.value = true;
};

const cancelDelete = () => {
    showDeleteConfirm.value = false;
    habitToDelete.value = null;
};

const confirmDelete = async () => {
    if (!isTauri.value || habitToDelete.value === null) return;
    
    try {
        const id = habitToDelete.value;
        await invoke('delete_habit', { id });
        habits.value = habits.value.filter(h => h.id !== id);
        completedHabitIds.value = completedHabitIds.value.filter(cid => cid !== id);
    } catch (e) {
        console.error('Failed to delete habit:', e);
    } finally {
        showDeleteConfirm.value = false;
        habitToDelete.value = null;
    }
};

const toggleHabit = async (id: number) => {
    if (!isTauri.value) return;
    
    const wasCompleted = isCompleted(id);
    const newState = !wasCompleted;


    if (newState) {
        completedHabitIds.value.push(id);
    } else {
        completedHabitIds.value = completedHabitIds.value.filter(cid => cid !== id);
    }

    try {
        await invoke('toggle_habit_completion', { 
            habitId: id, 
            date: today, 
            completed: newState 
        });
    } catch (e) {

        if (newState) {
            completedHabitIds.value = completedHabitIds.value.filter(cid => cid !== id);
        } else {
            completedHabitIds.value.push(id);
        }
        console.error('Failed to toggle habit:', e);
    }
};

onMounted(() => {
    isTauri.value = !!(window as any).__TAURI_INTERNALS__;
    if (isTauri.value) {
        loadData();
    } else {
        console.warn('Running in browser mode.');
        habits.value = [
            { id: 1, title: 'Drink Water' },
            { id: 2, title: 'Read 10 pages' },
            { id: 3, title: 'Exercise' }
        ];
        completedHabitIds.value = [1];
    }
});
</script>

<style scoped>
.habit-tracker {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    padding: 10px;
    box-sizing: border-box;
    color: var(--text-color-primary);
    overflow: hidden;
    gap: 20px;
}

.list-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.header {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 2rem;
    margin-bottom: 10px;
    letter-spacing: 2px;
}

.habit-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 10px;
    padding-right: 5px;
}

.habit-list::-webkit-scrollbar {
    width: 4px;
}
.habit-list::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.1);
}
.habit-list::-webkit-scrollbar-thumb {
    background: #fff;
    border-radius: 2px;
}

.empty-state {
    text-align: center;
    color: #555;
    margin-top: 20px;
    font-size: 1.2rem;
}
</style>

