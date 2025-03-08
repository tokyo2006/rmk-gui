<script lang="ts" setup>
const props = defineProps<{
  keyProp: Key;
  border?: boolean;
}>();
const emit = defineEmits(['click']);

const keyWidth = computed(() => {
  return `calc(58px * ${props.keyProp.width[0]})`;
});
</script>

<template>
  <div class="w-[58px] h-[58px] p-1" :style="{ width: keyWidth }">
    <label
      class="flex justify-center items-center bg-base-100 h-full w-full rounded-md border shadow-sm px-1 cursor-pointer"
      :class="{ 'border-primary': props.border }"
    >
      <input type="button" @click="$emit('click')" class="hidden" :value="[keyProp.lyr_row_col]" />
      <span
        v-if="!props.keyProp.display[1]"
        class="flex justify-center text-center"
        :class="{ 'text-[10px]': keyProp.display[0].length > 1 }"
      >
        {{ keyProp.display[0] }}
      </span>
      <span v-else class="flex flex-col justify-center">
        <p class="flex justify-center items-center text-xs">{{ keyProp.display[0] }}</p>
        <p class="flex justify-center items-center text-xs">{{ keyProp.display[1] }}</p>
      </span>
    </label>
  </div>
</template>
