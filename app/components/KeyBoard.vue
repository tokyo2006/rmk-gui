<script lang="ts" setup>
const props = defineProps<{
  layoutKeymap: any[];
}>();
console.log(props.layoutKeymap);
const selectedValue = ref('');
watch(selectedValue, () => {
  console.log('update:modelValue', selectedValue.value);
});
</script>

<template>
  <div>
    <div class="relative bg-red-200">
      <Key
        v-for="(key, index) in layoutKeymap"
        :key="index"
        :keymap="key"
        v-model="selectedValue"
        :lower="key.keycode"
        :length="key.width[0]"
        :radio="key.keycode"
        :style="{
          position: 'absolute',
          left: key.position_x[0] * 2.7 + 'em',
          top: key.position_y[0] * 2.7 + 'em',
        }"
      />
    </div>
  </div>
</template>
