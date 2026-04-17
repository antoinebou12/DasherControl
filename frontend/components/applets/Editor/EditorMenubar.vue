<template>
  <div class="menubar">
    <n-button size="tiny" quaternary :type="editor.isActive('bold') ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleBold().run())">
      <i class="bx bx-bold" />
    </n-button>
    <n-button size="tiny" quaternary :type="editor.isActive('italic') ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleItalic().run())">
      <i class="bx bx-italic" />
    </n-button>
    <n-button size="tiny" quaternary :type="editor.isActive('strike') ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleStrike().run())">
      <i class="bx bx-strikethrough" />
    </n-button>
    <n-button size="tiny" quaternary :type="editor.isActive('code') ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleCode().run())">
      <i class="bx bx-code" />
    </n-button>
    <n-button size="tiny" quaternary :type="editor.isActive('heading', { level: 1 }) ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleHeading({ level: 1 }).run())">H1</n-button>
    <n-button size="tiny" quaternary :type="editor.isActive('heading', { level: 2 }) ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleHeading({ level: 2 }).run())">H2</n-button>
    <n-button size="tiny" quaternary :type="editor.isActive('bulletList') ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleBulletList().run())">
      <i class="bx bx-list-ul" />
    </n-button>
    <n-button size="tiny" quaternary :type="editor.isActive('orderedList') ? 'primary' : 'default'" @click="run(() => editor.chain().focus().toggleOrderedList().run())">
      <i class="bx bx-list-ol" />
    </n-button>
    <n-button size="tiny" quaternary @click="run(() => editor.chain().focus().setHorizontalRule().run())">hr</n-button>
    <n-button size="tiny" quaternary @click="run(() => editor.chain().focus().undo().run())"><i class="bx bx-undo" /></n-button>
    <n-button size="tiny" quaternary @click="run(() => editor.chain().focus().redo().run())"><i class="bx bx-redo" /></n-button>
  </div>
</template>

<script setup lang="ts">
import type { Editor } from "@tiptap/core";
import { NButton } from "naive-ui";

defineProps<{
  editor: Editor;
  run: (fn: () => void) => void;
}>();
</script>

<style lang="scss" scoped>
.menubar {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-bottom: 6px;

  :deep(.n-button) {
    background-color: var(--fg);
    color: var(--bg);
    border: var(--bg) 1px solid;
  }
}
</style>
