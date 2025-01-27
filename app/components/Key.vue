<script lang="ts" setup>
const props = defineProps<{
  keyProp: Key;
}>();

const keyWidth = computed(() => {
  return `calc(58px * ${props.keyProp.width[0]})`;
});

const selectedLyrRowCol = defineModel<[number, number, number]>();

const emit = defineEmits(['onClick']);
const click = () => {
  emit('onClick');
};
console.log(props.keyProp.display.length == 1);
</script>

<template>
  <div class="w-[58px] h-[58px] p-1" :style="{ width: keyWidth }">
    <label
      class="flex justify-center items-center bg-base-100 h-full w-full rounded-md border shadow-sm px-1 cursor-pointer"
      :class="{ 'border-primary': selectedLyrRowCol === props.keyProp.lyr_row_col }"
    >
      <input type="button" @click="click()" class="hidden" :value="[keyProp.lyr_row_col]" />
      <span
        v-if="!props.keyProp.display[1]"
        class="flex justify-center"
        :class="{ 'text-[10px]': keyProp.display[0].length > 1 }"
      >
        {{ keyProp.display[0] }}
      </span>
      <span v-else class="flex justify-center">
        <p class="flex justify-center items-center mt-2 w-3 text-sm">{{ keyProp.display[1] }}</p>
        <p class="flex justify-center items-center mb-2 w-2.5 text-sm">{{ keyProp.display[0] }}</p>
      </span>
    </label>
  </div>
</template>
