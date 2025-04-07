export const usePageMacro = defineStore('PageMacro', () => {
  const selectedMacro = ref(0);
  return { selectedMacro };
});
