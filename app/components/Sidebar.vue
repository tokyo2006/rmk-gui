<script lang="ts" setup>
const router = useRouter();

const selectedPage = ref('/');
watch(selectedPage, (newValue) => {
  router.push(newValue);
});
</script>

<template>
  <div class="bg-base-100 min-w-48 border-r border-base-300">
    <img src="~/assets/rmk_logo.png" class="h-16 mx-auto">
    <label class="custom-option py-3 flex w-36 mx-auto mt-2 border-base-100">
      <input type="radio" name="radio-17" class="radio hidden" value="/" v-model="selectedPage" checked />
      <div class="flex justify-center items-center w-full">
        <span class="icon-[material-symbols--home] text-2xl mr-3" />
        <span class="w-15 font-bold">Home</span>
      </div>
    </label>
    <label class="custom-option py-3 flex w-36 mx-auto mt-2 border-base-100">
      <input type="radio" name="radio-17" class="radio hidden" value="keymap" v-model="selectedPage" />
      <div class="flex justify-center items-center w-full">
        <span class="icon-[material-symbols--format-shapes-rounded] text-2xl mr-3" />
        <span class="w-15 font-bold"> Keymap </span>
      </div>
    </label>
    <label class="custom-option py-3 flex w-36 mx-auto mt-2 border-base-100">
      <input type="radio" name="radio-17" class="radio hidden" value="macro" v-model="selectedPage" />
      <div class="flex justify-center items-center w-full">
        <span class="icon-[material-symbols--autoplay] text-2xl mr-3" />
        <span class="w-15 font-bold line-through"> Macro </span>
      </div>
    </label>
    <label class="custom-option py-3 flex w-36 mx-auto mt-2 border-base-100">
      <input type="radio" name="radio-17" class="radio hidden" value="ota" v-model="selectedPage" />
      <div class="flex justify-center items-center w-full">
        <span class="icon-[material-symbols--arrow-circle-up] text-2xl mr-3" />
        <span class="w-15 font-bold line-through"> OTA </span>
      </div>
    </label>
    <label class="custom-option py-3 flex w-36 mx-auto mt-2 border-base-100">
      <input type="radio" name="radio-17" class="radio hidden" value="setting" v-model="selectedPage" />
      <div class="flex justify-center items-center w-full">
        <span class="icon-[mdi--cog] text-2xl mr-3" />
        <span class="w-15 font-bold line-through"> Setting </span>
      </div>
    </label>
  </div>
</template>
