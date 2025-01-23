<script lang="ts" setup>
const layout = ref<any[]>();
layout.value = await invoke('get_layout_keymap');
let keycode: any[] = await invoke('get_keycode_list');

let currKey:any;
const logKey = (e: any) => {
  currKey = e;
};

const setKeycode = async (key: any) => {
  console.log(key, currKey);
  await invoke('set_keycode', {
    layer: currKey.layer,
    row: currKey.row,
    col: currKey.col,
    keycode: key[1],
  });
  await invoke('update_keymap');
  layout.value = await invoke('get_layout_keymap');
  console.log(layout.value);
}
</script>

<template>
  <div class="flex flex-col h-0">
    <KeyBoard @selected="logKey" v-model="layout"></KeyBoard>
    <div class="flex-grow max-h-full flex flex-wrap justify-between w-10/12 mx-auto mt-8 overflow-y-auto overflow-x-hidden">
        <Key v-for="key in keycode" @onClick="setKeycode(key)" type="button" :key="key.keycode" :lower="key[0]" radioGroup="1"/>
    </div>
  </div>
</template>
