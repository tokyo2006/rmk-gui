export const useHidDevicesStore = defineStore('HidDevicesStore', () => {
  const devices = ref<VialDevice[]>([]);
  const pathString = computed(() => devices.value.map((device) => device.path.join(':')).join(';'));
  return { devices, pathString };
});
