<script lang="ts" setup>
const hidDevicesStore = useHidDevicesStore();
const selectedDevicePath = ref();
watch(selectedDevicePath, async (newValue) => {
  await invoke('connect_vial_device', { path: newValue });
  await invoke('update_keymap');
  console.log(await invoke('get_key_count'));
});
if (hidDevicesStore.devices.length > 0) {
  selectedDevicePath.value = hidDevicesStore.devices[0]?.path;
} else {
  selectedDevicePath.value = 'Device not found';
}
</script>

<template>
  <div class="flex justify-start items-center bg-base-100 h-16 border-b border-base-300">
    <select class="focus:outline-none ml-2" v-model="selectedDevicePath">
      <option v-for="device in hidDevicesStore.devices" :key="device.product_string" :value="device.path">
        {{ device.product_string }}
      </option>
    </select>
  </div>
</template>
