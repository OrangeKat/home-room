<template>
    <div class="sidebar">
        <div class="menu-items">
            <button class="icon-btn settings-btn">
                <img src="@/assets/icons/settings.svg" alt="Settings" class="icon" />
                <span class="label">Settings</span>
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.sidebar {
    position: fixed;
    left: 0;
    top: 0;
    height: 100vh;
    width: 60px;
    border-right: 1px solid rgba(255, 255, 255, 0.1);
    transition: width 0.3s ease;
    overflow: hidden;
    z-index: 1000;
    padding-top: 20px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
}

.sidebar:hover {
    width: 200px;
    background: #222;
    transition: width 0.3s ease;
}

.menu-items {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
}

.icon-btn {
    background: none;
    border: none;
    color: #888;
    cursor: pointer;
    display: flex;
    align-items: center;
    padding: 10px;
    width: calc(100% - 16px);
    margin: 0 8px;
    border-radius: 8px;
    transition: color 0.2s, background 0.2s;
    font-family: inherit;
    white-space: nowrap;
}

.icon-btn:hover {
    color: var(--text-color-primary);
    background: rgba(255, 255, 255, 0.05);
}

.icon-btn .icon {
    width: 24px;
    height: 24px;
    margin-right: 15px;
    filter: invert(0.5);
    transition: filter 0.2s;
}

.icon-btn:hover .icon {
    filter: invert(1);
}

.label {
    opacity: 0;
    transition: opacity 0s;
    font-size: 1rem;
}

.sidebar:hover .label {
    opacity: 1;
    transition: opacity 0.2s ease 0.3s;
}
</style>
