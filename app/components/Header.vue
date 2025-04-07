<script lang="ts" setup>
const hidDevicesStore = useHidDevicesStore();
const selectedDevicePath = ref();

watch(selectedDevicePath, async (newValue: string) => {
  await invoke('connect_vial_device', { path: eval('[' + newValue + ']') }).catch(showErrorToast);
  await invoke('update_keymap').catch(showErrorToast);
});

if (hidDevicesStore.devices.length > 0) {
  selectedDevicePath.value = hidDevicesStore.devices[0]?.path.toString();
} else {
  showErrorToast('No keyboard found');
}

const route = useRoute();
const componentName = computed(() => route.meta.headerComponent);
</script>

<template>
  <div class="flex justify-start gap-3 items-center bg-base-100 h-16 border-b border-base-300 p-3">
    <Select v-model="selectedDevicePath">
      <SelectTrigger class="w-48 h-full">
        <SelectValue placeholder="Select a Keyboard" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectItem
            v-for="device in hidDevicesStore.devices"
            :key="device.product_string"
            :value="device.path.toString()"
          >
            {{ device.product_string }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
    <component :is="componentName" />
  </div>
</template>
