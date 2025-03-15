<script lang="ts" setup>
const hidDevicesStore = useHidDevicesStore();
const selectedDevicePath = ref();
watch(selectedDevicePath, async (newValue: string) => {
  await invoke('connect_vial_device', { path: eval(newValue) });
  await invoke('update_keymap');
});
if (hidDevicesStore.devices.length > 0) {
  selectedDevicePath.value = hidDevicesStore.devices[0]?.path.toString();
} else {
  selectedDevicePath.value = 'Device not found';
}
</script>

<template>
  <div class="flex justify-start items-center bg-base-100 h-16 border-b border-base-300">
    <Select v-model="selectedDevicePath">
      <SelectTrigger class="w-64 ml-2">
        <SelectValue placeholder="Select a Keyboard"/>
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectItem v-for="device in hidDevicesStore.devices" :key="device.product_string" :value="device.path.toString()"> {{device.product_string}} </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  </div>
</template>
