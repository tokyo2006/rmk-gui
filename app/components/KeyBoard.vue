<script lang="ts" setup>
const props = defineProps<{
  layoutKeymap: any[];
}>();
console.log(props.layoutKeymap);
const selectedValue = ref();

const emit = defineEmits(['update:modelValue']);
watch(selectedValue, () => {
  emit('update:modelValue', selectedValue.value);
});

const containerSize = computed(() => {
  let maxWidth = 0;
  let maxHeight = 0;

  props.layoutKeymap.forEach(key => {
    const rightEdge = key.position_x[0] * 58 + key.width[0] * 58;
    const bottomEdge = key.position_y[0] * 58 + 58; // 假设每个 Key 的高度是 58px
    maxWidth = Math.max(maxWidth, rightEdge);
    maxHeight = Math.max(maxHeight, bottomEdge);
  });

  return { width: maxWidth, height: maxHeight };
});
</script>

<template>
  <div>
    <div
      class="relative mx-auto mt-4 bg-base-100 outline outline-4 outline-base-100 rounded"
      :style="{
        height: `${containerSize.height}px`,
        width: `${containerSize.width}px`
      }"
    >
      <Key
        v-for="(key, index) in layoutKeymap"
        :key="index"
        :keymap="key"
        v-model="selectedValue"
        type="radio"
        :lower="key.keycode"
        :length="key.width[0]"
        :value="key"
        :radioGroup="key.keycode"
        :style="{
          position: 'absolute',
          left: key.position_x[0] * 58 + 'px',
          top: key.position_y[0] * 58 + 'px',
        }"
      />
    </div>
  </div>
</template>
