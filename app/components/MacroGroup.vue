<script setup lang="ts">
const props = defineProps<{
  action: Object;
}>();
const actionType = Object.keys(props.action)[0];
let actionValue = Object.values(props.action)[0];
if (actionType === 'Tap' || actionType === 'Up' || actionType === 'Down') {
  for (let i = 0; i < actionValue.length; i++) {
    actionValue[i] = await invoke<Key>('get_key_from_keycode', { keycode: actionValue[i] }).catch((e) => {
      showErrorToast(e);
      return { keycode: 0, display: '' };
    });
  }
}
console.log(actionType, actionValue);
</script>

<template>
  <div class="bg-base-100 w-full h-16 rounded-lg flex items-center py-1">
    <Icon class="text-3xl text-base-300 mx-2 cursor-move" name="material-symbols:drag-indicator" />
    <span class="w-12 text-center">{{ actionType }}</span>
    <div class="h-full w-0.5 bg-base-200 mx-4"></div>
    <Input v-if="actionType === 'Text'" v-model="actionValue" class="w-96" />
    <NumberField v-if="actionType === 'Delay'" class="w-96">
      <NumberFieldContent>
        <NumberFieldDecrement />
        <NumberFieldInput :value="actionValue" />
        <NumberFieldIncrement />
      </NumberFieldContent>
    </NumberField>
    <template v-if="actionType === 'Tap' || actionType === 'Up' || actionType === 'Down'">
      <Key v-for="key in actionValue" :keyProp="key" />
    </template>
    <Icon class="text-2xl ml-auto mr-2 text-base-300" name="material-symbols:close-small-outline-rounded" />
  </div>
</template>
