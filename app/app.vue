<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core';

console.log(await invoke('get_vial_devices'));
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