<template>
    <div class="time-picker-overlay" @click.self="$emit('close')">
        <div class="time-picker">
            <div class="columns">
                <div class="column" ref="hoursColumn" @scroll="handleScroll('hours', $event)">
                    <div class="spacer"></div>
                    <div 
                        v-for="h in 24" 
                        :key="h-1" 
                        class="item"
                        :class="{ active: selectedHour === (h-1) }"
                        @click="scrollTo(hoursColumn, h-1)"
                    >
                        {{ (h-1).toString().padStart(2, '0') }}
                    </div>
                    <div class="spacer"></div>
                </div>

                <div class="colon">:</div>

                <div class="column" ref="minutesColumn" @scroll="handleScroll('minutes', $event)">
                    <div class="spacer"></div>
                    <div 
                        v-for="m in 60" 
                        :key="m-1" 
                        class="item"
                        :class="{ active: selectedMinute === (m-1) }"
                        @click="scrollTo(minutesColumn, m-1)"
                    >
                        {{ (m-1).toString().padStart(2, '0') }}
                    </div>
                    <div class="spacer"></div>
                </div>
            </div>
            
            <div class="selection-bar"></div>

            <div class="actions">
                <button class="action-btn cancel" @click="$emit('close')">Cancel</button>
                <button class="action-btn confirm" @click="confirm">Set</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue';

const props = defineProps<{
    modelValue: string;
}>();

const emit = defineEmits(['update:modelValue', 'close']);

const selectedHour = ref(0);
const selectedMinute = ref(0);
const hoursColumn = ref<HTMLElement | null>(null);
const minutesColumn = ref<HTMLElement | null>(null);
const itemHeight = 40;

const init = () => {
    if (props.modelValue) {
        const [h, m] = props.modelValue.split(':').map(Number);
        selectedHour.value = h || 0;
        selectedMinute.value = m || 0;
    } else {
        const now = new Date();
        selectedHour.value = now.getHours();
        selectedMinute.value = now.getMinutes();
    }
    
    nextTick(() => {
        if (hoursColumn.value) {
            hoursColumn.value.scrollTop = selectedHour.value * itemHeight;
        }
        if (minutesColumn.value) {
            minutesColumn.value.scrollTop = selectedMinute.value * itemHeight;
        }
    });
};

const handleScroll = (type: 'hours' | 'minutes', event: Event) => {
    const target = event.target as HTMLElement;
    const index = Math.round(target.scrollTop / itemHeight);
    
    if (type === 'hours') {
        selectedHour.value = Math.max(0, Math.min(23, index));
    } else {
        selectedMinute.value = Math.max(0, Math.min(59, index));
    }
};

const scrollTo = (element: HTMLElement | null, index: number) => {
    if (element) {
        element.scrollTo({
            top: index * itemHeight,
            behavior: 'smooth'
        });
    }
};

const confirm = () => {
    const h = selectedHour.value.toString().padStart(2, '0');
    const m = selectedMinute.value.toString().padStart(2, '0');
    emit('update:modelValue', `${h}:${m}`);
    emit('close');
};

onMounted(() => {
    init();
});
</script>

<style scoped>
.time-picker-overlay {
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

.time-picker {
    background: #1a1a1a;
    border-radius: 12px;
    padding: 20px;
    width: 250px;
    box-shadow: 0 10px 30px rgba(0,0,0,0.5);
    border: 1px solid rgba(255,255,255,0.1);
    position: relative;
}

.columns {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 200px;
    position: relative;
    overflow: hidden;
}

.column {
    height: 100%;
    width: 60px;
    overflow-y: scroll;
    scroll-snap-type: y mandatory;
    scrollbar-width: none;
    -ms-overflow-style: none;
    position: relative;
    z-index: 2;
    text-align: center;
}

.column::-webkit-scrollbar {
    display: none;
}

.spacer {
    height: 80px;
    flex-shrink: 0;
}

.item {
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    color: #555;
    scroll-snap-align: center;
    cursor: pointer;
    transition: all 0.2s;
}

.item.active {
    color: var(--text-color-primary);
    font-size: 1.8rem;
    font-weight: bold;
}

.colon {
    color: var(--text-color-primary);
    font-size: 1.5rem;
    margin: 0 10px;
    padding-bottom: 5px;
}

.selection-bar {
    position: absolute;
    top: 50%;
    left: 20px;
    right: 20px;
    height: 40px;
    margin-top: -48px;
    border-top: 1px solid rgba(255,255,255,0.1);
    border-bottom: 1px solid rgba(255,255,255,0.1);
    pointer-events: none;
    z-index: 1;
}

.columns::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: linear-gradient(to bottom, 
        rgba(26,26,26,1) 0%, 
        rgba(26,26,26,0) 40%, 
        rgba(26,26,26,0) 60%, 
        rgba(26,26,26,1) 100%
    );
    pointer-events: none;
    z-index: 3;
}

.actions {
    display: flex;
    justify-content: space-between;
    margin-top: 20px;
}

.action-btn {
    background: none;
    border: none;
    padding: 8px 15px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 1rem;
    transition: background 0.2s;
}

.cancel {
    color: #888;
    font-family: 'JetBrains Mono', monospace;
}

.cancel:hover {
    color: var(--text-color-secondary);;
    background: rgba(255,255,255,0.05);
}

.confirm {
    color: #000;
    background: #fff;
    font-weight: bold;
    font-family: 'JetBrains Mono', monospace;
}

.confirm:hover {
    background: #eee;
}
</style>
