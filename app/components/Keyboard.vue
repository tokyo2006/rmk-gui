<template>
  <div class="keyboard-container">
    <div
      v-for="(row, rowIndex) in keyboardLayout"
      :key="`row-${rowIndex}`"
      class="keyboard-row"
    >
      <button
        v-for="(key, keyIndex) in row"
        :key="`key-${rowIndex}-${keyIndex}`"
        :class="['keyboard-key', { 'active': activeKeys.includes(key) }]"
        @mousedown="handleKeydown(key)"
        @mouseup="handleKeyup(key)"
        @mouseleave="handleKeyup(key)"
      >
        {{ key }}
      </button>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      keyboardLayout: [
        // 数字行
        ['~', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 'Backspace'],
        // 第一行
        ['Tab', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\\', 'Del'],
        // 第二行
        ['CapsLock', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ';', "'", 'Enter'],
        // 第三行
        ['Shift', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '/', 'Shift'],
        // 控制行
        ['Ctrl', 'Win', 'Alt', 'Space', 'Alt', 'Ctrl']
      ],
      activeKeys: [] // 存储被按下的键
    };
  },
  methods: {
    handleKeydown(key) {
      if (!this.activeKeys.includes(key)) {
        this.activeKeys.push(key);
        this.$emit('key-press', key);
      }
    },
    handleKeyup(key) {
      this.activeKeys = this.activeKeys.filter(k => k !== key);
    }
  }
};
</script>

<style>
.keyboard-container {
  /* 样式根据自己的需求来设置 */
}
.keyboard-row {
  display: flex;
}
.keyboard-key {
  padding: 10px;
  margin: 5px;
  border: 1px solid #000;
  border-radius: 5px;
  cursor: pointer;
}
.keyboard-key.active {
  background-color: #555;
  color: #fff;
}
</style>
