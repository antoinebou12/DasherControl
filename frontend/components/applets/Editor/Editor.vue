<template>
  <div ref="editorContainer" class="editor-container">
    <editor-menubar v-if="editor" :editor="editor" :run="run" />
    <editor-content class="editor-content" :editor="editor" />
  </div>
</template>

<script setup lang="ts">
import type { Editor } from "@tiptap/core";
import { EditorContent, useEditor } from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import EditorMenubar from "./EditorMenubar.vue";
import { getCurrentInstance, onBeforeUnmount, ref, watch } from "vue";

const props = defineProps<{
  name?: string;
  textContent?: string;
}>();

const editorContainer = ref<HTMLElement | null>(null);
const instance = getCurrentInstance();

function saveToParents(ed?: Editor | null) {
  const e = ed ?? editor.value;
  if (!e) return;
  const htmlText = e.getHTML();
  const proxy = instance?.proxy as any;
  const p = proxy?.$parent;
  if (p?.$attrs) p.$attrs.textContent = htmlText;
  const grid = p?.$parent?.$parent?.$parent;
  if (grid?.currentAppletData) {
    grid.currentAppletData.textContent = htmlText;
  }
}

function resizeAfterBlur(ed: Editor) {
  const proxy = instance?.proxy as any;
  const contentEl = ed.view.dom as HTMLElement;
  if (!editorContainer.value || !contentEl) return;
  editorContainer.value.style.height = `${100 + contentEl.offsetHeight}px`;
  const gridItemMain = proxy?.$parent?.$parent?.$parent?.$refs?.gridItemMain as HTMLElement | undefined;
  const draggableHandle = proxy?.$parent?.$parent?.$parent?.$refs?.draggableHandle as HTMLElement | undefined;
  if (gridItemMain && contentEl.offsetHeight > gridItemMain.offsetHeight && draggableHandle) {
    draggableHandle.style.height = "auto";
    setTimeout(() => {
      draggableHandle.style.height = "24px";
    }, 1);
  }
}

const editor = useEditor({
  extensions: [StarterKit],
  content: props.textContent || "",
  onUpdate: () => saveToParents(),
  editorProps: {
    handleDOMEvents: {
      blur: () => {
        const ed = editor.value;
        if (ed) {
          resizeAfterBlur(ed);
          saveToParents(ed);
        }
        return false;
      },
    },
  },
});

function run(fn: () => void) {
  fn();
  saveToParents();
}

watch(
  () => props.textContent,
  (v) => {
    const ed = editor.value;
    if (!ed) return;
    const next = v || "";
    if (next !== ed.getHTML()) {
      ed.commands.setContent(next, false);
    }
  },
);

onBeforeUnmount(() => {
  editor.value?.destroy();
});
</script>

<style lang="scss" scoped>
.editor-container {
  box-shadow: rgb(0 0 0 / 10%) 0px 10px 15px -3px,
    rgb(0 0 0 / 5%) 0px 4px 6px -2px;
  border: 1px solid black;

  .editor-content {
    overflow-wrap: break-word;
    word-wrap: break-word;
    word-break: break-word;

    :deep(*) {
      caret-color: currentColor;
    }

    :deep(pre) {
      padding: 0.7rem 1rem;
      border-radius: 5px;
      background: var(--bg);
      color: var(--fg);
      font-size: 0.8rem;
      overflow-x: auto;

      code {
        display: block;
      }
    }

    :deep(p code) {
      padding: 0.2rem 0.4rem;
      border-radius: 5px;
      font-size: 0.8rem;
      font-weight: bold;
      background: rgba(var(--bg-rgb), 0.1);
      color: rgba(var(--fg-rgb), 0.8);
    }

    :deep(ul),
    :deep(ol) {
      padding-left: 1rem;
    }

    :deep(li > p),
    :deep(li > ol),
    :deep(li > ul) {
      margin: 0;
    }

    :deep(a) {
      color: inherit;
    }

    :deep(blockquote) {
      border-left: 3px solid rgba(var(--bg-rgb), 0.1);
      color: rgba(var(--bg-rgb), 0.8);
      padding-left: 0.8rem;
      font-style: italic;

      p {
        margin: 0;
      }
    }

    :deep(img) {
      max-width: 100%;
      border-radius: 3px;
    }
  }
}
</style>
