<script lang="ts" setup>
import { Serial } from '@ijprest/kle-serial';

const layout: any[] = await invoke('get_layout_keymap');
</script>

<template>
  <KeyBoard :layoutKeymap="layout"></KeyBoard>
</template>
