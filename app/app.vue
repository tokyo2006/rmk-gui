<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';

await invoke('get_vial_devices');
await invoke('update_keymap');
</script>

<template>
    <main class="flex h-screen bg-base-200">
        <Sidebar/>
        <div class="flex-grow flex flex-col">
            <Header/>
            <NuxtPage class="flex-grow"/>
        </div>
    </main>
</template>

<style>
.page-enter-active,
.page-leave-active {
  transition: all 0.1s;
}

.page-enter-from,
.page-leave-to {
  opacity: 0;
  filter: blur(1rem);
}
</style>