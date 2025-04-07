<script lang="ts" setup>
import MacroSelecter from '~/components/MacroSelecter.vue';
definePageMeta({
  headerComponent: markRaw(MacroSelecter),
});

const macros = await invoke<any[]>('get_marcoes').catch((e) => {
  showErrorToast(e);
  return [[]];
});
</script>

<template>
  <div class="m-2 flex flex-col gap-2">
    <MacroGroup v-for="macro in macros[0]" :action="macro" />
  </div>
</template>
