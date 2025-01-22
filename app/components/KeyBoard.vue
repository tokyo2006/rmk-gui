<script lang="ts" setup>
const props = defineProps<{
  layoutKeymap: any[],
}>();
console.log(props.layoutKeymap);
const selectedValue = ref('');
watch(selectedValue,() => {
  console.log('update:modelValue', selectedValue.value);
});
</script>

<template>
  <div class="flex flex-col w-[650px] gap-1">
    <div class="flex justify-between w-full">
      <Key v-model="selectedValue" radio="full" lower="Esc" />
      <Key v-model="selectedValue" radio="full" lower="F1" />
      <Key v-model="selectedValue" radio="full" lower="F2" />
      <Key v-model="selectedValue" radio="full" lower="F3" />
      <Key v-model="selectedValue" radio="full" lower="F4" />
      <Key v-model="selectedValue" radio="full" lower="F5" />
      <Key v-model="selectedValue" radio="full" lower="F6" />
      <Key v-model="selectedValue" radio="full" lower="F7" />
      <Key v-model="selectedValue" radio="full" lower="F8" />
      <Key v-model="selectedValue" radio="full" lower="F9" />
      <Key v-model="selectedValue" radio="full" lower="F10" />
      <Key v-model="selectedValue" radio="full" lower="F11" />
      <Key v-model="selectedValue" radio="full" lower="F12" />
    </div>

    <div class="flex justify-between w-full">
      <Key v-model="selectedValue" radio="full" lower="`" upper="~" />
      <Key v-model="selectedValue" radio="full" lower="1" upper="!" />
      <Key v-model="selectedValue" radio="full" lower="2" upper="@" />
      <Key v-model="selectedValue" radio="full" lower="3" upper="#" />
      <Key v-model="selectedValue" radio="full" lower="4" upper="$" />
      <Key v-model="selectedValue" radio="full" lower="5" upper="%" />
      <Key v-model="selectedValue" radio="full" lower="6" upper="^" />
      <Key v-model="selectedValue" radio="full" lower="7" upper="&" />
      <Key v-model="selectedValue" radio="full" lower="8" upper="*" />
      <Key v-model="selectedValue" radio="full" lower="9" upper="(" />
      <Key v-model="selectedValue" radio="full" lower="0" upper=")" />
      <Key v-model="selectedValue" radio="full" lower="-" upper="-" />
      <Key v-model="selectedValue" radio="full" lower="=" upper="+" />
      <Key v-model="selectedValue" radio="full" lower="BACKSPACE" length=2 />
    </div>

    <div class="flex justify-between w-full">
      <Key v-model="selectedValue" radio="full" lower="TAB" length=1.5 />
      <Key v-model="selectedValue" radio="full" lower="Q" />
      <Key v-model="selectedValue" radio="full" lower="W" />
      <Key v-model="selectedValue" radio="full" lower="E" />
      <Key v-model="selectedValue" radio="full" lower="R" />
      <Key v-model="selectedValue" radio="full" lower="T" />
      <Key v-model="selectedValue" radio="full" lower="Y" />
      <Key v-model="selectedValue" radio="full" lower="U" />
      <Key v-model="selectedValue" radio="full" lower="I" />
      <Key v-model="selectedValue" radio="full" lower="O" />
      <Key v-model="selectedValue" radio="full" lower="P" />
      <Key v-model="selectedValue" radio="full" lower="[" upper="{" />
      <Key v-model="selectedValue" radio="full" lower="]" upper="}" />
      <Key v-model="selectedValue" radio="full" lower="\" upper="|" length=1.5 />
    </div>

    <div class="flex justify-between w-full">
      <Key v-model="selectedValue" radio="full" lower="CAPSLOCK" length=1.75 />
      <Key v-model="selectedValue" radio="full" lower="A" />
      <Key v-model="selectedValue" radio="full" lower="S" />
      <Key v-model="selectedValue" radio="full" lower="D" />
      <Key v-model="selectedValue" radio="full" lower="F" />
      <Key v-model="selectedValue" radio="full" lower="G" />
      <Key v-model="selectedValue" radio="full" lower="H" />
      <Key v-model="selectedValue" radio="full" lower="J" />
      <Key v-model="selectedValue" radio="full" lower="K" />
      <Key v-model="selectedValue" radio="full" lower="L" />
      <Key v-model="selectedValue" radio="full" lower=";" upper=":" />
      <Key v-model="selectedValue" radio="full" lower="'" upper="'" />
      <Key v-model="selectedValue" radio="full" lower="ENTER" length=2.25 />
    </div>

    <div class="flex justify-between w-full">
      <Key v-model="selectedValue" radio="full" lower="L SHIFT" length=2.25 />
      <Key v-model="selectedValue" radio="full" lower="Z" />
      <Key v-model="selectedValue" radio="full" lower="X" />
      <Key v-model="selectedValue" radio="full" lower="C" />
      <Key v-model="selectedValue" radio="full" lower="V" />
      <Key v-model="selectedValue" radio="full" lower="B" />
      <Key v-model="selectedValue" radio="full" lower="N" />
      <Key v-model="selectedValue" radio="full" lower="M" />
      <Key v-model="selectedValue" radio="full" lower="," upper="<" />
      <Key v-model="selectedValue" radio="full" lower="." upper=">" />
      <Key v-model="selectedValue" radio="full" lower="/" upper="?" />
      <Key v-model="selectedValue" radio="full" lower="R SHIFT" length=2.75 />
    </div>

    <div class="flex justify-between w-full">
      <Key v-model="selectedValue" radio="full" lower="L CTRL" length=1.25 />
      <Key v-model="selectedValue" radio="full" lower="WIN" length=1.25 />
      <Key v-model="selectedValue" radio="full" lower="L ALT" length=1.25 />
      <Key v-model="selectedValue" radio="full" lower="SPACE" length=6.25 />
      <Key v-model="selectedValue" radio="full" lower="R ALT" length=1.25 />
      <Key v-model="selectedValue" radio="full" lower="WIN" length=1.25 />
      <Key v-model="selectedValue" radio="full" lower="MENU" length=1.25 />
      <Key v-model="selectedValue" radio="full" lower="R CTRL" length=1.25 />
    </div>
  </div>
</template>