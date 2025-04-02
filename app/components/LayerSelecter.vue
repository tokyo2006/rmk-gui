<script setup lang="ts">
const { keyboard } = useKeyboardStore();
const pageKeymap = usePageKeymap();
const selectedLayer = ref(0);
watch(selectedLayer, async (newValue: number) => {
  pageKeymap.selectedLayer = newValue;
  await invoke('update_keymap');
});
</script>

<template>
  <Selector
    :items="Array.from({ length: keyboard.layer }, (_, i) => i)"
    :label="'Layer:'"
    :default="0"
    v-model="selectedLayer"
    class="h-full"
  />
</template>
