<script lang="ts" setup>
const props = defineProps<{
  keyProp: Key;
  border?: boolean;
}>();
const emit = defineEmits(['click']);

const keyWidth1 = computed(() => {
  return `calc(58px * ${props.keyProp.width[0]} - 8px)`;
});
const keyWidth2 = computed(() => {
  return `calc(58px * ${props.keyProp.width[1]} - 8px)`;
});
const keyHeight1 = computed(() => {
  return `calc(58px * ${props.keyProp.height[0]} - 8px)`;
});
const keyHeight2 = computed(() => {
  return `calc(58px * ${props.keyProp.height[1]} - 8px)`;
});
const keyMaxWidth = computed(() => {
  return `calc(58px * ${props.keyProp.width[0] > props.keyProp.width[1] ? props.keyProp.width[0] : props.keyProp.width[1]})`;
});
const keyMaxHeight = computed(() => {
  return `calc(58px * ${props.keyProp.height[0] > props.keyProp.height[1] ? props.keyProp.height[0] : props.keyProp.height[1]})`;
});
</script>

<template>
  <div
    class="w-[58px] h-[58px] p-1 rounded-md cursor-pointer relative"
    :style="{ width: keyMaxWidth, height: keyMaxHeight }"
  >
    <label>
      <div
        class="rounded-md outline outline-[1px] cursor-pointer shadow-sm absolute outline-base-300"
        :style="{ width: keyWidth1, height: keyHeight1 }"
        :class="{ ' outline-primary outline-[2px]': props.border }"
      ></div>
      <div
        class="rounded-md outline outline-[1px] cursor-pointer shadow-sm absolute outline-base-300"
        :style="{
          width: keyWidth2,
          height: keyHeight2,
          top: props.keyProp.position_y[1] * 58 + 4 + 'px',
          left: props.keyProp.position_x[1] * 58 + 4 + 'px',
        }"
        :class="{ ' outline-primary outline-[2px]': props.border }"
      ></div>
      <div
        class="rounded-md bg-base-100 cursor-pointer absolute"
        :style="{
          width: keyWidth2,
          height: keyHeight2,
          top: props.keyProp.position_y[1] * 58 + 4 + 'px',
          left: props.keyProp.position_x[1] * 58 + 4 + 'px',
        }"
      ></div>
      <div
        class="rounded-md bg-base-100 cursor-pointer absolute flex justify-center items-center"
        :style="{ width: keyWidth1, height: keyHeight1 }"
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
      </div>
    </label>
  </div>
</template>
