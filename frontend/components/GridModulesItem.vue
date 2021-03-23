<template>
  <grid-item
    class="gridItem"
    :ref="gridItem"
    :static="static"
    :x="positonx"
    :y="pospositony"
    :w="width"
    :h="height"
    :i="i"
    @resize="resizeEvent"
    @resized="resizedEvent"
    @container-resized="containerResizedEvent">
    <div 
      class="grid-item-main"
      :ref="gridItemContainer">
      <div class="EditItemMenu"

      </div>
      
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
    props: {
      applets: { type: Object, required: true }
    }
    beforeMount() {
        this.$options.components.Node = this.treeNode
    },
    watch: {
        appletsInstance: function(value){
          
        }
    }
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
            return result;
        },
        resizeEvent: function(i, newH, newW, newHPx, newWPx){
          this.$ref.gridItem.w = newW
          this.$ref.gridItem.h = newH
          this.$ref.gridItemContainer.style.setProperty("--gridItemContainerheight", fontSize);
        },
        resizedEvent: function(i, newX, newY, newHPx, newWPx){
            const msg = "RESIZED i=" + i + ", X=" + newX + ", Y=" + newY + ", H(px)=" + newHPx + ", W(px)=" + newWPx;
            this.eventLog.push(msg);
            console.log(msg);
        },
        containerResizedEvent: function(i, newH, newW, newHPx, newWPx){
            const msg = "CONTAINER RESIZED i=" + i + ", H=" + newH + ", W=" + newW + ", H(px)=" + newHPx + ", W(px)=" + newWPx;
            this.eventLog.push(msg);
            console.log(msg);
        },

    }
}
<style lang="scss">
.gridItem{
  --grid_item_main_width: 
}

</style>