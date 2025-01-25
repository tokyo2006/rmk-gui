<script lang="ts" setup>
const hidDevicesStore = useHidDevicesStore();
hidDevicesStore.devices = await invoke('get_vial_devices');
</script>

<template>
  <main class="flex h-screen bg-base-200">
    <Sidebar />
    <div class="flex-grow flex flex-col">
      <Header />
      <NuxtPage class="flex-grow" />
    </div>
  </main>
</template>
