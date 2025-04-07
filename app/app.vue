<script lang="ts" setup>
const hidDevicesStore = useHidDevicesStore();
hidDevicesStore.devices = await invoke<{ product_string: string; path: number[] }[]>('get_vial_devices').catch((e) => {
  showErrorToast(e);
  return [];
});
</script>

<template>
  <main class="flex h-screen bg-base-200">
    <Sidebar />
    <div class="flex-grow flex flex-col">
      <Header />
      <NuxtPage class="flex-grow" />
    </div>
    <Toaster />
  </main>
</template>
