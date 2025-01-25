<script lang="ts" setup>
const hidDevicesStore = useHidDevicesStore();
const selectedDevicePath = ref();
watch(selectedDevicePath, async (newValue) => {
  await invoke('connect_vial_device', { path: newValue });
});
</script>

<template>
  <div class="flex justify-start items-center bg-base-100 h-16 border-b border-base-300">
    <select class="focus:outline-none ml-2" :v-bind="selectedDevicePath">
      <option v-for="device in hidDevicesStore.devices" :key="device.product_string" :value="device.path">
        {{ device.product_string }}
      </option>
    </select>
  </div>
</template>
