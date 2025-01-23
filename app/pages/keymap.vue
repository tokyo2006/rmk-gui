<script lang="ts" setup>
const layout: any[] = await invoke('get_layout_keymap');
let keycode: any[] = await invoke('get_keycode_list');

let currKey:any;
const logKey = (e: any) => {
  currKey = e;
};

const setKeycode = async (key: any) => {
  console.log(key, currKey);
  invoke('set_keycode', {
    layer: currKey.layer,
    row: currKey.row,
    col: currKey.col,
    keycode: key[1],
  });
  keycode = await invoke('get_keycode_list');
}
</script>

<template>
  <div class="flex flex-col h-0">
    <KeyBoard @update:modelValue="logKey" :layoutKeymap="layout"></KeyBoard>
    <div class="flex-grow max-h-full flex flex-wrap justify-between w-10/12 mx-auto mt-8 overflow-y-auto overflow-x-hidden">
        <Key v-for="key in keycode" @onClick="setKeycode(key)" type="button" :key="key.keycode" :lower="key[0]" radioGroup="1"/>
    </div>
  </div>
</template>
