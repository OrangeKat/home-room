<template>
    <div 
        class="habit-item" 
        :class="{ completed: isCompleted }"
    >
        <div class="checkbox" @click="$emit('toggle', habit.id)">
            <div class="check-mark" v-if="isCompleted">✓</div>
        </div>
        <div class="habit-content">
            <div class="habit-title" @click="$emit('toggle', habit.id)">{{ habit.title }}</div>
        </div>
        <button class="delete-btn" @click="$emit('delete', habit.id)">×</button>
    </div>
</template>

<script setup lang="ts">
import type { Habit } from '@/types';

defineProps<{
    habit: Habit;
    isCompleted: boolean;
}>();

defineEmits<{
    (e: 'toggle', id: number): void;
    (e: 'delete', id: number): void;
}>();
</script>

<style scoped>
.habit-item {
    display: flex;
    align-items: center;
    gap: 10px;
    background: rgba(255, 255, 255, 0.05);
    padding: 8px;
    border-radius: 4px;
    transition: background 0.2s;
    user-select: none;
}

.habit-item:hover {
    background: rgba(255, 255, 255, 0.1);
}

.checkbox {
    width: 20px;
    height: 20px;
    border: 2px solid #555;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: border-color 0.2s, background 0.2s;
}

.habit-item.completed .checkbox {
    border-color: var(--pastel-green);
    background: rgba(var(--pastel-green-rgb), 0.2);
}

.check-mark {
    color: var(--pastel-green);
    font-size: 14px;
    font-weight: bold;
}

.habit-content {
    flex: 1;
    overflow: hidden;
}

.habit-title {
    font-size: 1rem;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-family: 'JetBrains Mono', monospace;
}

.delete-btn {
    background: none;
    border: none;
    color: var(--text-color-secondary);
    cursor: pointer;
    font-size: 1.2rem;
    padding: 0 5px;
    font-family: 'JetBrains Mono', monospace;
}

.delete-btn:hover {
    color: var(--pastel-red);
}
</style>
