<template>
  <grid-item
    :static="static"
    :x="positonx"
    :y="pospositony"
    :w="width"
    :h="height"
    :i="i"
  >
    <div class="grid-item-container">
    <vs-card>
    </div>

  </grid-item>
</template>
<script>
import { GridLayout, GridItem } from "vue-grid-layout"
export default {
    components: {
        GridLayout,
        GridItem,
        Editor
    },
    data() {
        return {
            index: 0
            draggable: false,
            resizable: true,
            positionx: 0,
            positiony: 0,
            width: 8,
            height: 8,

        }
    },
    methods: {
        showTitle() {
            let result = i;
            return result;
        },

    }
}