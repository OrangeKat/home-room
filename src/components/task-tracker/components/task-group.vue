<template>
  <div class="task-group">
    <div class="group-header">{{ label }}</div>
    <TaskItem 
      v-for="task in tasks" 
      :key="task.id" 
      :task="task"
      @toggle="$emit('toggle', $event)"
      @remove="$emit('remove', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import type { Task } from '@/types';
import TaskItem from './task-item.vue';

defineProps<{
  label: string;
  tasks: Task[];
}>();

defineEmits<{
  (e: 'toggle', id: number): void;
  (e: 'remove', id: number): void;
}>();
</script>

<style scoped>
.group-header {
    font-size: 1rem;
    color: #888;
    margin-top: 5px;
    margin-bottom: 2px;
    padding-left: 5px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}
</style>
