<template>
  <div class="menubar">
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('bold') ? 'flat' : 'tonal'"
      :color="editor.isActive('bold') ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleBold().run())"
    >
      <i class="bx bx-bold" />
    </v-btn>
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('italic') ? 'flat' : 'tonal'"
      :color="editor.isActive('italic') ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleItalic().run())"
    >
      <i class="bx bx-italic" />
    </v-btn>
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('strike') ? 'flat' : 'tonal'"
      :color="editor.isActive('strike') ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleStrike().run())"
    >
      <i class="bx bx-strikethrough" />
    </v-btn>
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('code') ? 'flat' : 'tonal'"
      :color="editor.isActive('code') ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleCode().run())"
    >
      <i class="bx bx-code" />
    </v-btn>
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('heading', { level: 1 }) ? 'flat' : 'tonal'"
      :color="editor.isActive('heading', { level: 1 }) ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleHeading({ level: 1 }).run())"
    >
      H1
    </v-btn>
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('heading', { level: 2 }) ? 'flat' : 'tonal'"
      :color="editor.isActive('heading', { level: 2 }) ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleHeading({ level: 2 }).run())"
    >
      H2
    </v-btn>
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('bulletList') ? 'flat' : 'tonal'"
      :color="editor.isActive('bulletList') ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleBulletList().run())"
    >
      <i class="bx bx-list-ul" />
    </v-btn>
    <v-btn
      size="x-small"
      class="menubar-btn"
      :variant="editor.isActive('orderedList') ? 'flat' : 'tonal'"
      :color="editor.isActive('orderedList') ? 'primary' : undefined"
      @click="run(() => editor.chain().focus().toggleOrderedList().run())"
    >
      <i class="bx bx-list-ol" />
    </v-btn>
    <v-btn size="x-small" class="menubar-btn" variant="tonal" @click="run(() => editor.chain().focus().setHorizontalRule().run())">
      hr
    </v-btn>
    <v-btn size="x-small" class="menubar-btn" variant="tonal" @click="run(() => editor.chain().focus().undo().run())">
      <i class="bx bx-undo" />
    </v-btn>
    <v-btn size="x-small" class="menubar-btn" variant="tonal" @click="run(() => editor.chain().focus().redo().run())">
      <i class="bx bx-redo" />
    </v-btn>
  </div>
</template>

<script setup lang="ts">
import type { Editor } from "@tiptap/core";

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
}

.menubar-btn {
  min-width: 2rem;
  background-color: var(--applet-fg, var(--fg)) !important;
  color: var(--applet-bg, var(--bg)) !important;
  border: 1px solid var(--applet-border, var(--bg)) !important;
}

.menubar-btn.v-btn--variant-flat {
  color: #fff !important;
}
</style>
