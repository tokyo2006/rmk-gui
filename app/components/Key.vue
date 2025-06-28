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
const translate = computed(() => {
  return (
    `calc((-${props.keyProp.position_x[0]} + ${props.keyProp.rotation[1]}) * 58px)` +
    `calc((-${props.keyProp.position_y[0]} + ${props.keyProp.rotation[2]}) * 58px)`
  );
});

// Add computed properties for dynamic font sizing
const primaryTextClass = computed(() => {
  const text = props.keyProp.display[0];
  const secondaryText = props.keyProp.display[1];

  // For single line text
  if (!secondaryText) {
    const length = text.length;
    if (length <= 1) return 'text-sm';
    if (length <= 3) return 'text-xs';
    if (length <= 5) return 'text-[10px]';
    if (length <= 8) return 'text-[8px]';
    return 'text-[7px]';
  }

  // For two-line text, calculate combined length to ensure consistent sizing
  const totalLength = text.length + secondaryText.length;
  if (totalLength <= 4) return 'text-xs';
  if (totalLength <= 8) return 'text-[10px]';
  if (totalLength <= 12) return 'text-[8px]';
  return 'text-[7px]';
});

const secondaryTextClass = computed(() => {
  // Always use the same class as primary text for consistency
  return primaryTextClass.value;
});
</script>

<template>
  <div
    class="w-[58px] h-[58px] p-1 rounded-md cursor-pointer relative"
    :style="{
      width: keyMaxWidth,
      height: keyMaxHeight,
      transform: `rotate(${props.keyProp.rotation[0]}deg)`,
      transformOrigin: translate,
    }"
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
          class="flex justify-center items-center text-center w-full h-full px-1 leading-tight"
          :class="primaryTextClass"
          style="word-break: break-all; overflow-wrap: break-word; hyphens: auto; white-space: normal"
        >
          {{ keyProp.display[0] }}
        </span>
        <span v-else class="flex flex-col justify-center items-center w-full h-full px-1 leading-none gap-0">
          <p
            class="flex justify-center items-center text-center w-full"
            :class="primaryTextClass"
            style="word-break: break-all; overflow-wrap: break-word; hyphens: auto; white-space: normal"
          >
            {{ keyProp.display[0] }}
          </p>
          <p
            class="flex justify-center items-center text-center w-full"
            :class="secondaryTextClass"
            style="word-break: break-all; overflow-wrap: break-word; hyphens: auto; white-space: normal"
          >
            {{ keyProp.display[1] }}
          </p>
        </span>
      </div>
    </label>
  </div>
</template>
