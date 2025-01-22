<script lang="ts" setup>
const props = defineProps<{
  lower: string,
  radio: string,
  upper?: string,
  length?: number,
}>();

const spanWidth = computed(() => {
  return `calc(40px * ${Number(props.length) || 1})`;
});

const selectedValue = defineModel();
</script>

<template>
  <label class="flex justify-center items-center bg-base-100 h-10 min-w-10 rounded-md border shadow-sm px-1 cursor-pointer"
    :class="{'border-primary': selectedValue === props.lower}" :style="{ width: spanWidth }">
    <input type="radio" v-model="selectedValue" :name="radio" class="hidden" :value="lower"/>
    <span v-if="!props.upper" class="flex justify-center" :class="{'text-[10px]': lower.length > 1}"
      :style="{ width: spanWidth }">
      {{ lower }}
    </span>
    <span v-else class="flex justify-center" :style="{ width: spanWidth }">
      <p class="flex justify-center items-center mt-2 w-3 text-sm"> {{ lower }} </p>
      <p class="flex justify-center items-center mb-2 w-2.5 text-sm"> {{ upper }} </p>
    </span>
  </label>
</template>
