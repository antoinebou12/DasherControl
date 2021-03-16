<template>
    <div class="editor-container">
        <div v-bind:id="holderId"></div>
    </div>
</template>

<script>
import EditorJS from '@editorjs/editorjs';

let id = 0;

export default {
  name: 'Editor',
  props: {
      name: String 
  },
  data() {
    return {
        id: null,
        holderId: "editor-"+ this.id,
        editor: new EditorJS({
            holder: `editor-${this.id}`
        }),
    }
  },
  beforeCreate() {
    this.id = id.toString();
    id += 1;
  },
}
</script>

<style>
.editor-container {
    box-shadow: rgb(0 0 0 / 10%) 0px 10px 15px -3px, rgb(0 0 0 / 5%) 0px 4px 6px -2px;
    border: 1px solid black;
    border-radius: .5rem;
}
</style>
