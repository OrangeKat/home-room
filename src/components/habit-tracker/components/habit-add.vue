<template>
    <div class="add-habit">
        <div class="input-row single-line">
            <div class="input-wrapper">
                <input 
                    v-model="newHabitTitle" 
                    type="text" 
                    placeholder="New Habit" 
                    class="text-input"
                    @keyup.enter="handleAdd"
                />
            </div>
            <button class="add-btn" @click="handleAdd">+</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const newHabitTitle = ref('');

const emit = defineEmits<{
    (e: 'add', title: string): void;
}>();

const handleAdd = () => {
    if (!newHabitTitle.value.trim()) return;
    emit('add', newHabitTitle.value);
    newHabitTitle.value = '';
};
</script>

<style scoped>
.add-habit {
    display: flex;
    flex-direction: column;
}

.input-row {
    display: flex;
    gap: 5px;
    align-items: center;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    padding: 2px;
}

.input-wrapper {
    flex: 1;
    height: 100%;
}

.text-input {
    background: none;
    border: none;
    color: var(--text-color-primary);
    padding: 8px 12px;
    font-size: 0.9rem;
    outline: none;
    width: 100%;
    font-family: 'JetBrains Mono', monospace;
}

.add-btn {
    background: #fff;
    border: none;
    color: #000;
    width: 30px;
    height: 30px;
    font-weight: bold;
    cursor: pointer;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 2px;
    font-size: 1rem;
    font-family: 'JetBrains Mono', monospace;
}

.add-btn:hover {
    opacity: 0.9;
}
</style>
