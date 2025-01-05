<script lang="ts" setup>
const props = defineProps<{
  lower: String,
  upper?: String,
  length?: String,
}>()

const spanWidth = computed(() => {
  return `calc(40px * ${Number(props.length) || 1})`;
});
</script>

<template>
  <span v-if="!props.upper"
    class="flex bg-base-200 h-10 min-w-10 px-1 border-2 shadow-sm justify-center items-center rounded-md"
    :class="{ 'text-xs': lower.length > 1 }" :style="{ width: spanWidth }">
    {{ lower }}
  </span>
  <span v-else class="flex bg-base-200 h-10 min-w-10 px-1 border-2 shadow-sm justify-center items-center rounded-md"
    :style="{ width: spanWidth }">
    <p class="flex justify-center items-center mt-2 w-3"> {{ lower }} </p>
    <p class="flex justify-center items-center mb-2 w-2.5 text-sm"> {{ upper }} </p>
  </span>
</template>