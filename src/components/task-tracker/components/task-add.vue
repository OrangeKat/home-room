<template>
  <div class="add-task">
    <div class="input-row single-line">
      <button class="date-btn compact" @click="showDatePicker = true">
        {{ formattedDateCompact }}
      </button>

      <button class="time-btn compact" @click="showTimePicker = true">
        {{ newTaskTime ? newTaskTime : '--:--' }}
      </button>

      <div class="input-wrapper">
        <input 
          v-model="newTaskTitle" 
          type="text" 
          placeholder="New Task" 
          class="text-input"
          @keyup.enter="handleAddTask"
        />
        <button 
          class="urgent-toggle-inline" 
          :class="{ active: isUrgentInput }" 
          @click="isUrgentInput = !isUrgentInput"
          title="Mark as Urgent"
        >
          !
        </button>
      </div>

      <button class="add-btn" @click="handleAddTask">+</button>
    </div>

    <TimePicker 
      v-if="showTimePicker" 
      v-model="newTaskTime" 
      @close="showTimePicker = false" 
    />
    <DatePicker
      v-if="showDatePicker"
      v-model="newTaskDate"
      @close="showDatePicker = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import TimePicker from './time-picker.vue';
import DatePicker from './date-picker.vue';

const props = defineProps<{
  isTauri: boolean;
}>();

const emit = defineEmits<{
  (e: 'add', task: { title: string, date: string, time: string, isUrgent: boolean }): void;
}>();

const newTaskTitle = ref('');
const newTaskDate = ref(new Date().toISOString().split('T')[0]);
const newTaskTime = ref('');
const isUrgentInput = ref(false);
const showTimePicker = ref(false);
const showDatePicker = ref(false);

const formattedDateCompact = computed(() => {
    if (!newTaskDate.value) return 'Date';
    const d = new Date(newTaskDate.value);
    const today = new Date().toISOString().split('T')[0];
    const tomorrow = new Date(Date.now() + 86400000).toISOString().split('T')[0];
    
    if (newTaskDate.value === today) return 'Today';
    if (newTaskDate.value === tomorrow) return 'Tmrrw';
    
    return d.toLocaleDateString(undefined, { month: 'numeric', day: 'numeric' });
});

const handleAddTask = () => {
  if (!newTaskTitle.value || !newTaskDate.value) return;

  if (!props.isTauri) {
    alert('Cannot add tasks in browser mode. Please run the Tauri desktop app.');
    return;
  }

  emit('add', {
    title: newTaskTitle.value,
    date: newTaskDate.value,
    time: newTaskTime.value,
    isUrgent: isUrgentInput.value,
  });

  newTaskTitle.value = '';
  newTaskTime.value = '';
  isUrgentInput.value = false;
};
</script>

<style scoped>
.add-task {
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.input-row {
    display: flex;
    gap: 1px;
    align-items: center;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    padding: 2px;
}

.date-btn.compact, .time-btn.compact {
    width: auto;
    min-width: 50px;
    padding: 8px 12px;
    font-size: 0.8rem;
    background: none;
    border: none;
    color: #ccc;
    border-right: 1px solid rgba(255,255,255,0.1);
    border-radius: 0;
    cursor: pointer;
    font-family: 'JetBrains Mono', monospace;
}

.date-btn.compact:hover, .time-btn.compact:hover {
    background: rgba(255,255,255,0.05);
    color: var(--text-color-primary);
}

.input-wrapper {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
    height: 100%;
}

.text-input {
    background: none;
    border: none;
    color: var(--text-color-primary);
    padding: 8px 30px 8px 12px;
    font-size: 0.9rem;
    outline: none;
    width: 100%;
    height: 100%;
    font-family: 'JetBrains Mono', monospace;
}

.urgent-toggle-inline {
    position: absolute;
    right: 5px;
    background: none;
    border: none;
    color: #555;
    font-weight: bold;
    cursor: pointer;
    font-size: 1.2rem;
    transition: color 0.2s;
    padding: 0;
    font-family: 'JetBrains Mono', monospace;
}

.urgent-toggle-inline:hover {
    color: #888;
}

.urgent-toggle-inline.active {
    color: var(--pastel-red);;
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
