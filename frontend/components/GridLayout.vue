<template>
    <grid-layout 
                :layout.sync="layout"
                 :col-num="12"
                 :row-height="24"
                 :is-draggable="draggable"
                 :is-resizable="resizable"
                 :vertical-compact="true"
                 :use-css-transforms="true">
        <grid-item 
                   v-for="item in layout"
                   v-bind:key="item.i"
                   :static="item.static"
                   :x="item.x"
                   :y="item.y"
                   :w="item.w"
                   :h="item.h"
                   :i="item.i">
            <div class="grid-item-container">
                <span class="text">{{itemTitle(item)}}</span>
                <div class="grid-item-main">
                    <Editor class="editor" />
                </div>
            </div>
        </grid-item>
    </grid-layout>
</template>

<script>
import { GridLayout, GridItem } from "vue-grid-layout"
import Editor from './applets/Editor.vue'
export default {
    components: {
        GridLayout,
        GridItem,
        Editor
    },
    data() {
        return {
            layout: [
                {"x":0,"y":0,"w":6,"h":12,"i":"0", static: false},
                {"x":6,"y":0,"w":6,"h":12,"i":"1", static: false},
            ],
            draggable: true,
            resizable: true,
            colNum: 12,
            rowHeight: 24,
            index: 0
        }
    },
    methods: {
        itemTitle(item) {
            let result = item.i;
            if (item.static) {
                result += " - Static";
            }
            return result;
        }
    }
}
</script>

<style scoped>
.vue-grid-layout {
    background: #FFF;
}
.vue-grid-item:not(.vue-grid-placeholder) {
    box-shadow: rgb(0 0 0 / 10%) 0px 10px 15px -3px, rgb(0 0 0 / 5%) 0px 4px 6px -2px;
    border: 1px solid black;
    border-radius: .5rem;
}
.vue-grid-item .resizing {
    opacity: 0.9;
}
.vue-grid-item .static {
    background: #cce;
}
.vue-grid-item .text {
    font-size: 24px;
    text-align: center;
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    margin: auto;
    height: 100%;
    width: 100%;
}
.vue-grid-item .no-drag {
    height: 100%;
    width: 100%;
}
.vue-grid-item .minMax {
    font-size: 12px;
}
.vue-grid-item .add {
    cursor: pointer;
}
.vue-draggable-handle {
    position: absolute;
    width: 20px;
    height: 20px;
    top: 0;
    left: 0;
    background: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='10'><circle cx='5' cy='5' r='5' fill='#999999'/></svg>") no-repeat;
    background-position: bottom right;
    padding: 0 8px 8px 0;
    background-repeat: no-repeat;
    background-origin: content-box;
    box-sizing: border-box;
    cursor: pointer;
}
.grid-item-container{
    width: 100%;
    height: 100%;
}

.editor{
    background: rgba(0,128,255,0.1);
    height: 200px;
    margin: 25px;
}
</style>