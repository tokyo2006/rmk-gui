<script lang="ts" setup>
const layout = ref<any[]>();
layout.value = await invoke<Key[]>('get_layout_keymap');
console.log(layout.value);
let keycode: Key[] = await invoke('get_keycode_list');

let currLyrRowCol: [number, number, number];
const logKey = (e: any) => {
  currLyrRowCol = e;
};

const setKeycode = async (key: Key) => {
  console.log(key, currLyrRowCol);
  await invoke('set_keycode', {
    layer: currLyrRowCol[0],
    row: currLyrRowCol[1],
    col: currLyrRowCol[2],
    keycode: key.keycode,
  });
  await invoke('update_keymap');
  layout.value = await invoke<Key[]>('get_layout_keymap');
};
</script>

<template>
  <div class="flex flex-col h-0">
    <KeyBoard @updateLyrRowCol="logKey" v-model="layout"></KeyBoard>
    <div
      class="flex-grow max-h-full flex flex-wrap justify-between w-10/12 mx-auto mt-8 overflow-y-auto overflow-x-hidden"
    >
      <Key v-for="key in keycode" :keyProp="key" @click="setKeycode(key)"/>
    </div>
  </div>
</template>
