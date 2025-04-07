<script setup lang="ts">
const { keyboard } = useKeyboardStore();
const pageKeymap = usePageKeymap();
const { selectedLayer } = storeToRefs(pageKeymap);
watch(selectedLayer, async () => {
  await invoke('update_keymap').catch(showErrorToast);
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
