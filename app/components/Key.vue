<script lang="ts" setup>
const props = defineProps<{
  lower: string;
  type: 'button' | 'radio';
  radioGroup?: string;
  upper?: string;
  length?: number;
  value?: any;
}>();

const keyWidth = computed(() => {
  return `calc(58px * ${props.length})`;
});

interface SelectedValue {
  keycode?: string;
}

const selectedValue = defineModel<SelectedValue>();

const emit = defineEmits(['onClick'])
const click = () => {
  emit('onClick')
}
</script>

<template>
  <div class="w-[58px] h-[58px] p-1" :style="{ width: keyWidth }">
    <label class="flex justify-center items-center bg-base-100 h-full w-full rounded-md border shadow-sm px-1 cursor-pointer"
      :class="{ 'border-primary': selectedValue === props.value && props.type === 'radio' }">
      <input v-if="props.type === 'radio'" type="radio" v-model="selectedValue" :name="radioGroup" class="hidden" :value="value" />
      <input v-if="props.type === 'button'" type="button" :name="radioGroup" @click="click()" class="hidden" :value="lower" />
      <span v-if="!props.upper" class="flex justify-center" :class="{ 'text-[10px]': lower.length > 1 }">
        {{ lower }}
      </span>
      <span v-else class="flex justify-center">
        <p class="flex justify-center items-center mt-2 w-3 text-sm">{{ lower }}</p>
        <p class="flex justify-center items-center mb-2 w-2.5 text-sm">{{ upper }}</p>
      </span>
    </label>
  </div>
</template>
