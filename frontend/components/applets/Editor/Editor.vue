<template>
  <div class="editor-container">
    <editor-menu-bar :editor="editor" v-slot="{ commands, isActive }">
      <div class="menubar">
        <vs-button
          class="menubar__button"
          :active="isActive.bold()"
          @click="commands.bold">
          B
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.italic()"
          @click="commands.italic">
          /
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.strike()"
          @click="commands.strike">
          -
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.underline()"
          @click="commands.underline">
          _
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.code()"
          @click="commands.code">
          pre
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.paragraph()"
          @click="commands.paragraph">
          p
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.heading({ level: 1 })"
          @click="commands.heading({ level: 1 })"
        >
          H1
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.heading({ level: 2 })"
          @click="commands.heading({ level: 2 })"
        >
          H2
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.heading({ level: 3 })"
          @click="commands.heading({ level: 3 })">
          H3
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.bullet_list()"
          @click="commands.bullet_list">
          ul
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.ordered_list()"
          @click="commands.ordered_list">
          ol
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.blockquote()"
          @click="commands.blockquote"
        >
          >
        </vs-button>

        <vs-button
          class="menubar__button"
          :active="isActive.code_block()"
          @click="commands.code_block"
        >
          code
        </vs-button>

        <vs-button class="menubar__button" @click="commands.horizontal_rule">
          hr
        </vs-button>

        <vs-button class="menubar__button" @click="commands.undo">
          undo
        </vs-button>

        <vs-button class="menubar__button" @click="commands.redo">
          redo
        </vs-button>

        <vs-button class="menubar__button" @click="save">
        save
        </vs-button>
        <vs-button @click="save()">
          <i class="bx bx-save"/>
        </vs-button>
      </div>
    </editor-menu-bar>

    <editor-content class="editor-content" :editor="editor" />
  </div>
</template>

<script>
import { Editor, EditorContent, EditorMenuBar } from "tiptap";
import {
  Blockquote,
  CodeBlock,
  HardBreak,
  Heading,
  HorizontalRule,
  OrderedList,
  BulletList,
  ListItem,
  TodoItem,
  TodoList,
  Bold,
  Code,
  Italic,
  Link,
  Strike,
  Underline,
  History,
  Image
} from "tiptap-extensions";

export default {
  name: "Editor",
  components: {
    EditorContent,
    EditorMenuBar
  },
  props: {
    name: String,
    textContent: String
  },
  data() {
    return {
      editor: new Editor({
        extensions: [
          new Blockquote(),
          new BulletList(),
          new CodeBlock(),
          new HardBreak(),
          new Heading({levels: [1, 2, 3]}),
          new HorizontalRule(),
          new ListItem(),
          new OrderedList(),
          new TodoItem(),
          new TodoList(),
          new Link(),
          new Bold(),
          new Code(),
          new Italic(),
          new Strike(),
          new Underline(),
          new History(),
          new Image()
        ],
      }),
    }
  },
  created() {
    this.editor.setContent(this.textContent)
  },
  beforeDestroy() {
    this.editor.destroy();
  },
  methods: {
    save(){
      let htmlText = this.editor.getHTML()
      // GridItem
      this.$parent.$attrs["textContent"] = htmlText;
      // GridItemApplet
      this.$parent.$parent.$parent.currentAppletData["textContent"] = htmlText;
    },
  }
};
</script>

<style lang="scss" scoped>
.menubar {
  display: flex;
}
.editor-container {
  box-shadow: rgb(0 0 0 / 10%) 0px 10px 15px -3px,
    rgb(0 0 0 / 5%) 0px 4px 6px -2px;
  border: 1px solid black;

  button {
    border-radius: 0px;
  }

  .editor-content {
    overflow-wrap: break-word;
    word-wrap: break-word;
    word-break: break-word;
    height:100%;

    * {
      caret-color: currentColor;
    }

    pre {
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

    p code {
      padding: 0.2rem 0.4rem;
      border-radius: 5px;
      font-size: 0.8rem;
      font-weight: bold;
      background: rgba(var(--bg-rgb), 0.1);
      color: rgba(var(--fg-rgb), 0.8);
    }

    ul,
    ol {
      padding-left: 1rem;
    }

    li > p,
    li > ol,
    li > ul {
      margin: 0;
    }

    a {
      color: inherit;
    }

    blockquote {
      border-left: 3px solid rgba(var(--bg-rgb), 0.1);
      color: rgba(var(--bg-rgb), 0.8);
      padding-left: 0.8rem;
      font-style: italic;

      p {
        margin: 0;
      }
    }

    img {
      max-width: 100%;
      border-radius: 3px;
    }

    table {
      border-collapse: collapse;
      table-layout: fixed;
      width: 100%;
      margin: 0;
      overflow: hidden;

      td,
      th {
        min-width: 1em;
        border: 2px solid var(--fg-rgb);
        padding: 3px 5px;
        vertical-align: top;
        box-sizing: border-box;
        position: relative;
        > * {
          margin-bottom: 0;
        }
      }

      th {
        font-weight: bold;
        text-align: left;
      }

      .selectedCell:after {
        z-index: 2;
        position: absolute;
        content: "";
        left: 0;
        right: 0;
        top: 0;
        bottom: 0;
        background: rgba(200, 200, 255, 0.4);
        pointer-events: none;
      }

      .column-resize-handle {
        position: absolute;
        right: -2px;
        top: 0;
        bottom: 0;
        width: 4px;
        z-index: 20;
        background-color: #adf;
        pointer-events: none;
      }
    }

    .tableWrapper {
      margin: 1em 0;
      overflow-x: auto;
    }

    .resize-cursor {
      cursor: ew-resize;
      cursor: col-resize;
    }
  }
}
</style>
