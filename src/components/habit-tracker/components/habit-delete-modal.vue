<template>
    <div v-if="show" class="confirm-overlay">
        <div class="confirm-box">
            <p>Delete this habit?</p>
            <div class="confirm-actions">
                <button class="cancel-btn" @click="$emit('cancel')">No</button>
                <button class="confirm-btn" @click="$emit('confirm')">Yes</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
defineProps<{
    show: boolean;
}>();

defineEmits<{
    (e: 'confirm'): void;
    (e: 'cancel'): void;
}>();
</script>

<style scoped>
.confirm-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
}

.confirm-box {
    background: #333;
    padding: 15px;
    border-radius: 8px;
    text-align: center;
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.confirm-box p {
    margin: 0 0 15px 0;
    font-size: 1rem;
    font-family: 'JetBrains Mono', monospace;
}

.confirm-actions {
    display: flex;
    gap: 10px;
    justify-content: center;
}

.confirm-btn, .cancel-btn {
    padding: 5px 15px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-family: 'JetBrains Mono', monospace;
    font-weight: bold;
}

.confirm-btn {
    background: var(--pastel-red);
    color: white;
}

.cancel-btn {
    background: #555;
    color: white;
}

.confirm-btn:hover {
    background: var(--pastel-red-hover);
}

.cancel-btn:hover {
    background: #666;
}
</style>
