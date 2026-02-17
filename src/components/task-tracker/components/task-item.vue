<template>
  <div 
    class="task-item" 
    :class="{ completed: task.completed, urgent: task.isUrgent }"
  >
    <div class="task-time">
      {{ task.time && task.time !== '00:00' && !task.time.startsWith('00:00') ? task.time.substring(0, 5) : '--:--' }}
    </div>
    <div class="task-content">
      <div class="task-title" @click="$emit('toggle', task.id)">{{ task.title }}</div>
      <div v-if="task.isUrgent" class="urgent-badge">URGENT</div>
    </div>
    <button class="delete-btn" @click="$emit('remove', task.id)">×</button>
  </div>
</template>

<script setup lang="ts">
import type { Task } from '@/types';

defineProps<{
  task: Task;
}>();

defineEmits<{
  (e: 'toggle', id: number): void;
  (e: 'remove', id: number): void;
}>();
</script>

<style scoped>
.task-item {
    display: flex;
    align-items: center;
    gap: 10px;
    background: rgba(255, 255, 255, 0.05);
    padding: 8px;
    border-radius: 4px;
    transition: background 0.2s;
}

.task-item:hover {
    background: rgba(255, 255, 255, 0.1);
}

.task-item.urgent {
    border-left: 3px solid #ff4444;
    background: rgba(255, 68, 68, 0.1);
}

.task-time {
    font-size: 1.2rem;
    color: var(--text-color-secondary);;
    min-width: 45px;
}

.task-content {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 8px;
    overflow: hidden;
}

.task-title {
    font-size: 1.2rem;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.urgent-badge {
    font-size: 0.8rem;
    background: #ff4444;
    color: var(--text-color-primary);
    padding: 1px 4px;
    border-radius: 2px;
    letter-spacing: 1px;
}

.completed .task-title {
    text-decoration: line-through;
    opacity: 0.5;
}

.delete-btn {
    background: none;
    border: none;
    color: var(--text-color-secondary);;
    cursor: pointer;
    font-size: 1.2rem;
    padding: 0 5px;
    font-family: 'JetBrains Mono', monospace;
}

.delete-btn:hover {
    color: var(--text-color-primary);
}
</style>
