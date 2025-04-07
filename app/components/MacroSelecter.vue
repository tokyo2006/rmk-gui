<script setup lang="ts">
const { keyboard } = useKeyboardStore();
const selectedMacro = ref(0);
</script>

<template>
  <Selector
    :items="Array.from({ length: keyboard.macro }, (_, i) => i)"
    :label="'Macro:'"
    :default="0"
    v-model="selectedMacro"
    class="h-full"
  />
</template>
